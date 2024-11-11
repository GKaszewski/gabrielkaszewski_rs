#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::extract::Multipart;
use format::redirect;
use loco_rs::prelude::*;

use crate::{
    models::{
        projects::{get_category_from_string, CreateProject, CreateProjectForm},
        users,
    },
    services,
    shared::get_technologies_from_string::get_technologies_from_string,
};

async fn create_project(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Form(project_data): Form<CreateProjectForm>,
) -> Result<Response> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    let technologies = get_technologies_from_string(&project_data.technologies);

    let project_data = CreateProject {
        name: project_data.name,
        description: project_data.description,
        technologies,
        category: get_category_from_string(&project_data.category),
        download_url: project_data.download_url,
        github_url: project_data.github_url,
        visit_url: project_data.visit_url,
        short_description: project_data.short_description,
    };

    let project = services::projects::add_project(&ctx, project_data).await?;
    format::json(&project)
}

async fn create_project_with_thumbnails(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    payload: Multipart,
) -> Result<Response> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    services::projects::add_project_with_thumbnails_multipart(&auth, &ctx, payload).await?;
    
    redirect("/projects")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/projects/")
        .add("/", post(create_project))
        .add("/upload", post(create_project_with_thumbnails))
}
