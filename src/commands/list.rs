//! Contains a function called by the CLI when listing job applications.

use diesel::SqliteConnection;

use crate::{
    errors::FettersError,
    models::QueriedSprint,
    repositories::{job::JobRepository, sprint::SprintRepository},
    utils::display::display_jobs,
};

/// List all job applications stored in the `jobs` SQLite table.
pub fn list_jobs(
    connection: &mut SqliteConnection,
    company: &Option<String>,
    link: &Option<String>,
    notes: &Option<String>,
    sprint: &Option<String>,
    status: &Option<String>,
    title: &Option<String>,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let mut job_repo = JobRepository { connection };
    let all_jobs = job_repo.list_jobs(company, link, notes, sprint, status, title)?;

    display_jobs(all_jobs, &sprint.as_ref().unwrap_or(&current_sprint.name));

    Ok(())
}
