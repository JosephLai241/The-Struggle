//! Contains functions called by the CLI when managing sprints.

use chrono::Local;
use diesel::SqliteConnection;
use inquire::Select;
use owo_colors::OwoColorize;

use crate::{
    config::configuration::Config,
    errors::FettersError,
    models::{NewSprint, QueriedSprint, SprintUpdate},
    repositories::sprint::SprintRepository,
    utils::display::display_sprint,
};

/// Display the current sprint and its metadata in a table.
pub fn show_current_sprint(queried_sprint: QueriedSprint) {
    display_sprint(&vec![queried_sprint], "CURRENT SPRINT");
}

/// Create a new sprint.
pub fn create_new_sprint(
    connection: &mut SqliteConnection,
    name: &Option<String>,
    config: Config,
    current_sprint: &QueriedSprint,
) -> Result<(), FettersError> {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let new_sprint_name = name.clone().unwrap_or(today.clone());

    // Throw `SprintNameConflict` if there is a naming collision between today's date and the
    // current sprint's name (defaults to date of creation).
    if new_sprint_name.eq(&current_sprint.name) {
        return Err(FettersError::SprintNameConflict(today));
    }

    let mut sprint_repo = SprintRepository { connection };

    // Update the current sprint's `end_date` field.
    sprint_repo.update_sprint(
        current_sprint.id,
        SprintUpdate {
            name: None,
            start_date: None,
            end_date: Some(&today),
        },
    )?;

    // Create a new sprint.
    let queried_sprint = sprint_repo.add_job_sprint(NewSprint {
        name: &new_sprint_name,
        start_date: &today,
        end_date: None,
        num_jobs: &0,
    })?;

    // Write the new sprint to the configuration file.
    let mut new_config = Config::from(config);
    new_config.current_sprint = queried_sprint.name.clone();
    new_config.save_to_file()?;

    display_sprint(&vec![queried_sprint], "NEW SPRINT");

    Ok(())
}

/// Display all tracked sprints.
pub fn show_all_sprints(connection: &mut SqliteConnection) -> Result<(), FettersError> {
    let mut sprint_repo = SprintRepository { connection };
    let all_sprints = sprint_repo.get_all_sprints()?;

    display_sprint(&all_sprints, "ALL SPRINTS");

    Ok(())
}

/// Set the current sprint by selecting one of the options in the `Select` menu.
pub fn set_sprint(connection: &mut SqliteConnection, config: Config) -> Result<(), FettersError> {
    let mut sprint_repo = SprintRepository { connection };
    let all_sprints = sprint_repo.get_all_sprints()?;

    let sprint_selection = Select::new("Select a new sprint:", all_sprints).prompt_skippable()?;
    loop {
        match sprint_selection {
            Some(queried_sprint) => {
                let mut new_config = Config::from(config);
                new_config.current_sprint = queried_sprint.name;

                new_config.save_to_file()?;

                println!(
                    "{}",
                    format!(
                        "Successfully set the current sprint to [{}]!",
                        &new_config.current_sprint
                    )
                    .green()
                    .bold()
                );

                return Ok(());
            }
            _ => println!("Unknown input. Please try again."),
        }
    }
}
