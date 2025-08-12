//! Contains a function called by the CLI when deleting a job.

use diesel::SqliteConnection;
use inquire::{Confirm, Select};
use owo_colors::OwoColorize;

use crate::{
    cli::QueryArgs, errors::FettersError, models::QueriedSprint, repositories::job::JobRepository,
    utils::display::display_jobs,
};

/// Delete a tracked job application.
pub fn delete_job(
    connection: &mut SqliteConnection,
    query_args: &QueryArgs,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let mut job_repo = JobRepository { connection };
    let all_jobs = job_repo.list_jobs(&query_args)?;

    display_jobs(
        &all_jobs,
        &query_args.sprint.as_ref().unwrap_or(&current_sprint.name),
    );

    if let Some(job_id) = Select::new(
        "Select the ID of the job you want to delete:",
        all_jobs.into_iter().map(|job| job.id).collect(),
    )
    .with_vim_mode(true)
    .prompt_skippable()?
    {
        match Confirm::new("Confirm deletion?").prompt_skippable()? {
            Some(true) => {
                let mut job_repo = JobRepository { connection };
                job_repo.delete_job(job_id)?;

                println!(
                    "{}",
                    format!("\nDeleted entry for sprint [{}]!\n", current_sprint.name)
                        .green()
                        .bold()
                );

                return Ok(());
            }
            Some(false) => {
                println!("{}", "Cancelled.".red().bold());
                return Ok(());
            }
            None => println!("{}", "Invalid input, try again".red().bold()),
        }
    }

    Ok(())
}
