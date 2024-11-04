use loco_rs::prelude::*;

use crate::services;

pub struct AddDataFile;

#[async_trait]
impl Task for AddDataFile {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "add_data_file".to_string(),
            detail: "Task for adding a new data file".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let file_path = vars.cli_arg("file_path")?;
        let file_name = vars.cli_arg("file_name")?;
        let protected = vars.cli_arg("protected")?;
        let uuid_name = vars.cli_arg("uuid_name")?;

        let protected = protected.parse::<bool>().unwrap_or(false);
        let uuid_name = uuid_name.parse::<bool>().unwrap_or(false);

        services::data::add_data_file_from_path(
            &app_context,
            file_path,
            file_name,
            protected,
            uuid_name,
        )
        .await?;

        tracing::info!("Data file added successfully");

        Ok(())
    }
}
