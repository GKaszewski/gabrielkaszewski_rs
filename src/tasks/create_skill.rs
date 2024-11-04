use loco_rs::prelude::*;

use crate::models::_entities::skills::ActiveModel;

pub struct CreateSkillData;

#[async_trait]
impl Task for CreateSkillData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "create_skill".to_string(),
            detail: "Task for creating a new skill".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let name = vars.cli_arg("name")?;

        let mut item = ActiveModel {
            ..Default::default()
        };

        item.name = Set(name.to_string());

        let item = item.insert(&app_context.db).await?;

        tracing::info!(
            skill_id = item.id,
            skill_name = &item.name,
            "Skill created successfully",
        );

        Ok(())
    }
}
