use loco_rs::prelude::*;

use crate::{
    models::{
        _entities::{
            project_thumbnails,
            projects::{self, ActiveModel, Entity, Model},
        },
        projects::{get_category_from_string, get_string_from_category, CreateProject, ProjectDto, UpdateProject},
    }, services::data::add_data_file_from_path, shared::get_technologies_from_string::get_technologies_from_string
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

    let projects_dto = projects_with_thumbnails
        .into_iter()
        .map(|(project, thumbnails)| {
            let thumbnails = thumbnails
                .into_iter()
                .map(|thumbnail| thumbnail.data_id.to_string())
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