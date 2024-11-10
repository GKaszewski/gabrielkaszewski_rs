use loco_rs::prelude::*;

use crate::services;

pub async fn upload(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "website/data-upload.html", data!({}))
}

pub async fn list(v: impl ViewRenderer, ctx: &AppContext) -> Result<impl IntoResponse> {
    let data = services::data::get_all_data(&ctx).await?;
    format::render().view(&v, "website/data.html", data!({ "data": data }))
}