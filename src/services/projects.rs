use loco_rs::prelude::*;

use crate::{models::{_entities::{project_thumbnails, projects::{self, Entity, Model}}, projects::{get_category_from_string, ProjectDto}}, shared::get_technologies_from_string::get_technologies_from_string};

pub async fn get_all_projects(ctx: &AppContext) -> Result<Vec<Model>> {
    let projects = Entity::find().all(&ctx.db).await?;
    Ok(projects)
}

pub async fn get_project_by_id(ctx: &AppContext, id: i32) -> Result<Model> {
    let project = Entity::find_by_id(id).one(&ctx.db).await?;
    let project = project.ok_or_else(|| ModelError::EntityNotFound)?;
    Ok(project)
}

pub async fn get_archived_projects(ctx: &AppContext) -> Result<Vec<Model>> {
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

pub async fn get_highlighted_projects(ctx: &AppContext) -> Result<Vec<Model>> {
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