use loco_rs::prelude::*;

use crate::services::projects;

pub struct DeleteProject;

#[async_trait]
impl Task for DeleteProject {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "delete_project".to_string(),
            detail: "Task for deleting a project".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let project_id = vars.cli_arg("id")?;
        let project_id = project_id.parse::<i32>();

        let project_id = match project_id {
            Ok(project_id) => project_id,
            Err(_) => return Err(Error::Any("Invalid project ID".into())),
        };

        projects::delete_project(app_context, project_id).await?;

        tracing::info!("Project {} deleted successfully", project_id);
        Ok(())
    }
}