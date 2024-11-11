use loco_rs::prelude::*;

use crate::models::users;

pub async fn is_logged_in(
    auth: &auth::JWT,
    ctx: &AppContext,
) -> Result<bool> {
    match users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}