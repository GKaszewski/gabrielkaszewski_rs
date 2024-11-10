use super::_entities::jobs::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub type Jobs = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JobWithTechnologies {
    pub id: i32,
    pub position: String,
    pub company: String,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub technologies: Vec<String>,
    pub still_working: bool,
}