//! Contains utility functions for creating a new title or selecting an existing one.

use diesel::SqliteConnection;
use inquire::{Select, Text};
use owo_colors::OwoColorize;

use crate::{errors::FettersError, models::QueriedTitle, repositories::title::TitleRepository};

/// Contains all variants that may be returned from the create_or_use_title() function.
pub enum TitleType {
    /// The user has created a new title.
    NewTitle(String),
    /// The user has selected an existing title.
    QueriedTitle(QueriedTitle),
}

/// Display the `Select` menu for existing job titles or create a new title.
pub fn create_or_use_title(connection: &mut SqliteConnection) -> Result<TitleType, FettersError> {
    let mut title_repo = TitleRepository { connection };
    let existing_titles = title_repo.get_all_titles()?;

    let queried_title = if existing_titles.is_empty() {
        println!(
            "{}",
            "There are currently no stored job titles!".yellow().bold()
        );
        create_new_title()?
    } else {
        get_existing_or_create_title(&mut title_repo, existing_titles)?
    };

    Ok(queried_title)
}

/// Create a new job title.
fn create_new_title() -> Result<TitleType, FettersError> {
    loop {
        match Text::new("Enter a new job title:").prompt_skippable()? {
            Some(name) if !name.trim().is_empty() => {
                return Ok(TitleType::NewTitle(name));
            }
            Some(_) | None => println!("{}", "Please enter a title!".red().bold()),
        }
    }
}

/// Select an existing job title or create a new one.
fn get_existing_or_create_title(
    title_repo: &mut TitleRepository,
    existing_titles: Vec<QueriedTitle>,
) -> Result<TitleType, FettersError> {
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
                Ok(TitleType::QueriedTitle(title_repo.get_title(title.id)?))
            } else {
                Err(FettersError::UnknownError(
                    "No selection was provided.".to_string(),
                ))
            }
        } else {
            Ok(create_new_title()?)
        }
    } else {
        Err(FettersError::UnknownError(
            "No selection was provided.".to_string(),
        ))
    }
}
