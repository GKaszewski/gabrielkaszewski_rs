use loco_rs::prelude::*;

use crate::models::{
    _entities::jobs::{ActiveModel, Entity, Model},
    jobs::{get_technologies_from_string, JobWithTechnologies},
};

pub async fn get_all_jobs(ctx: &AppContext) -> Result<Vec<Model>> {
    let jobs = Entity::find().all(&ctx.db).await?;
    Ok(jobs)
}

pub async fn get_all_jobs_with_technologies(ctx: &AppContext) -> Result<Vec<JobWithTechnologies>> {
    let jobs = Entity::find().all(&ctx.db).await?;
    let jobs_with_technologies = jobs
        .into_iter()
        .map(|job| {
            let technologies = get_technologies_from_string(&job.technologies);
            JobWithTechnologies {
                id: job.id,
                position: job.position,
                company: job.company,
                start_date: job.start_date,
                end_date: job.end_date,
                technologies,
                still_working: job.still_working,
            }
        })
        .collect();
    Ok(jobs_with_technologies)
}
