use loco_rs::prelude::*;

use crate::models::users::{self};

pub struct CreateUserData;

#[async_trait]
impl Task for CreateUserData {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "create_user".to_string(),
            detail: "Task for creating a new user".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let username = vars.cli_arg("username")?;
        let email = vars.cli_arg("email")?;
        let password = vars.cli_arg("password")?;

        let user = users::Model::create_with_password(
            &app_context.db,
            &users::RegisterParams {
                name: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            },
        )
        .await?;

        tracing::info!(
            user_id = user.id,
            user_email = &user.email,
            "User created successfully",
        );

        Ok(())
    }
}
