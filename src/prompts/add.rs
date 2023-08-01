//! Contains functions pertaining to adding a new job application.

use inquire::{Select, Text};
use rusqlite::Connection;

use crate::{
    errors::FettersError,
    models::config::FettersSettings,
    sqlite::{self, queries},
    utils,
};

/// Run the interactive prompt to set the job title.
pub fn add_job_title(fetters_settings: &FettersSettings) -> Result<String, FettersError> {
    let action = Select::new(
        "Select a job title option:",
        vec!["Select from preset titles", "Create new title"],
    )
    .with_vim_mode(true)
    .prompt()?;

    if action == "Select from preset titles" && !fetters_settings.presets.titles.is_empty() {
        Ok(Select::new(
            "Select a job title:",
            fetters_settings.presets.titles.clone(),
        )
        .with_vim_mode(true)
        .prompt()?)
    } else {
        let new_title = Text::new("Enter a new job title:")
            .with_help_message("This job title will be saved to your configuration")
            .prompt()?;

        utils::config::add_new_job_title(&new_title)?;

        Ok(new_title)
    }
}

/// Run the interactive prompt to set the job status.
pub fn add_job_status(fetters_settings: &FettersSettings) -> Result<String, FettersError> {
    let action = Select::new(
        "Select a job status option:",
        vec!["Select from preset status", "Create new status"],
    )
    .with_vim_mode(true)
    .prompt()?;

    if action == "Select from preset status" && !fetters_settings.presets.status_mappings.is_empty()
    {
        Ok(Select::new(
            "Select a status:",
            fetters_settings
                .presets
                .status_mappings
                .clone()
                .into_keys()
                .collect(),
        )
        .with_vim_mode(true)
        .prompt()?)
    } else {
        let new_status = Text::new("Enter a new job status:")
            .with_help_message("This status will be saved to your configuration")
            .prompt()?;
        let new_color = Text::new("Enter a color for this job status:").with_help_message("Refer to https://docs.rs/ansi_term/latest/ansi_term/enum.Color.html#variants for all options").prompt()?;

        utils::config::add_new_job_status(&new_status, &new_color)?;

        Ok(new_status)
    }
}

/// Run the interactive prompt to add a stint to the job application.
pub fn add_job_stint(connection: &Connection) -> Result<Option<i32>, FettersError> {
    let stints = queries::get_all_stints(connection)?;

    let stint_id = if stints.is_empty() {
        create_new_job_stint(connection)?
    } else {
        let action = Select::new(
            "Select a stint option:",
            vec!["Add to existing stint", "Create new stint"],
        )
        .with_vim_mode(true)
        .with_help_message("OPTIONAL. Press <ESC> to skip.")
        .prompt_skippable()?;

        if let Some(action) = action {
            if action == "Add to existing stint" {
                let stint_label_selection = Select::new(
                    "Select an existing stint:",
                    stints
                        .clone()
                        .into_iter()
                        .map(|stint| stint.stint)
                        .collect(),
                )
                .with_help_message("OPTIONAL. Press <ESC> to skip.")
                .with_vim_mode(true)
                .prompt_skippable()?;

                if let Some(selection) = stint_label_selection {
                    stints
                        .iter()
                        .find(|stint| stint.stint == selection)
                        .map(|stint| stint.id.unwrap_or(0))
                } else {
                    None
                }
            } else {
                create_new_job_stint(connection)?
            }
        } else {
            None
        }
    };

    Ok(stint_id)
}

/// Create a new job stint.
fn create_new_job_stint(connection: &Connection) -> Result<Option<i32>, FettersError> {
    let stint_input = Text::new("Enter a name for the new stint:")
        .with_help_message("OPTIONAL. Press <ESC> to skip.")
        .prompt_skippable()?;

    if let Some(new_stint) = stint_input {
        sqlite::queries::write_stint(connection, new_stint.clone())?;

        let stint_id = sqlite::queries::get_all_stints(connection)?
            .iter()
            .find(|stint| stint.stint == new_stint)
            .map(|stint| stint.id.unwrap_or(0));

        Ok(stint_id)
    } else {
        Ok(None)
    }
}

/// Run the interactive prompt to add notes to the job application.
pub fn add_job_notes() -> Result<Option<String>, FettersError> {
    Ok(Text::new("Enter notes for this job application:")
        .with_help_message("OPTIONAL. Press <ESC> to skip.")
        .prompt_skippable()?)
}

/// Run the interactive prompt to add a link pointing to the job application.
pub fn add_job_link() -> Result<Option<String>, FettersError> {
    Ok(Text::new("Paste the link to this job application:")
        .with_help_message("OPTIONAL. Press <ESC> to skip.")
        .prompt_skippable()?)
}
