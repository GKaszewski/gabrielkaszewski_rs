use std::path::PathBuf;

use crate::models::_entities::data::{self, ActiveModel, Entity, Model};
use crate::models::users::users;
use crate::shared::get_file_name_with_extension::get_file_name_with_extension_from_field;
use axum::extract::Multipart;
use axum_extra::headers::Range;
use axum_extra::TypedHeader;

use bytes::Bytes;
use loco_rs::prelude::*;
use tokio::fs::File;

use std::fs::{self};

use axum_range::KnownSize;
use axum_range::Ranged;

pub async fn get_all_data(ctx: &AppContext) -> ModelResult<Vec<Model>> {
    let data = Entity::find().all(&ctx.db).await?;
    Ok(data)
}

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

pub async fn add_data_file_from_path(
    ctx: &AppContext,
    file_path: &str,
    file_name: &str,
    protected: bool,
    uuid_name: bool,
) -> ModelResult<Model> {
    let ext = String::from(file_name.split('.').last().unwrap_or("txt"));
    let file_name = if uuid_name {
        let temp_file_name = uuid::Uuid::new_v4().to_string();
        format!("{}.{}", temp_file_name, ext)
    } else {
        file_name.to_string()
    };

    let path = PathBuf::from(file_path);
    let content = fs::read(&path).map_err(|_| ModelError::Any("Failed to read file".into()))?;
    let content = Bytes::from(content);

    let mut item = ActiveModel {
        ..Default::default()
    };

    item.protected = Set(protected);
    item.file_name = Set(file_name.to_string());
    item.file_url = Set(format!("uploads/{}", file_name));

    let item = item.insert(&ctx.db).await?;

    match ctx.storage.as_ref().upload(&path.as_path(), &content).await {
        Ok(_) => {}
        Err(_) => return Err(ModelError::Any("Failed to save file to storage".into())),
    }

    Ok(item)
}

pub async fn add(
    auth: &auth::JWT,
    ctx: &AppContext,
    mut payload: Multipart,
    uuid_name: bool,
) -> ModelResult<Model> {
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
                let (og_file_name, ext) = get_file_name_with_extension_from_field(&field, "txt").map_err(|_| ModelError::Any("Failed to get file name".into()))?;

                let temp_file_name = if uuid_name {
                    let temp_file_name = uuid::Uuid::new_v4().to_string();
                    format!("{}.{}", temp_file_name, ext)
                } else {
                    og_file_name.to_string()
                };

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

pub async fn clear_all_data(ctx: &AppContext) -> ModelResult<()> {
    let data = get_all_data(&ctx).await?;
    for item in data {
        let path = PathBuf::from(&item.file_url);
        tracing::info!("Deleting file: {:?}", path);
        match ctx.storage.as_ref().delete(&path).await {
            Ok(_) => {}
            Err(_) => return Err(ModelError::Any("Failed to delete file from storage".into())),
        }

        item.delete(&ctx.db).await?;
    }
    Ok(())
}

pub async fn delete_data_by_file_name(file_name: &str, ctx: &AppContext) -> ModelResult<()> {
    let data = get_data_by_file_name(&file_name, &ctx).await?;

    let path = PathBuf::from(&data.file_url);
    match ctx.storage.as_ref().delete(&path).await {
        Ok(_) => {}
        Err(_) => return Err(ModelError::Any("Failed to delete file from storage".into())),
    }

    data.delete(&ctx.db).await?;
    Ok(())
}

pub async fn delete_multiple_data_by_file_names(
    file_names: Vec<String>,
    ctx: &AppContext,
) -> ModelResult<()> {
    for file_name in file_names {
        delete_data_by_file_name(&file_name, &ctx).await?;
    }
    Ok(())
}

pub async fn delete_data_by_id(id: i32, ctx: &AppContext) -> ModelResult<()> {
    let data = Entity::find().filter(data::Column::Id.eq(id)).one(&ctx.db).await?;

    match data {
        Some(data) => {
            let path = PathBuf::from(&data.file_url);
            match ctx.storage.as_ref().delete(&path).await {
                Ok(_) => {}
                Err(_) => return Err(ModelError::Any("Failed to delete file from storage".into())),
            }

            data.delete(&ctx.db).await?;
        }
        None => return Err(ModelError::EntityNotFound),
    }
    
    Ok(())
}