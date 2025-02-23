use loco_rs::prelude::*;
use sea_orm::QueryOrder;

use crate::{models::{
    _entities::jobs::{ActiveModel, Column, Entity, Model},
    jobs::{CreateJobForm, JobWithTechnologies},
}, shared::get_technologies_from_string::get_technologies_from_string};

pub async fn get_all_jobs(ctx: &AppContext) -> Result<Vec<Model>> {
    let jobs = Entity::find().all(&ctx.db).await?;
    Ok(jobs)
}

pub async fn get_all_jobs_with_technologies(ctx: &AppContext) -> Result<Vec<JobWithTechnologies>> {
    let jobs = Entity::find()
    .order_by_asc(Column::StartDate)
    .all(&ctx.db).await?;
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

pub async fn create_job_from_form(ctx: &AppContext, job_data: &CreateJobForm) -> Result<Model> {
    let new_job = ActiveModel {
        company: Set(job_data.company.clone()),
        position: Set(job_data.position.clone()),
        start_date: Set(job_data.start_date.clone()),
        end_date: Set(job_data.end_date.clone()),
        technologies: Set(job_data.technologies.clone()),
        still_working: Set(job_data.still_working),
        ..Default::default()
    };

    let job = new_job.insert(&ctx.db).await?;

    Ok(job)
}
