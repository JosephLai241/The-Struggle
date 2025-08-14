//! Contains a function called by the CLI when deleting a job.

use diesel::SqliteConnection;
use inquire::{Confirm, Select};
use owo_colors::OwoColorize;

use crate::{
    cli::QueryArgs,
    errors::FettersError,
    models::sprint::QueriedSprint,
    repositories::job::JobRepository,
    utils::{display::display_jobs, prompt::get_inquire_config},
};

/// Delete a tracked job application.
pub fn delete_job(
    connection: &mut SqliteConnection,
    query_args: &mut QueryArgs,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let default_sprint = Some(current_sprint.name.clone());

    // Search the default sprint if no sprint filter was specified.
    if query_args.sprint.is_none() {
        query_args.sprint = default_sprint;
    }

    let mut job_repo = JobRepository { connection };
    let matched_jobs = job_repo.list_jobs(query_args, current_sprint)?;

    if matched_jobs.is_empty() {
        return Err(FettersError::NoJobsAvailable(
            query_args
                .sprint
                .clone()
                .as_ref()
                .unwrap_or(&current_sprint.name.clone())
                .to_string(),
        ));
    }

    display_jobs(
        &matched_jobs,
        query_args.sprint.as_ref().unwrap_or(&current_sprint.name),
    );

    if let Some(job) = Select::new("Select the job you want to delete:", matched_jobs)
        .with_render_config(get_inquire_config())
        .prompt_skippable()?
    {
        match Confirm::new("Confirm deletion?")
            .with_default(true)
            .with_render_config(get_inquire_config())
            .prompt_skippable()?
        {
            Some(true) => {
                let mut job_repo = JobRepository { connection };
                job_repo.delete_job(job.id)?;

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
