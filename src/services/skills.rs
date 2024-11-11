use loco_rs::prelude::*;
use sea_orm::QueryOrder;

use crate::models::_entities::skills::{Column, Entity, Model};

pub async fn get_all_skills(ctx: &AppContext) -> Result<Vec<Model>> {
    let skills = Entity::find()
    .order_by_asc(Column::Name)
    .all(&ctx.db).await?;
    Ok(skills)
}
