#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;

mod m20241029_235230_skills;
mod m20241030_002154_jobs;
mod m20241030_024340_projects;
mod m20241030_024830_data;
mod m20241106_005545_project_thumbnails;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // inject-below
            Box::new(m20241106_005545_project_thumbnails::Migration),
            Box::new(m20241030_024830_data::Migration),
            Box::new(m20241030_024340_projects::Migration),
            Box::new(m20241030_002154_jobs::Migration),
            Box::new(m20241029_235230_skills::Migration),
            Box::new(m20220101_000001_users::Migration),
        ]
    }
}