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
    auth: &auth::JWT,
    range: Option<TypedHeader<Range>>,
    file_name: &str,
    ctx: &AppContext,
) -> Result<Ranged<KnownSize<File>>> {
    let data = get_data_by_file_name(&file_name, &ctx).await?;
    if data.protected {
        match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
            Ok(_) => {}
            Err(_) => return unauthorized("Unauthorized"),
        }
    }

    let file = File::open(&data.file_url).await?;
    let body = KnownSize::file(file).await?;
    let range = range.map(|TypedHeader(range)| range);
    Ok(Ranged::new(range, body))
}

pub async fn add(auth: &auth::JWT, ctx: &AppContext, mut payload: Multipart) -> ModelResult<Model> {
    let _current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let mut protected = None;
    let mut file_name = None;

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
                file_name = match field.file_name() {
                    Some(file_name) => Some(String::from(file_name)),
                    None => return Err(ModelError::Any("Failed to get file name".into())),
                };

                if file_name.is_none() {
                    return Err(ModelError::Any("Failed to get file name".into()));
                }

                let path = PathBuf::from("uploads").join(file_name.as_ref().unwrap());

                let content = field
                    .bytes()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get bytes".into()))?;
                
                match ctx.storage.as_ref().upload(path.as_path(), &content).await {
                    Ok(_) => {}
                    Err(_) => return Err(ModelError::Any("Failed to save file to storage".into())),
                }
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
    item.file_name = Set(file_name);

    let item = item.insert(&ctx.db).await?;
    Ok(item)
}
