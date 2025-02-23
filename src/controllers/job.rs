#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{
    models::{jobs::CreateJobForm, users},
    services,
};

async fn create_job(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Form(job_data): Form<CreateJobForm>,
) -> Result<Response> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => {}
        Err(_) => return unauthorized("Unauthorized"),
    }

    let job = services::jobs::create_job_from_form(&ctx, &job_data).await?;
    format::json(&job)
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/jobs/").add("/", post(create_job))
}
