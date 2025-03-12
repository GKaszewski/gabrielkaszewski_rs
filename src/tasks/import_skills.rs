use loco_rs::prelude::*;
use serde::Deserialize;

use crate::services::skills::add_skills;

pub struct ImportSkills;

#[derive(Deserialize)]
struct Skill {
    name: String,
}

#[derive(Deserialize)]
struct Skills {
    skills: Vec<Skill>,
}

#[async_trait]
impl Task for ImportSkills {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "import_skills".to_string(),
            detail: "Task for importing skills from json file or stdin".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let from_file = vars.cli_arg("from_file").ok().map(|v| v.parse::<bool>().unwrap_or(false)).unwrap_or(false);

        match from_file {
            true => {
                let file_path = vars.cli_arg("file_path")?;

                let data = std::fs::read_to_string(file_path)?;
                let skills: Skills = serde_json::from_str(&data)?;
                process_skills(app_context, skills).await?;

                Ok(())
            },
            false => {
                let raw_data = vars.cli_arg("raw_data")?;
                let skills: Skills = get_skills_from_raw_data(&raw_data);
                process_skills(app_context, skills).await?;

                Ok(())
            },
        }
        
    }
}

fn get_skills_from_raw_data(raw_data: &str) -> Skills {
    let skills = raw_data.split(',').map(|s| Skill { name: s.to_string() }).collect();
    Skills { skills }
}

async fn process_skills(app_context: &AppContext, skills: Skills) -> Result<()> {
    let skills_names = skills.skills.iter().map(|s| s.name.clone()).collect();
    let items = add_skills(app_context, skills_names).await?;

    for item in items {
        tracing::info!(
            skill_id = item.id,
            skill_name = &item.name,
            "Skill created successfully",
        );
    }

    Ok(())
}