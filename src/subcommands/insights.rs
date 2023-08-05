//! Contains all functionality pertaining to the `insights` subcommand.

use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use ansi_term::{Color, Style};
use chatgpt::prelude::ChatGPT;
use chrono::{DateTime, Local};
use diesel::SqliteConnection;
use lazy_static::lazy_static;
use piechart::{Chart, Data};
use rand::Rng;
use termimad::{rgb, FmtText, MadSkin};

use crate::{
    errors::FettersError,
    models::{config::FettersSettings, job::Job, stint::Stint},
    sqlite,
};

lazy_static! {
    /// The `MadSkin` that is applied to the Markdown resume.
    static ref MADSKIN: MadSkin = {
        let mut skin = MadSkin::default();
        skin.set_headers_fg(rgb(183, 65, 14));

        skin
    };
}

/// Display job application insights alongside a pie chart.
pub async fn display_insights(
    connection: &mut SqliteConnection,
    date_range: Option<String>,
    fetters_settings: &FettersSettings,
    stint: Option<String>,
) -> Result<(), FettersError> {
    let mut all_jobs = sqlite::queries::get_all_jobs(connection)?;
    let total_num_jobs = all_jobs.len();

    if let Some(date_range) = date_range {
        filter_by_date_range(&mut all_jobs, date_range)?;
    }
    if let Some(stint) = stint {
        let all_stints = sqlite::queries::get_all_stints(connection)?;
        let mapped_stints = all_stints
            .into_iter()
            .fold(HashMap::new(), |mut hashmap, stint| {
                hashmap.insert(stint.id, stint);
                hashmap
            });

        filter_by_stint(&mut all_jobs, mapped_stints, stint);
    }

    let status_mappings = fetters_settings.get_status_mappings_and_colors();
    let status_counts = count_indiscriminately(&all_jobs);

    display_piechart(
        fetters_settings,
        status_counts,
        status_mappings,
        total_num_jobs,
    );

    if let Some(api_key) = &fetters_settings.chatgpt.api_key {
        let summary = get_chatgpt_summary(&all_jobs, api_key).await?;
        let formatted_markdown = FmtText::from(&MADSKIN, &summary, Some(100));

        println!("\n{formatted_markdown}");
    }

    Ok(())
}

/// Filter jobs based on time range.
fn filter_by_date_range(all_jobs: &mut Vec<Job>, date_range: String) -> Result<(), FettersError> {
    let now = Local::now().format("YYYY/MM/DD HH:MM:SS").to_string();
    let ranges: Vec<&str> = date_range.split(',').collect();

    let raw_lower_limit =
        ranges
            .first()
            .map(|date| date.to_string())
            .ok_or(FettersError::GenericError(
                "Failed to get the lower date range! Cannot filter by date range.".to_string(),
            ))?;
    let raw_upper_limit = if ranges.len() >= 2 {
        ranges.last().map(|date| date.to_string()).unwrap_or(now)
    } else {
        now
    };

    let lower_limit: DateTime<Local> = DateTime::from_str(&raw_lower_limit)?;
    let upper_limit: DateTime<Local> = DateTime::from_str(&raw_upper_limit)?;

    all_jobs.retain(|job| {
        let date_added: DateTime<Local> = DateTime::from_str(&job.date_added)
            .expect("Failed to convert the job's date_added field to DateTime! Cannot filter by date range.");

        date_added >= lower_limit && date_added <= upper_limit
    });

    Ok(())
}

/// Filter jobs based on the stint name.
fn filter_by_stint(
    all_jobs: &mut Vec<Job>,
    mapped_stints: HashMap<Option<i32>, Stint>,
    stint: String,
) {
    all_jobs.retain(|job| {
        let job_stint = mapped_stints
            .get(&job.stint)
            .map(|stint| stint.stint.clone())
            .expect("Failed to convert the job's stint ID to name! Cannot filter by stint.");

        job_stint == stint
    });
}

/// Calculate the total number of job applications for all status.
fn count_indiscriminately(all_jobs: &[Job]) -> BTreeMap<String, i32> {
    let mut status_counts: BTreeMap<String, i32> = BTreeMap::new();

    for job in all_jobs.iter() {
        if let Some(count) = status_counts.get_mut(&job.status) {
            *count += 1;
        } else {
            status_counts.insert(job.status.clone(), 1);
        }
    }

    status_counts
}

/// Create a piechart that visually represents the ratios of each job application category.
fn display_piechart(
    fetters_settings: &FettersSettings,
    status_counts: BTreeMap<String, i32>,
    status_mappings: BTreeMap<String, Style>,
    total_num_jobs: usize,
) {
    let mut piechart_data: Vec<Data> = Vec::new();
    for (status, count) in status_counts.iter() {
        let value = *count as f32 / total_num_jobs as f32;

        let (color, fill) = if let Some(style) = status_mappings.get(status) {
            (Some(*style), 'o')
        } else {
            let mut rng = rand::thread_rng();
            let random_color_value = rng.gen_range(0..=255);

            (Some(Color::Fixed(random_color_value).bold()), '?')
        };

        let data = Data {
            color,
            fill,
            label: status.into(),
            value,
        };

        piechart_data.push(data);
    }

    Chart::new()
        .radius(fetters_settings.display.chart.radius)
        .aspect_ratio(fetters_settings.display.chart.aspect_ratio)
        .legend(true)
        .draw(&piechart_data);
}

/// Ask ChatGPT to give a summary for the job application data.
async fn get_chatgpt_summary(all_jobs: &[Job], api_key: &str) -> Result<String, FettersError> {
    let chatgpt_client = ChatGPT::new(api_key)?;

    let message = format!(
        "Given the following data that was stored in a SQLite database, summarize the following job application data and provide your summary in Markdown format: {:?}", 
        all_jobs
    );

    let summary = &chatgpt_client
        .send_message(message)
        .await?
        .message()
        .content
        .clone();

    Ok(summary.to_string())
}
