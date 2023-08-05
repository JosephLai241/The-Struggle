//! Contains functions pertaining to querying the SQLite instance.

use diesel::{query_dsl::RunQueryDsl, SqliteConnection};

use crate::{
    errors::FettersError,
    models::{job::Job, stint::Stint},
    schema::{job_data::dsl::*, stints::dsl::*},
};

/// Retrieve all stored jobs from SQLite.
pub fn get_all_jobs(connection: &mut SqliteConnection) -> Result<Vec<Job>, FettersError> {
    let jobs = job_data.load(connection)?;

    Ok(jobs)
}

/// Retrieve all stored stints from SQLite.
pub fn get_all_stints(connection: &mut SqliteConnection) -> Result<Vec<Stint>, FettersError> {
    let all_stints = stints.load(connection)?;

    Ok(all_stints)
}

/// Write a new job to the SQLite instance.
pub fn write_job(connection: &mut SqliteConnection, job: Job) -> Result<(), FettersError> {
    diesel::insert_into(job_data)
        .values(&job)
        .execute(connection)?;

    Ok(())
}

/// Write a new stint to the SQLite instance.
pub fn write_stint(
    connection: &mut SqliteConnection,
    stint_name: String,
) -> Result<(), FettersError> {
    let new_stint = Stint::new(stint_name);

    diesel::insert_into(stints)
        .values(&new_stint)
        .execute(connection)?;

    Ok(())
}
