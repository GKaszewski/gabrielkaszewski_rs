use loco_rs::prelude::*;

use crate::services;

pub struct ClearData;

#[async_trait]
impl Task for ClearData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "clear_data".to_string(),
            detail: "Task for clearing all data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        services::data::clear_all_data(&app_context).await?;

        tracing::info!("All data cleared successfully");

        Ok(())
    }
}