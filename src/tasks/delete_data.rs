use loco_rs::prelude::*;

use crate::services;

pub struct DeleteData;

#[async_trait]
impl Task for DeleteData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "delete_data".to_string(),
            detail: "Task for deleting all data".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let file_name = vars.cli_arg("name")?;

        services::data::delete_data_by_file_name(&file_name, &app_context).await?;

        tracing::info!("Data file {} deleted successfully", file_name);
        Ok(())
    }
}