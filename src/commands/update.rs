//! Contains a function called by the CLI when updating a job.

use diesel::SqliteConnection;
use inquire::Select;

use crate::{
    errors::FettersError, models::QueriedSprint, repositories::job::JobRepository,
    utils::display::display_jobs,
};

/// Update a tracked job application.
pub fn update_job(
    connection: &mut SqliteConnection,
    company: &Option<String>,
    link: &Option<String>,
    notes: &Option<String>,
    sprint: &Option<String>,
    status: &Option<String>,
    title: &Option<String>,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    // Query job repo for all jobs, filtering by company name
    //  If no jobs match, exit
    // Select::new() a job in the returned list.
    // MultiSelect::new() fields to edit
    // Confirm update
    // PUT operation on the existing job?

    let default_sprint = Some(current_sprint.name.clone());

    let mut job_repo = JobRepository { connection };
    let all_jobs = job_repo.list_jobs(
        company,
        link,
        notes,
        if let None = sprint {
            &default_sprint
        } else {
            sprint
        },
        status,
        title,
    )?;

    if all_jobs.is_empty() {
        return Err(FettersError::NoJobsAvailable(current_sprint.name.clone()));
    }

    display_jobs(&all_jobs, &current_sprint.name);

    let selected_job = Select::new(
        "Select the ID of the job you want to modify:",
        all_jobs.into_iter().map(|job| job.id).collect(),
    )
    .prompt_skippable()?;

    // Display table of matched jobs
    // display select menu containing IDs for each of those matched jobs

    Ok(())
}
