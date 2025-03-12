use loco_rs::prelude::*;
use sea_orm::QueryOrder;

use crate::models::_entities::skills::{ActiveModel, Column, Entity, Model};

pub async fn get_all_skills(ctx: &AppContext) -> Result<Vec<Model>> {
    let skills = Entity::find()
    .order_by_asc(Column::Name)
    .all(&ctx.db).await?;
    Ok(skills)
}

pub async fn add_skill(ctx: &AppContext, name: String) -> Result<Model> {
    let new_skill = ActiveModel {
        name: Set(name),
        ..Default::default()
    };
    let new_skill = new_skill.insert(&ctx.db).await?;
    Ok(new_skill)
}

pub async fn add_skills(ctx: &AppContext, skills: Vec<String>) -> Result<Vec<Model>> {
    let mut new_skills = vec![];
    for skill in skills {
        let new_skill = ActiveModel {
            name: Set(skill),
            ..Default::default()
        };
        let new_skill = new_skill.insert(&ctx.db).await?;
        new_skills.push(new_skill);
    }
    Ok(new_skills)
}