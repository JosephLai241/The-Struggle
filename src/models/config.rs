//! Contains models pertaining to configuration settings.

use std::{collections::BTreeMap, fs::File, io::Read};

use serde::{Deserialize, Serialize};

use crate::errors::FettersError;

/// Terminal display settings for better visual appeal.
#[derive(Debug, Deserialize, Serialize)]
pub struct Display {
    /// The color to use for pattern matches.
    pub match_color: String,
    /// The max column width of tables.
    pub max_column_width: usize,
}

/// Preset settings for faster job application logging.
#[derive(Debug, Deserialize, Serialize)]
pub struct Presets {
    /// Contains all job status mapped to a specific color.
    pub status_mappings: BTreeMap<String, String>,
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
        let mut toml_file = File::open(path)?;
        let mut contents = String::new();
        toml_file.read_to_string(&mut contents)?;

        Ok(toml::from_str(&contents).unwrap_or(Self::default()))
    }
}

impl Default for FettersSettings {
    fn default() -> Self {
        Self {
            display: Display {
                match_color: "red".to_string(),
                max_column_width: 45,
            },
            presets: Presets {
                status_mappings: vec![
                    ("hired".to_string(), "green".to_string()),
                    ("in-progress".to_string(), "yellow".to_string()),
                    ("offer-received".to_string(), "cyan".to_string()),
                    ("pending".to_string(), "blue".to_string()),
                    ("rejected".to_string(), "red".to_string()),
                ]
                .into_iter()
                .collect(),
                titles: vec!["Software Engineer".to_string()],
            },
        }
    }
}
