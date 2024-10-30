#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::views;

pub async fn render_index(
    ViewEngine(v): ViewEngine<TeraView>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    views::website::index(v, &ctx).await
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(render_index))
}
