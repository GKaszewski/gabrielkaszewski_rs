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

use crate::models::users;
use crate::services;

async fn get_data(
    auth: Option<auth::JWT>,
    range: Option<TypedHeader<Range>>,
    Path(file_name): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Ranged<KnownSize<File>>> {
    services::data::serve_data_file(&auth, range, &file_name, &ctx).await
}

async fn upload_data(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    payload: Multipart,
) -> Result<Response> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    services::data::add(&auth, &ctx, payload, true).await?;
    format::html("<h1>File uploaded successfully</h1>")
}

async fn delete_data(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(file_id): Path<i32>,
) -> Result<Response> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    services::data::delete_data_by_id(file_id, &ctx).await?;
    format::html("<h1>File deleted successfully</h1>")
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/data")
        .add("/upload", post(upload_data))
        .add("/:file_name", get(get_data))
        .add("/id/:file_id", delete(delete_data))
}
