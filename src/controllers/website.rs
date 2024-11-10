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
    views::website::projects(v, &ctx).await
}

pub async fn render_project_detail(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse> {
    views::website::project_detail(v, &ctx, id).await
}

pub async fn render_project_detail_from_name(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
    Path(name): Path<String>,
) -> Result<impl IntoResponse> {
    views::website::project_detail_from_name(v, &ctx, name).await
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(render_index))
        .add("/upload", get(render_upload))
        .add("/login", get(render_login))
        .add("/projects", get(render_projects))
        .add("/projects/:id", get(render_project_detail))
        .add("/projects/project/:name", get(render_project_detail_from_name))
        .add("/about", get(render_about))
}
