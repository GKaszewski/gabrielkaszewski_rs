use loco_rs::prelude::*;

use crate::services;

pub async fn index(v: impl ViewRenderer, ctx: &AppContext) -> Result<impl IntoResponse> {
    let skills = services::skills::get_all_skills(ctx).await?;
    let jobs = services::jobs::get_all_jobs_with_technologies(ctx).await?;

    format::render().view(
        &v,
        "website/index.html",
        data!({ "skills": skills, "jobs": jobs }),
    )
}

pub async fn projects(v: impl ViewRenderer, ctx: &AppContext) -> Result<impl IntoResponse> {
    let projects = services::projects::get_all_projects_dto(ctx).await?;

    format::render().view(&v, "website/projects.html", data!({"projects": projects}))
}

pub async fn project_detail(v: impl ViewRenderer, ctx: &AppContext, id: i32) -> Result<impl IntoResponse> {
    let project = services::projects::get_project_dto(ctx, id).await?;

    format::render().view(&v, "website/project_detail.html", data!({"project": project}))
}

pub async fn project_detail_from_name(v: impl ViewRenderer, ctx: &AppContext, name: String) -> Result<impl IntoResponse> {
    let project = services::projects::get_project_dto_by_name(ctx, &name).await?;

    format::render().view(&v, "website/project_detail.html", data!({"project": project}))
}

pub async fn about(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    let age = services::website::get_current_age();

    format::render().view(&v, "website/about.html", data!({"age": age}))
}
