//! Contains a function called by the CLI when adding a new job.

use chrono::Local;
use diesel::sqlite::SqliteConnection;
use inquire::{Select, Text};
use owo_colors::OwoColorize;
use tabled::{
    Table,
    settings::{
        Alignment, Color, Style,
        location::Locator,
        object::{Columns, Rows},
        style::LineText,
    },
};

use crate::models::{NewJob, NewTitle, QueriedJob, QueriedStatus, QueriedTitle};
use crate::repositories::{job::JobRepository, statuses::StatusRepository, title::TitleRepository};
use crate::{errors::FettersError, models::QueriedSprint};

/// Run the inquire menu to track a new job application.
pub fn add_job(
    connection: &mut SqliteConnection,
    company_name: &str,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let title = create_or_use_title(connection)?;
    let status = select_status(connection)?;
    let link = input_link()?;
    let notes = input_notes()?;

    let new_job = NewJob {
        company_name,
        created: Local::now().date_naive().format("%Y-%m-%d").to_string(),
        title_id: title.id,
        status_id: status.id,
        link: link.as_deref(),
        notes: notes.as_deref(),
        sprint_id: current_sprint.id,
    };

    // TODO: ADD CONFIRMATION PROMPT HERE BEFORE WRITING ANYTHING TO SQLITE (including title,
    // modify create_new_title()).

    let mut job_repo = JobRepository { connection };
    let queried_job = job_repo.add_job(new_job)?;

    display_job(connection, &queried_job)?;

    Ok(())
}

/// Display the `Select` menu for existing job titles or create a new title.
fn create_or_use_title(connection: &mut SqliteConnection) -> Result<QueriedTitle, FettersError> {
    let mut title_repo = TitleRepository { connection };
    let existing_titles = title_repo.get_all_titles()?;

    let queried_title = if existing_titles.is_empty() {
        println!("{}", "There are currently no stored job titles!".cyan());
        create_new_title(&mut title_repo)?
    } else {
        get_existing_or_create_title(&mut title_repo, existing_titles)?
    };

    Ok(queried_title)
}

/// Create a new job title.
fn create_new_title(title_repo: &mut TitleRepository) -> Result<QueriedTitle, FettersError> {
    loop {
        match Text::new("Enter a new job title:").prompt_skippable()? {
            Some(name) if !name.trim().is_empty() => {
                return Ok(title_repo.add_title(NewTitle { name: &name })?);
            }
            Some(_) | None => println!("{}", "Please enter a title!".red().bold()),
        }
    }
}

/// Select an existing job title or create a new one.
fn get_existing_or_create_title(
    title_repo: &mut TitleRepository,
    existing_titles: Vec<QueriedTitle>,
) -> Result<QueriedTitle, FettersError> {
    let existing_or_new = Select::new(
        "Do you want to choose an existing job title or create a new one?",
        vec!["Existing", "New"],
    )
    .with_vim_mode(true)
    .prompt_skippable()?;

    if let Some(selection) = existing_or_new {
        if selection == "Existing" {
            let title_selection =
                Select::new("Select a title:", existing_titles).prompt_skippable()?;

            if let Some(title) = title_selection {
                Ok(title_repo.get_title(title.id)?)
            } else {
                Err(FettersError::UnknownError(
                    "No selection was provided.".to_string(),
                ))
            }
        } else {
            Ok(create_new_title(title_repo)?)
        }
    } else {
        Err(FettersError::UnknownError(
            "No selection was provided.".to_string(),
        ))
    }
}

/// Select a job application status.
fn select_status(connection: &mut SqliteConnection) -> Result<QueriedStatus, FettersError> {
    let mut status_repo = StatusRepository { connection };
    let all_statuses = status_repo.get_all_statuses()?;

    let selected =
        Select::new("Select a status for this application:", all_statuses).prompt_skippable()?;

    if let Some(status) = selected {
        Ok(status)
    } else {
        Err(FettersError::UnknownError(
            "No selection was provided.".to_string(),
        ))
    }
}

/// Input an optional link to the job application.
fn input_link() -> Result<Option<String>, FettersError> {
    Ok(Text::new("[OPTIONAL] Enter a link to this job application:").prompt_skippable()?)
}

/// Input optional notes for the job application.
fn input_notes() -> Result<Option<String>, FettersError> {
    Ok(Text::new("[OPTIONAL] Enter any notes for this job application:").prompt_skippable()?)
}

/// Display the attributes of the newly added job in a table.
fn display_job(connection: &mut SqliteConnection, job: &QueriedJob) -> Result<(), FettersError> {
    let mut job_repo = JobRepository { connection };
    let tabled_job = job_repo.get_job(job.id)?;

    let mut table = Table::new([tabled_job]);
    table
        .with(LineText::new("New job", Rows::first()).offset(2))
        .with(Style::rounded())
        .modify(Columns::first(), Alignment::left())
        .modify(Locator::content("GHOSTED"), Color::FG_BRIGHT_WHITE)
        .modify(Locator::content("HIRED"), Color::FG_BRIGHT_GREEN)
        .modify(Locator::content("IN PROGRESS"), Color::FG_BRIGHT_YELLOW)
        .modify(
            Locator::content("NOT HIRING ANYMORE"),
            Color::rgb_fg(201, 201, 201),
        )
        .modify(Locator::content("OFFER RECEIVED"), Color::FG_BRIGHT_MAGENTA)
        .modify(Locator::content("PENDING"), Color::FG_BRIGHT_BLUE)
        .modify(Locator::content("REJECTED"), Color::FG_BRIGHT_RED);

    println!("{table}");

    Ok(())
}
