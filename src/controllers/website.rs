#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::models::users;
use crate::views;

pub async fn render_index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    views::website::index(v, &ctx).await
}

pub async fn render_login(ViewEngine(v): ViewEngine<TeraView>) -> impl IntoResponse {
    views::auth::login(v).await
}

pub async fn render_upload(
    auth: auth::JWT,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let _current_user = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    views::data::upload(v).await
}

pub async fn render_about(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::website::about(v).await
}

pub async fn render_projects(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    views::projects::projects(v, &ctx).await
}

pub async fn render_project_detail(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    views::projects::project_detail(v, &ctx, id).await
}

pub async fn render_project_detail_from_name(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse> {
    views::projects::project_detail_from_name(v, &ctx, name).await
}

pub async fn render_create_project(
    auth: auth::JWT,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    views::projects::create_project(v).await
}

pub async fn render_data(
    auth: auth::JWT,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
     match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    views::data::list(v, &ctx).await    
}

async fn render_create_job(
    auth: auth::JWT,
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    views::job::create_job(v).await
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(render_index))
        .add("/upload", get(render_upload))
        .add("/login", get(render_login))
        .add("/projects", get(render_projects))
        .add("/projects/create", get(render_create_project))
        .add("/projects/:id", get(render_project_detail))
        .add("/projects/project/:name", get(render_project_detail_from_name))
        .add("/jobs/create", get(render_create_job))
        .add("/data", get(render_data))
        .add("/about", get(render_about))
}
