//! Contains a function called by the CLI when displaying application insights.

use diesel::SqliteConnection;

use crate::{
    errors::FettersError, models::sprint::QueriedSprint, repositories::job::JobRepository,
    utils::display::display_insights,
};

/// Display all job insights.
pub fn show_insights(
    connection: &mut SqliteConnection,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let mut job_repo = JobRepository { connection };
    let jobs_per_status = job_repo.count_jobs_per_status(current_sprint)?;
    let jobs_per_sprint = job_repo.count_jobs_per_sprint(current_sprint)?;

    if !jobs_per_status.is_empty() && !jobs_per_sprint.is_empty() {
        display_insights(jobs_per_status, "JOBS PER STATUS", false);
        display_insights(jobs_per_sprint, "JOBS PER SPRINT", true);
    } else {
        return Err(FettersError::NoJobsAvailable(current_sprint.name.clone()));
    }

    Ok(())
}
