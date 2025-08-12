//! Contains a function called by the CLI when deleting a job.

use diesel::SqliteConnection;

use crate::{errors::FettersError, models::QueriedSprint};

/// Delete a tracked job application.
pub fn delete_job(
    connection: &mut SqliteConnection,
    company: &Option<String>,
    link: &Option<String>,
    notes: &Option<String>,
    sprint: &Option<String>,
    status: &Option<String>,
    title: &Option<String>,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    Ok(())
}
