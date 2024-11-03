use std::path::PathBuf;

use crate::models::_entities::data::{self, ActiveModel, Entity, Model};
use crate::models::users::users;
use axum::extract::Multipart;
use axum_extra::headers::Range;
use axum_extra::TypedHeader;
use loco_rs::prelude::*;
use tokio::fs::File;

use axum_range::KnownSize;
use axum_range::Ranged;

pub async fn get_data_by_file_name(file_name: &str, ctx: &AppContext) -> ModelResult<Model> {
    let data = Entity::find()
        .filter(data::Column::FileName.eq(file_name))
        .one(&ctx.db)
        .await?;

    data.ok_or_else(|| ModelError::EntityNotFound)
}

pub async fn serve_data_file(
    auth: &Option<auth::JWT>,
    range: Option<TypedHeader<Range>>,
    file_name: &str,
    ctx: &AppContext,
) -> Result<Ranged<KnownSize<File>>> {
    let data = match get_data_by_file_name(&file_name, &ctx).await {
        Ok(data) => data,
        Err(_) => return not_found(),
    };

    if data.protected {
        match auth {
            None => return unauthorized("Unauthorized"),
            Some(auth) => match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
                Ok(_) => {}
                Err(_) => return unauthorized("Unauthorized"),
            },
        }
    }

    match File::open(&data.file_url).await {
        Ok(file) => {
            let body = KnownSize::file(file).await?;
            let range = range.map(|TypedHeader(range)| range);
            Ok(Ranged::new(range, body))
        }
        Err(_) => return not_found(),
    }
}

pub async fn add(auth: &auth::JWT, ctx: &AppContext, mut payload: Multipart) -> ModelResult<Model> {
    let _current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let mut protected = None;
    let mut file_name = None;
    let mut content = None;
    let mut file_path = None;

    while let Some(field) = payload
        .next_field()
        .await
        .map_err(|_| ModelError::Any("Failed to get next field".into()))?
    {
        let name = field
            .name()
            .ok_or_else(|| ModelError::Any("Failed to get field name".into()))?;
        match name {
            "protected" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?
                    .parse::<bool>()
                    .map_err(|_| ModelError::Any("Failed to parse bool".into()))?;
                protected = Some(value);
            }
            "file" => {
                let og_file_name = field
                    .file_name()
                    .ok_or_else(|| ModelError::Any("Failed to get file name".into()))?;
                let ext = String::from(og_file_name.split('.').last().unwrap_or("txt"));

                let temp_file_name = uuid::Uuid::new_v4().to_string();
                let temp_file_name = format!("{}.{}", temp_file_name, ext);

                file_name = Some(temp_file_name.clone());

                let path = PathBuf::from(temp_file_name);
                file_path = Some(path.clone());

                let data_content = field
                    .bytes()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get bytes".into()))?;

                content = Some(data_content.clone());
            }
            _ => {}
        }
    }

    let protected =
        protected.ok_or_else(|| ModelError::Any("Protected field is required".into()))?;

    let file_name = file_name.ok_or_else(|| ModelError::Any("File field is required".into()))?;

    let mut item = ActiveModel {
        ..Default::default()
    };

    item.protected = Set(protected);
    item.file_name = Set(file_name.clone());
    item.file_url = Set(format!("uploads/{}", file_name));

    let item = item.insert(&ctx.db).await?;

    let file_path = file_path.ok_or_else(|| ModelError::Any("File path is required".into()))?;
    let content = content.ok_or_else(|| ModelError::Any("Content is required".into()))?;

    match ctx
        .storage
        .as_ref()
        .upload(file_path.as_path(), &content)
        .await
    {
        Ok(_) => {}
        Err(_) => return Err(ModelError::Any("Failed to save file to storage".into())),
    }
    Ok(item)
}
