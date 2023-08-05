//! Contains utilities for configuring `fetters`.

use std::fs;

use directories::ProjectDirs;
use lazy_static::lazy_static;

use crate::{errors::FettersError, models::config::FettersSettings};

lazy_static! {
    /// The default TOML configuration file.
    static ref TOML_CONFIG: &'static [u8; 339] = include_bytes!("../../fetters.toml");
}

/// Check if the project directories and configuration file are set up on the user's machine.
/// Creates a new configuration if it does not already exist (first time setup), otherwise returns
/// the existing configuration from the system.
pub fn configure_fetters() -> Result<FettersSettings, FettersError> {
    match ProjectDirs::from("", "", "fetters") {
        Some(project_directory) => {
            let config_path = project_directory.config_dir().join("fetters.toml");

            if !config_path.exists() {
                match &config_path.parent() {
                    Some(parent) => fs::create_dir_all(parent)?,
                    None => {
                        return Err(FettersError::PathError(
                            "Failed to retrieve fetters application directory path!".to_string(),
                        ))
                    }
                }

                fs::write(&config_path, *TOML_CONFIG)?;
            }

            Ok(FettersSettings::read_from(
                config_path.to_string_lossy().to_string(),
            )?)
        }
        None => Err(FettersError::ApplicationError),
    }
}

/// Add a new job status to the TOML configuration file.
pub fn add_new_job_status(new_status: &str, new_color: &str) -> Result<(), FettersError> {
    match ProjectDirs::from("", "", "fetters") {
        Some(project_directory) => {
            let config_path = project_directory.config_dir().join("fetters.toml");

            let mut config = FettersSettings::read_from(config_path.to_string_lossy().to_string())?;

            let fixed_status = new_status.replace(' ', "-").to_lowercase();
            let fixed_color = new_color.to_lowercase();

            config
                .presets
                .status_mappings
                .insert(fixed_status, fixed_color);

            let toml_string = toml::to_string_pretty(&config)?;
            fs::write(&config_path, toml_string)?;

            Ok(())
        }
        None => Err(FettersError::ApplicationError),
    }
}

/// Add a new job title to the TOML configuration file.
pub fn add_new_job_title(new_title: &str) -> Result<(), FettersError> {
    match ProjectDirs::from("", "", "fetters") {
        Some(project_directory) => {
            let config_path = project_directory.config_dir().join("fetters.toml");

            let mut config = FettersSettings::read_from(config_path.to_string_lossy().to_string())?;
            config.presets.titles.push(new_title.to_string());

            let toml_string = toml::to_string_pretty(&config)?;
            fs::write(&config_path, toml_string)?;

            Ok(())
        }
        None => Err(FettersError::ApplicationError),
    }
}
