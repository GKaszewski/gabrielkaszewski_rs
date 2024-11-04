use loco_rs::prelude::*;

use crate::models::_entities::jobs::ActiveModel;

pub struct CreateJobData;

#[async_trait]
impl Task for CreateJobData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "create_job".to_string(),
            detail: "Task for creating a new job".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let company = vars.cli_arg("company")?;
        let position = vars.cli_arg("position")?;
        let start_date = vars.cli_arg("start_date")?;
        let end_date = vars.cli_arg("end_date")?;
        let technologies = vars.cli_arg("technologies")?;
        let still_working = vars.cli_arg("still_working")?;

        let start_date = start_date.parse::<Date>();
        let end_date = end_date.parse::<Date>();
        let still_working = still_working.parse::<bool>();

        let mut item = ActiveModel {
            ..Default::default()
        };

        item.company = Set(company.to_string());
        item.position = Set(position.to_string());
        if let Ok(start_date) = start_date {
            item.start_date = Set(start_date);
        }
        if let Ok(end_date) = end_date {
            item.end_date = Set(Some(end_date));
        }
        item.technologies = Set(technologies.to_string());
        if let Ok(still_working) = still_working {
            item.still_working = Set(still_working);
        }

        let item = item.insert(&app_context.db).await?;

        tracing::info!(
            job_id = item.id,
            job_position = &item.position,
            "Job created successfully",
        );

        Ok(())
    }
}
