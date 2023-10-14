//! Contains functions pertaining to querying the SQLite instance.

use diesel::{
    query_dsl::{methods::FilterDsl, RunQueryDsl},
    ExpressionMethods, SqliteConnection,
};

use crate::{
    errors::FettersError,
    models::{job::Job, stint::Stint},
};

/// Retrieve all stored jobs from SQLite.
pub fn get_all_jobs(connection: &mut SqliteConnection) -> Result<Vec<Job>, FettersError> {
    use crate::schema::job_data::dsl::*;

    let jobs = job_data.load(connection)?;

    Ok(jobs)
}

/// Retrieve all stored stints from SQLite.
pub fn get_all_stints(connection: &mut SqliteConnection) -> Result<Vec<Stint>, FettersError> {
    use crate::schema::stints::dsl::*;

    let all_stints = stints.load(connection)?;

    Ok(all_stints)
}

/// Write a new job to the SQLite instance.
pub fn write_job(connection: &mut SqliteConnection, job: Job) -> Result<(), FettersError> {
    use crate::schema::job_data::dsl::*;

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
    use crate::schema::stints::dsl::*;

    let new_stint = Stint::new(stint_name);

    diesel::insert_into(stints)
        .values(&new_stint)
        .execute(connection)?;

    Ok(())
}

/// Delete a job from the SQLite instance.
pub fn delete_job(connection: &mut SqliteConnection, job_id: i32) -> Result<(), FettersError> {
    use crate::schema::job_data::dsl::*;

    diesel::delete(job_data.filter(id.eq(job_id))).execute(connection)?;

    Ok(())
}
