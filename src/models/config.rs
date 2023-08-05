//! Contains models pertaining to configuration settings.

use std::{collections::BTreeMap, fs::File, io::Read};

use ansi_term::{Color, Style};
use serde::{Deserialize, Serialize};

use crate::errors::FettersError;

/// Terminal chart settings for job application insights.
#[derive(Debug, Deserialize, Serialize)]
pub struct Chart {
    /// The aspect ratio to use.
    pub aspect_ratio: u16,
    /// The radius of the chart.
    pub radius: u16,
}

/// ChatGPT settings for automatic resume generation.
#[derive(Debug, Deserialize, Serialize)]
pub struct Chatgpt {
    /// The API key used to access the ChatGPT API.
    pub api_key: Option<String>,
}

/// Terminal display settings for better visual appeal.
#[derive(Debug, Deserialize, Serialize)]
pub struct Display {
    /// Terminal chart settings for job application insights.
    pub chart: Chart,
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
    /// ChatGPT settings for automatic resume generation.
    pub chatgpt: Chatgpt,
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

    /// Get the `ansi_term` `Style` for the `match_color` attribute. If the provided color value is
    /// unknown, tries to parse into `u8`. Returns `Style::default()` if all else fails.
    pub fn get_match_color_style(&self) -> Style {
        self.raw_color_to_style(&self.display.match_color)
    }

    /// Get all status mappings and the `ansi_term` `style` associated with each status. If the
    /// provided color value is unknown, tries to parse into `u8`. Returns `Style::default()` if
    /// all else fails.
    pub fn get_status_mappings_and_colors(&self) -> BTreeMap<String, Style> {
        let mut mappings_and_colors = BTreeMap::new();

        for (status, color) in self.presets.status_mappings.iter() {
            let style = self.raw_color_to_style(color);

            mappings_and_colors.insert(status.clone(), style);
        }

        mappings_and_colors
    }

    /// Convert the provided color to an `ansi_term` `Style`. If the provided color value is
    /// unknown, tries to parse into `u8`. Returns `Style::default()` if all else fails.
    fn raw_color_to_style(&self, value: &str) -> Style {
        match value {
            "black" => Color::Black.bold(),
            "blue" => Color::Blue.bold(),
            "cyan" => Color::Cyan.bold(),
            "green" => Color::Green.bold(),
            "purple" => Color::Purple.bold(),
            "red" => Color::Red.bold(),
            "white" => Color::White.bold(),
            "yellow" => Color::Yellow.bold(),
            _ => self
                .display
                .match_color
                .parse::<u8>()
                .map(|fixed_value| Color::Fixed(fixed_value).bold())
                .unwrap_or(Style::default()),
        }
    }
}

impl Default for FettersSettings {
    fn default() -> Self {
        Self {
            chatgpt: Chatgpt { api_key: None },
            display: Display {
                chart: Chart {
                    aspect_ratio: 3,
                    radius: 12,
                },
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
