//! Contains all functionality pertaining to interacting with SQLite.

use std::path::Path;

use diesel::query_dsl::methods::FilterDsl;
use diesel::query_dsl::methods::FindDsl;
use diesel::sqlite::SqliteConnection;
use diesel::{Connection, ExpressionMethods, RunQueryDsl, SelectableHelper};

use crate::errors::FettersError;
use crate::models::{JobUpdate, NewJob, QueriedJob};

/// Contains all functionality pertaining to interacting with the SQLite database.
pub struct Database {
    /// The SQLite connection.
    pub connection: SqliteConnection,
}

impl Database {
    /// Create a new connection to the SQLite database.
    pub fn new_connection(db_path: &str) -> Result<Database, FettersError> {
        let connection = SqliteConnection::establish(db_path)?;
        Ok(Database { connection })
    }

    /// Adds a new job to the `jobs` table.
    pub fn add_job(&mut self, new_job: NewJob) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        Ok(diesel::insert_into(jobs)
            .values(&new_job)
            .returning(QueriedJob::as_returning())
            .get_result(&mut self.connection)?)
    }

    /// Updates an existing job with new changes.
    pub fn update_job(
        &mut self,
        job_id: i32,
        changes: JobUpdate,
    ) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        Ok(diesel::update(jobs.find(job_id))
            .set(&changes)
            .returning(QueriedJob::as_returning())
            .get_result(&mut self.connection)?)
    }

    /// Deletes an existing job.
    pub fn delete_job(&mut self, job_id: i32) -> Result<QueriedJob, FettersError> {
        use crate::schema::jobs::dsl::*;

        Ok(diesel::delete(jobs.find(job_id))
            .returning(QueriedJob::as_returning())
            .get_result(&mut self.connection)?)
    }
}
