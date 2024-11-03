#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::extract::Multipart;
use loco_rs::prelude::*;
use tokio::fs::File;

use axum_range::KnownSize;
use axum_range::Ranged;

use axum_extra::headers::Range;
use axum_extra::TypedHeader;

use crate::services;

pub async fn get_data(
    auth: auth::JWT,
    range: Option<TypedHeader<Range>>,
    Path(file_name): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Ranged<KnownSize<File>>> {
    services::data::serve_data_file(&auth, range, &file_name, &ctx).await
}

pub async fn upload_data(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    payload: Multipart,
) -> Result<Response> {
    services::data::add(&auth, &ctx, payload).await?;
    format::html("<h1>File uploaded successfully</h1>")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/data/")
        .add("/upload", post(upload_data))
        .add("/:file_name", get(get_data))
}
