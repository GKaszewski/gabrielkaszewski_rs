use loco_rs::prelude::*;

pub async fn upload(v: impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(&v, "website/data-upload.html", data!({}))
}
