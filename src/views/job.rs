use loco_rs::prelude::*;

pub async fn create_job(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "website/create-job.html", data!({}))
}
