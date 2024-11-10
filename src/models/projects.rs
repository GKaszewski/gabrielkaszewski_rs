use super::_entities::projects::{ActiveModel, Entity};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
pub type Projects = Entity;

impl ActiveModelBehavior for ActiveModel {
    // extend activemodel below (keep comment for generators)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Category {
    Web,
    Mobile,
    Desktop,
    Game,
    Api,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDto {
    pub id: i32,
    pub name: String,
    pub short_description: String,
    pub description: Option<String>,
    pub category: Category,
    pub github_url: Option<String>,
    pub download_url: Option<String>,
    pub visit_url: Option<String>,
    pub technologies: Vec<String>,
    pub thumbnails: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub short_description: String,
    pub description: Option<String>,
    pub category: Category,
    pub github_url: Option<String>,
    pub download_url: Option<String>,
    pub visit_url: Option<String>,
    pub technologies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectForm {
    pub name: String,
    pub short_description: String,
    pub description: Option<String>,
    pub category: String,
    pub github_url: Option<String>,
    pub download_url: Option<String>,
    pub visit_url: Option<String>,
    pub technologies: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub short_description: Option<String>,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub github_url: Option<String>,
    pub download_url: Option<String>,
    pub visit_url: Option<String>,
    pub technologies: Option<Vec<String>>,
}

pub fn get_category_from_string(category: &str) -> Category {
    match category {
        "Web" => Category::Web,
        "Mobile" => Category::Mobile,
        "Desktop" => Category::Desktop,
        "Game" => Category::Game,
        "Api" => Category::Api,
        "web" => Category::Web,
        "mobile" => Category::Mobile,
        "desktop" => Category::Desktop,
        "game" => Category::Game,
        "api" => Category::Api,
        _ => Category::Desktop,
    }
}

pub fn get_string_from_category(category: &Category) -> String {
    match category {
        Category::Web => "Web".to_string(),
        Category::Mobile => "Mobile".to_string(),
        Category::Desktop => "Desktop".to_string(),
        Category::Game => "Game".to_string(),
        Category::Api => "Api".to_string(),
    }
}