//! Contains the job repository abstraction class.

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{delete, insert_into, update};

use crate::errors::FettersError;
use crate::models::{JobUpdate, NewJob, QueriedJob, TabledJob};
use crate::schema::{jobs, sprints, statuses, titles};

/// Contains all methods pertaining to CRUD operations for the `jobs` table.
pub struct JobRepository<'a> {
    pub connection: &'a mut SqliteConnection,
}

impl<'a> JobRepository<'a> {
    /// Adds a new job to the `jobs` table.
    pub fn add_job(&mut self, new_job: NewJob) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        Ok(insert_into(jobs)
            .values(&new_job)
            .returning(QueriedJob::as_returning())
            .get_result(self.connection)?)
    }

    /// Updates an existing job with new changes.
    pub fn update_job(
        &mut self,
        job_id: i32,
        changes: JobUpdate,
    ) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        Ok(update(jobs.find(job_id))
            .set(&changes)
            .returning(QueriedJob::as_returning())
            .get_result(self.connection)?)
    }

    /// Deletes an existing job.
    pub fn delete_job(&mut self, job_id: i32) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        Ok(delete(jobs.find(job_id))
            .returning(QueriedJob::as_returning())
            .get_result(self.connection)?)
    }

    /// Search for a job by its record ID.
    pub fn get_job(&mut self, job_id: i32) -> Result<TabledJob, FettersError> {
        Ok(jobs::table
            .left_join(titles::table.on(jobs::title_id.eq(titles::id)))
            .left_join(statuses::table.on(jobs::status_id.eq(statuses::id)))
            .left_join(sprints::table.on(jobs::sprint_id.eq(sprints::id)))
            .select((
                jobs::id,
                jobs::created,
                jobs::company_name,
                titles::name.nullable(),
                statuses::name.nullable(),
                jobs::link,
                jobs::notes,
            ))
            .filter(jobs::id.eq(job_id))
            .first::<TabledJob>(self.connection)?)
    }

    /// List all jobs matching the query.
    pub fn list_jobs(
        &mut self,
        company: &Option<String>,
        link: &Option<String>,
        notes: &Option<String>,
        sprint: &Option<String>,
        status: &Option<String>,
        title: &Option<String>,
    ) -> Result<Vec<TabledJob>, FettersError> {
        let mut query = jobs::table
            .left_join(titles::table.on(jobs::title_id.eq(titles::id)))
            .left_join(statuses::table.on(jobs::status_id.eq(statuses::id)))
            .left_join(sprints::table.on(jobs::sprint_id.eq(sprints::id)))
            .select((
                jobs::id,
                jobs::created,
                jobs::company_name,
                titles::name.nullable(),
                statuses::name.nullable(),
                jobs::link,
                jobs::notes,
            ))
            .into_boxed::<Sqlite>();

        if let Some(company) = company {
            query = query.filter(jobs::company_name.like(format!("%{}%", company)));
        }

        if let Some(link) = link {
            query = query.filter(jobs::link.like(format!("%{}%", link)));
        }

        if let Some(notes) = notes {
            query = query.filter(jobs::notes.like(format!("%{}%", notes)));
        }

        if let Some(sprint) = sprint {
            query = query.filter(sprints::name.like(format!("%{}%", sprint)));
        }

        if let Some(status) = status {
            query = query.filter(statuses::name.like(format!("%{}%", status)));
        }

        if let Some(title) = title {
            query = query.filter(titles::name.like(format!("%{}%", title)));
        }

        Ok(query.load::<TabledJob>(self.connection)?)
    }
}
