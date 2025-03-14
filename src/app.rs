use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    storage::{self, Storage},
    task::Tasks,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{
    controllers, initializers, models::_entities::users, tasks, workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::job::routes())
            .add_route(controllers::project::routes())
            .add_route(controllers::data::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::website::routes())
    }

    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        // Create uploads directory if it doesn't exist
        let _ = std::fs::create_dir("uploads");


        let store = storage::drivers::local::new_with_prefix("uploads").map_err(Box::from)?;

        Ok(AppContext {
            storage: Storage::single(store).into(),
            ..ctx
        })
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
        tasks.register(tasks::create_user::CreateUserData);
        tasks.register(tasks::create_job::CreateJobData);
        tasks.register(tasks::create_skill::CreateSkillData);
        tasks.register(tasks::import_skills::ImportSkills);
        tasks.register(tasks::add_data_file::AddDataFile);
        tasks.register(tasks::delete_data::DeleteData);
        tasks.register(tasks::clear_data::ClearData);
        tasks.register(tasks::delete_project::DeleteProject);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}