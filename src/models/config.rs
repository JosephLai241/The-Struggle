//! Contains models pertaining to configuration settings.

use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::errors::FettersError;

/// Terminal display settings for better visual appeal.
#[derive(Debug, Deserialize, Serialize)]
pub struct Display {
    /// The max column width of tables.
    pub max_column_width: usize,
}

/// Preset settings for faster job application logging.
#[derive(Debug, Deserialize, Serialize)]
pub struct Presets {
    /// The different status that may be applied to a job application.
    pub status: Vec<String>,
    /// Pre-defined job titles to select from.
    pub titles: Vec<String>,
}

/// Contains all settings for `fetters`.
#[derive(Debug, Deserialize, Serialize)]
pub struct FettersSettings {
    /// Terminal display settings for better visual appeal.
    pub display: Display,
    /// Preset settings for faster job application logging.
    pub presets: Presets,
}

impl FettersSettings {
    /// Create a new instance of settings for `fetters` from a given configuration path.
    pub fn read_from(path: String) -> Result<Self, FettersError> {
        Ok(Config::builder()
            .add_source(File::with_name(&path))
            .build()?
            .try_deserialize()?)
    }
}
