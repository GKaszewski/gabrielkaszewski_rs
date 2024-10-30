use sea_orm::entity::prelude::*;
use super::_entities::data::{ActiveModel, Entity};
pub type Data = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
