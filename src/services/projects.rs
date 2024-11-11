use std::path::PathBuf;

use axum::extract::Multipart;
use loco_rs::prelude::*;

use crate::{
    models::{
        _entities::{
            data, project_thumbnails, projects::{self, ActiveModel, Entity, Model}
        },
        projects::{get_category_from_string, get_string_from_category, CreateProject, ProjectDto, UpdateProject}, users,
    }, services::data::add_data_file_from_path, shared::{get_file_name_with_extension::get_file_name_with_extension_from_field, get_technologies_from_string::get_technologies_from_string}
};

use super::data::delete_data_by_id;

pub async fn get_all_projects(ctx: &AppContext) -> ModelResult<Vec<Model>> {
    let projects = Entity::find().all(&ctx.db).await?;
    Ok(projects)
}

pub async fn get_project_by_id(ctx: &AppContext, id: i32) -> ModelResult<Model> {
    let project = Entity::find_by_id(id).one(&ctx.db).await?;
    let project = project.ok_or_else(|| ModelError::EntityNotFound)?;
    Ok(project)
}

pub async fn get_project_by_name(ctx: &AppContext, name: &str) -> ModelResult<Model> {
    let project = Entity::find()
        .filter(projects::Column::Name.contains(name))
        .one(&ctx.db)
        .await?;
    let project = project.ok_or_else(|| ModelError::EntityNotFound)?;
    Ok(project)
}

pub async fn get_archived_projects(ctx: &AppContext) -> ModelResult<Vec<Model>> {
    let archived_projects = Entity::find()
        .filter(
            model::query::condition()
                .eq(projects::Column::IsArchived, true)
                .build(),
        )
        .all(&ctx.db)
        .await?;
    Ok(archived_projects)
}

pub async fn get_highlighted_projects(ctx: &AppContext) -> ModelResult<Vec<Model>> {
    let highlighted_projects = Entity::find()
        .filter(
            model::query::condition()
                .eq(projects::Column::IsHighlighted, true)
                .build(),
        )
        .all(&ctx.db)
        .await?;
    Ok(highlighted_projects)
}

pub async fn get_all_projects_dto(ctx: &AppContext) -> Result<Vec<ProjectDto>> {
    let projects_with_thumbnails = Entity::find()
        .find_with_related(project_thumbnails::Entity)
        .all(&ctx.db)
        .await?;

    let thumbnails_ids = projects_with_thumbnails
        .iter()
        .map(|(_, thumbnails)| {
            thumbnails
                .iter()
                .map(|thumbnail| thumbnail.data_id)
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect::<Vec<i32>>();

    let thumbnails_data  = data::Entity::find()
        .filter(model::query::condition().is_in(data::Column::Id, thumbnails_ids).build())
        .all(&ctx.db)
        .await?;

    let thumbnails_map = thumbnails_data
        .into_iter()
        .map(|thumbnail| (thumbnail.id, thumbnail))
        .collect::<std::collections::HashMap<i32, data::Model>>();

    let projects_dto = projects_with_thumbnails
        .into_iter()
        .map(|(project, thumbnails)| {
            let thumbnails = thumbnails
                .into_iter()
                .map(|thumbnail| {
                    let thumbnail_data = thumbnails_map.get(&thumbnail.data_id).unwrap();
                    let url = format!("/api/data/{}", thumbnail_data.file_name);
                    url
                })
                .collect();

            ProjectDto {
                id: project.id,
                name: project.name,
                short_description: project.short_description,
                description: project.description,
                category: get_category_from_string(&project.category),
                github_url: project.github_url,
                download_url: project.download_url,
                visit_url: project.visit_url,
                technologies: get_technologies_from_string(&project.technology),
                thumbnails,
            }
        })
        .collect();

    Ok(projects_dto)
}

pub async fn get_project_dto(ctx: &AppContext, id: i32) -> Result<ProjectDto> {
    let project = get_project_by_id(ctx, id).await?;
    let thumbnails = project
        .find_related(project_thumbnails::Entity)
        .all(&ctx.db)
        .await?;

    let thumbnails = thumbnails
        .into_iter()
        .map(|thumbnail| thumbnail.data_id.to_string())
        .collect();
    let project_dto = ProjectDto {
        id: project.id,
        name: project.name,
        short_description: project.short_description,
        description: project.description,
        category: get_category_from_string(&project.category),
        github_url: project.github_url,
        download_url: project.download_url,
        visit_url: project.visit_url,
        technologies: get_technologies_from_string(&project.technology),
        thumbnails,
    };

    Ok(project_dto)
}

pub async fn get_project_dto_by_name(ctx: &AppContext, name: &str) -> Result<ProjectDto> {
    let project = get_project_by_name(ctx, name).await?;
    let thumbnails = project
        .find_related(project_thumbnails::Entity)
        .all(&ctx.db)
        .await?;

    let thumbnails = thumbnails
        .into_iter()
        .map(|thumbnail| thumbnail.data_id.to_string())
        .collect();
    let project_dto = ProjectDto {
        id: project.id,
        name: project.name,
        short_description: project.short_description,
        description: project.description,
        category: get_category_from_string(&project.category),
        github_url: project.github_url,
        download_url: project.download_url,
        visit_url: project.visit_url,
        technologies: get_technologies_from_string(&project.technology),
        thumbnails,
    };

    Ok(project_dto)
}

pub async fn add_project(
    ctx: &AppContext,
    data: CreateProject,
) -> ModelResult<Model> {
    let mut item = ActiveModel {
        ..Default::default()
    };

    item.name = Set(data.name);
    item.short_description = Set(data.short_description);
    item.description = Set(data.description);
    item.category = Set(get_string_from_category(&data.category));
    item.github_url = Set(data.github_url);
    item.download_url = Set(data.download_url);
    item.visit_url = Set(data.visit_url);
    item.technology = Set(data.technologies.join(","));

    let item = item.insert(&ctx.db).await?;

    Ok(item)
}

pub async fn add_project_with_thumbnails(
    ctx: &AppContext,
    thumbnails_paths: Vec<String>,
    data: CreateProject,
) -> Result<()> {
    let txn = ctx.db.begin().await?;

    let project = add_project(ctx, data).await?;
    let project_id = project.id;
    for thumbnail_path in thumbnails_paths {
        let thumbnail_data = add_data_file_from_path(ctx, &thumbnail_path, "thumbnail.png", false, true).await?;

        let thumbnail = project_thumbnails::ActiveModel {
            project_id: Set(project_id),
            data_id: Set(thumbnail_data.id),
            ..Default::default()
        };

        thumbnail.insert(&txn).await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn add_project_with_thumbnails_multipart(
    auth: &auth::JWT,
    ctx: &AppContext,
    mut payload: Multipart,
) -> Result<()> {
    let _current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let mut project_name = None;
    let mut short_description = None;
    let mut description = None;
    let mut category = None;
    let mut github_url = None;
    let mut download_url = None;
    let mut visit_url = None;
    let mut technologies = None;

    let mut thumbnails_file_names = Vec::new();
    let mut thumbnails = Vec::new();

    while let Some(field) = payload
    .next_field()
    .await
    .map_err(|_| ModelError::Any("Failed to get next field".into()))?
    {
        let name = field
        .name()
        .ok_or_else(|| ModelError::Any("Failed to get field name".into()))?;

        match name {
            "name" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                project_name = Some(value);
            }
            "short_description" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                short_description = Some(value);
            }
            "description" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                description = Some(value);
            }
            "category" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                category = Some(value);
            }
            "github_url" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                github_url = Some(value);
            }
            "download_url" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                download_url = Some(value);
            }
            "visit_url" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                visit_url = Some(value);
            }
            "technologies" => {
                let value = field
                    .text()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get text".into()))?;
                technologies = Some(value);
            }
            "thumbnail" => {
               let (_, ext) = get_file_name_with_extension_from_field(&field, "txt").map_err(|_| ModelError::Any("Failed to get file name".into()))?;

                let file_name = uuid::Uuid::new_v4().to_string();
                let file_name = format!("{}.{}", file_name, ext);

                let data_content = field
                    .bytes()
                    .await
                    .map_err(|_| ModelError::Any("Failed to get bytes".into()))?;

                thumbnails_file_names.push(file_name);
                thumbnails.push(data_content);
            },
            _ => {}
        }
    }

    let category = category.map(|s| get_category_from_string(&s));
    
    let project = CreateProject {
        name: project_name.ok_or_else(|| ModelError::Any("Name field is required".into()))?,
        short_description: short_description.ok_or_else(|| ModelError::Any("Short description field is required".into()))?,
        description: description,
        category: category.ok_or_else(|| ModelError::Any("Category field is required".into()))?,
        github_url: github_url,
        download_url: download_url,
        visit_url: visit_url,
        technologies: technologies.ok_or_else(|| ModelError::Any("Technologies field is required".into()))?.split(",").map(|s| s.to_string()).collect(),
    };

    let txn = ctx.db.begin().await?;

    let item = ActiveModel {
        name: Set(project.name),
        short_description: Set(project.short_description),
        description: Set(project.description),
        category: Set(get_string_from_category(&project.category)),
        github_url: Set(project.github_url),
        download_url: Set(project.download_url),
        visit_url: Set(project.visit_url),
        technology: Set(project.technologies.join(",")),
        ..Default::default()
    };

    let item = item.insert(&txn).await?;

    let project_id = item.id;

    for (thumbnail_file_name, thumbnail) in thumbnails_file_names.iter().zip(thumbnails.iter()) {
        let thumbnail_data = data::ActiveModel {
            file_name: Set(thumbnail_file_name.clone()),
            file_url: Set(format!("uploads/{}", thumbnail_file_name)),
            protected: Set(false),
            ..Default::default()
        };

        let thumbnail_data = thumbnail_data.insert(&txn).await?;
        let path = PathBuf::from(thumbnail_file_name);
        match ctx
        .storage
        .as_ref()
        .upload(
            path.as_path(),
            thumbnail,
        )
        .await {
            Ok(_) => {
                let thumbnail = project_thumbnails::ActiveModel {
                    project_id: Set(project_id),
                    data_id: Set(thumbnail_data.id),
                    ..Default::default()
                };

                thumbnail.insert(&txn).await?;
            },
            Err(_) => return Err(Error::Any("Failed to save file to storage".into())),
        }
    }

    txn.commit().await?;
    
    Ok(())
}

pub async fn update_project(
    ctx: &AppContext,
    id: i32,
    data: UpdateProject,
) -> ModelResult<Model> {
    let item = get_project_by_id(ctx, id).await?;
    let mut item = item.into_active_model();

    if let Some(name) = data.name {
        item.name = Set(name);
    }
    if let Some(short_description) = data.short_description {
        item.short_description = Set(short_description);
    }
    item.description = Set(data.description);
    
    if let Some(category) = data.category {
        item.category = Set(get_string_from_category(&category));
    }

    item.github_url = Set(data.github_url);
    item.download_url = Set(data.download_url);
    item.visit_url = Set(data.visit_url);
    if let Some(technologies) = data.technologies {
        item.technology = Set(technologies.join(","));
    }

    let item = item.update(&ctx.db).await?;
    Ok(item)
}

pub async fn update_thumbnails_for_project(
    ctx: &AppContext,
    id: i32,
    thumbnails_paths: Vec<String>,
) -> Result<()> {
    let txn = ctx.db.begin().await?;

    let project = get_project_by_id(ctx, id).await?;
    let project_id = project.id;
    let thumbnails = project
        .find_related(project_thumbnails::Entity)
        .all(&ctx.db)
        .await?;

    for thumbnail in thumbnails {
        let _ = delete_data_by_id(thumbnail.data_id, ctx);
        let _ = thumbnail.delete(&txn).await?;
    }

    for thumbnail_path in thumbnails_paths {
        let thumbnail_data = add_data_file_from_path(ctx, &thumbnail_path, "thumbnail.png", false, true).await?;

        let thumbnail = project_thumbnails::ActiveModel {
            project_id: Set(project_id),
            data_id: Set(thumbnail_data.id),
            ..Default::default()
        };

        thumbnail.insert(&txn).await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete_project(ctx: &AppContext, id: i32) -> Result<()> {
    let item = get_project_by_id(ctx, id).await?;
    let thumbnails = item.find_related(project_thumbnails::Entity).all(&ctx.db).await?;
    let thumbnails_data_ids = thumbnails.into_iter().map(|thumbnail| thumbnail.data_id).collect::<Vec<i32>>();
    for data_id in thumbnails_data_ids {
        let _ = delete_data_by_id(data_id, ctx);
    }

    let _ = item.delete(&ctx.db).await?;
    
    Ok(())
}

pub async fn delete_thumbnails_for_project(ctx: &AppContext, id: i32) -> Result<()> {
    let project = get_project_by_id(ctx, id).await?;
    let thumbnails = project
        .find_related(project_thumbnails::Entity)
        .all(&ctx.db)
        .await?;

    for thumbnail in thumbnails {
        let _ = delete_data_by_id(thumbnail.data_id, ctx);
        let _ = thumbnail.delete(&ctx.db).await?;
    }

    Ok(())
}