//! Contains a function called by the CLI when listing job applications.

use diesel::SqliteConnection;

use crate::{
    cli::QueryArgs, errors::FettersError, models::QueriedSprint, repositories::job::JobRepository,
    utils::display::display_jobs,
};

/// List all job applications stored in the `jobs` SQLite table.
pub fn list_jobs(
    connection: &mut SqliteConnection,
    query_args: &QueryArgs,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let mut job_repo = JobRepository { connection };
    let all_jobs = job_repo.list_jobs(&query_args)?;

    if all_jobs.is_empty() {
        return Err(FettersError::NoJobsAvailable(current_sprint.name.clone()));
    }

    display_jobs(
        &all_jobs,
        &query_args.sprint.as_ref().unwrap_or(&current_sprint.name),
    );

    Ok(())
}
