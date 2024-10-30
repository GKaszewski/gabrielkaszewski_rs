use loco_rs::prelude::*;

use crate::models::_entities::skills::{Entity, Model};

pub async fn get_all_skills(ctx: &AppContext) -> Result<Vec<Model>> {
    let skills = Entity::find().all(&ctx.db).await?;
    Ok(skills)
}
