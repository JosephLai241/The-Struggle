//! Contains a function called by the CLI when updating a job.

use diesel::SqliteConnection;
use inquire::{Confirm, MultiSelect, Select, Text};
use owo_colors::OwoColorize;
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::{
    cli::QueryArgs,
    errors::FettersError,
    models::{JobUpdate, NewTitle, QueriedSprint},
    repositories::{
        job::JobRepository, sprint::SprintRepository, statuses::StatusRepository,
        title::TitleRepository,
    },
    utils::{
        display::display_jobs,
        titles::{TitleType, create_or_use_title},
    },
};

/// Update a tracked job application.
pub fn update_job(
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
    let matched_jobs = job_repo.list_jobs(&query_args)?;

    if matched_jobs.is_empty() {
        return Err(FettersError::NoJobsAvailable(current_sprint.name.clone()));
    }

    display_jobs(&matched_jobs, &current_sprint.name);

    if let Some(job) = Select::new("Select the ID of the job you want to modify:", matched_jobs)
        .with_vim_mode(true)
        .prompt_skippable()?
    {
        if let Some(selections) = MultiSelect::new(
            "Select the fields you want to update:",
            UpdatableField::iter().collect(),
        )
        .with_vim_mode(true)
        .prompt_skippable()?
        {
            let mut new_company_name: Option<String> = None;
            let mut new_link: Option<String> = None;
            let mut new_notes: Option<String> = None;
            let mut new_sprint_id: Option<i32> = None;
            let mut new_status_id: Option<i32> = None;
            let mut new_title_id: Option<i32> = None;

            for selection in selections {
                match selection {
                    UpdatableField::CompanyName => {
                        new_company_name = Some(input_prompt(&selection)?);
                    }
                    UpdatableField::Link => {
                        new_link = Some(input_prompt(&selection)?);
                    }
                    UpdatableField::Notes => {
                        new_notes = Some(input_prompt(&selection)?);
                    }
                    UpdatableField::Sprint => {
                        set_new_sprint(connection, &mut new_sprint_id)?;
                    }
                    UpdatableField::Status => {
                        set_new_status(connection, &mut new_status_id)?;
                    }
                    UpdatableField::Title => {
                        set_new_title(connection, &mut new_title_id)?;
                    }
                }
            }

            match Confirm::new("Confirm updates?").prompt_skippable()? {
                Some(true) => {
                    let job_update = JobUpdate {
                        company_name: new_company_name.as_deref(),
                        title_id: new_title_id,
                        status_id: new_status_id,
                        link: new_link.as_deref(),
                        notes: new_notes.as_deref(),
                        sprint_id: new_sprint_id,
                    };

                    let mut job_repo = JobRepository { connection };
                    job_repo.update_job(job.id, job_update)?;

                    println!(
                        "{}",
                        format!("\nUpdated entry for sprint [{}]!\n", current_sprint.name)
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
    }

    Ok(())
}

/// This enum contains all updatable fields users can choose from.
#[derive(Debug, Display, EnumIter)]
enum UpdatableField {
    /// Update the company name.
    #[strum(to_string = "Company Name")]
    CompanyName,
    /// Update the job title.
    #[strum(to_string = "Title")]
    Title,
    /// Update the application status.
    #[strum(to_string = "Status")]
    Status,
    /// Update the link to the job listing/application.
    #[strum(to_string = "Link")]
    Link,
    /// Update notes for this application.
    #[strum(to_string = "Notes")]
    Notes,
    /// Update the sprint this job belongs to.
    #[strum(to_string = "Sprint")]
    Sprint,
}

/// Show an input prompt for text-based fields.
fn input_prompt(updatable_field: &UpdatableField) -> Result<String, FettersError> {
    let message = match updatable_field {
        UpdatableField::CompanyName => "Enter a new company name:",
        UpdatableField::Title => "Enter a new job title:",
        UpdatableField::Link => "Enter a new link to this job listing:",
        UpdatableField::Notes => "Enter new notes for this application:",
        _ => "Shiiii something went wrong here...",
    };

    loop {
        match (Text::new(message).prompt_skippable()?, updatable_field) {
            (Some(input), _) if !input.trim().is_empty() => {
                return Ok(input);
            }
            (Some(input), UpdatableField::CompanyName | UpdatableField::Title)
                if input.trim().is_empty() =>
            {
                println!("{}", "A new value is required for this field!".red().bold())
            }
            (Some(input), UpdatableField::Link | UpdatableField::Notes)
                if input.trim().is_empty() =>
            {
                return Ok("".to_string());
            }
            _ => {
                return Err(FettersError::UnknownError(
                    "An unknown error has occurred when executing the Inquire Text prompt!"
                        .to_string(),
                ));
            }
        }
    }
}

/// Set a new sprint for this application.
fn set_new_sprint(
    connection: &mut SqliteConnection,
    new_sprint_id: &mut Option<i32>,
) -> Result<(), FettersError> {
    let mut sprint_repo = SprintRepository { connection };
    let all_sprints = sprint_repo.get_all_sprints()?;

    let sprint_selection = Select::new("Select a new sprint:", all_sprints)
        .with_vim_mode(true)
        .prompt_skippable()?;

    loop {
        match sprint_selection {
            Some(queried_sprint) => {
                *new_sprint_id = Some(queried_sprint.id);
                return Ok(());
            }
            _ => println!("Unknown input. Please try again."),
        }
    }
}

/// Set a new status for this application.
fn set_new_status(
    connection: &mut SqliteConnection,
    new_status_id: &mut Option<i32>,
) -> Result<(), FettersError> {
    let mut status_repo = StatusRepository { connection };
    let all_statuses = status_repo.get_all_statuses()?;

    let status_selection = Select::new("Select a new status:", all_statuses)
        .with_vim_mode(true)
        .prompt_skippable()?;

    loop {
        match status_selection {
            Some(queried_status) => {
                *new_status_id = Some(queried_status.id);
                return Ok(());
            }
            _ => println!("Unknown input. Please try again."),
        }
    }
}

/// Set a new title for this application.
fn set_new_title(
    connection: &mut SqliteConnection,
    new_title_id: &mut Option<i32>,
) -> Result<(), FettersError> {
    let title_type = create_or_use_title(connection)?;
    let title_id = match title_type {
        TitleType::NewTitle(new_title) => {
            let mut title_repo = TitleRepository { connection };
            title_repo.add_title(NewTitle { name: &new_title })?.id
        }
        TitleType::QueriedTitle(queried_title) => queried_title.id,
    };

    *new_title_id = Some(title_id);

    Ok(())
}
