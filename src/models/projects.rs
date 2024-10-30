use super::_entities::projects::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub type Projects = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ProjectWithTechnologies {
    pub id: i32,
    pub name: String,
    pub technologies: Vec<String>,
    pub short_description: String,
    pub description: Option<String>,
    pub category: String,
    pub github_url: Option<String>,
    pub download_url: Option<String>,
    pub visit_url: Option<String>,
}
