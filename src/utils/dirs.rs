//! Contains utility functions for dealing with system directories.

use std::path::PathBuf;

use directories::ProjectDirs;

use crate::errors::FettersError;

/// Get the project data directory path.
pub fn get_data_dir_path() -> Result<PathBuf, FettersError> {
    if let Some(ref data_directory) = ProjectDirs::from("", "", "fetters") {
        return Ok(data_directory.data_dir().to_owned());
    }

    Err(FettersError::ApplicationError)
}
