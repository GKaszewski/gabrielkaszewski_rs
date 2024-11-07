use sea_orm::entity::prelude::*;
use super::_entities::project_thumbnails::{ActiveModel, Entity};
pub type ProjectThumbnails = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}
