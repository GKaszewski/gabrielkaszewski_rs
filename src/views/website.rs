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