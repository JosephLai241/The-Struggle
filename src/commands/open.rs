//! Contains a function called by the CLI when opening a job application in the browser.

use diesel::SqliteConnection;
use inquire::MultiSelect;
use owo_colors::OwoColorize;

use crate::{
    cli::QueryArgs, errors::FettersError, models::QueriedSprint, repositories::job::JobRepository,
    utils::display::display_jobs,
};

/// Open the link associated with a job application in the browser.
pub fn open_application(
    connection: &mut SqliteConnection,
    query_args: &mut QueryArgs,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let default_sprint = Some(current_sprint.name.clone());

    // Search the default sprint if no sprint filter was specified.
    if let None = query_args.sprint {
        query_args.sprint = default_sprint;
    }

    let mut job_repo = JobRepository { connection };
    let matched_jobs = job_repo.list_jobs(&query_args, current_sprint)?;

    display_jobs(
        &matched_jobs,
        &query_args.sprint.as_ref().unwrap_or(&current_sprint.name),
    );

    if let Some(selected_jobs) = MultiSelect::new(
        "Select the job applications you want to open in the browser:",
        matched_jobs,
    )
    .prompt_skippable()?
    {
        for job in selected_jobs {
            if let Some(link) = job.link {
                open::that(link)?;
                continue;
            }

            println!(
                "{}",
                "Job with ID {id} has no associated link. Cannot open link in browser!"
                    .red()
                    .bold()
            );
        }
    }

    Ok(())
}
