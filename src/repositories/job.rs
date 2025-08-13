//! Contains the job repository abstraction class.

use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use diesel::{delete, insert_into, update};

use crate::cli::QueryArgs;
use crate::errors::FettersError;
use crate::models::{JobUpdate, NewJob, QueriedJob, QueriedSprint, TabledJob};
use crate::repositories::sprint::SprintRepository;
use crate::schema::{jobs, sprints, statuses, titles};

/// Contains all methods pertaining to CRUD operations for the `jobs` table.
pub struct JobRepository<'a> {
    pub connection: &'a mut SqliteConnection,
}

impl<'a> JobRepository<'a> {
    /// Adds a new job to the `jobs` table.
    pub fn add_job(&mut self, new_job: NewJob) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        let queried_job = insert_into(jobs)
            .values(&new_job)
            .returning(QueriedJob::as_returning())
            .get_result(self.connection)?;

        let mut sprint_repo = SprintRepository {
            connection: self.connection,
        };
        sprint_repo.increment_num_jobs(new_job.sprint_id)?;

        Ok(queried_job)
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

        let queried_job = delete(jobs.find(job_id))
            .returning(QueriedJob::as_returning())
            .get_result(self.connection)?;

        let mut sprint_repo = SprintRepository {
            connection: self.connection,
        };
        sprint_repo.decrement_num_jobs(queried_job.sprint_id)?;

        Ok(queried_job)
    }

    /// List all jobs matching the query.
    pub fn list_jobs(
        &mut self,
        query_args: &QueryArgs,
        current_sprint: &QueriedSprint,
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

        if let Some(sprint) = &query_args.sprint {
            query = query.filter(sprints::name.like(format!("%{}%", sprint)));
        } else {
            query = query.filter(sprints::id.eq(current_sprint.id));
        }

        if let Some(company) = &query_args.company {
            query = query.filter(jobs::company_name.like(format!("%{}%", company)));
        }

        if let Some(link) = &query_args.link {
            query = query.filter(jobs::link.like(format!("%{}%", link)));
        }

        if let Some(notes) = &query_args.notes {
            query = query.filter(jobs::notes.like(format!("%{}%", notes)));
        }

        if let Some(status) = &query_args.status {
            query = query.filter(statuses::name.like(format!("%{}%", status)));
        }

        if let Some(title) = &query_args.title {
            query = query.filter(titles::name.like(format!("%{}%", title)));
        }

        Ok(query.load::<TabledJob>(self.connection)?)
    }
}
