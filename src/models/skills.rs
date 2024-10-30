use sea_orm::entity::prelude::*;
use super::_entities::skills::{ActiveModel, Entity};
pub type Skills = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
