//! Contains all models pertaining to job listings.

use chrono::Local;
use serde::{Deserialize, Serialize};

/// Contains all attributes associated with a job application.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Job {
    /// The ID of this record in SQLite.
    pub id: Option<i32>,
    /// The name of the company.
    pub company: String,
    /// The date on which this job listing was added.
    pub date_added: String,
    /// The link to the job listing.
    pub link: Option<String>,
    /// Any notes pertaining to the job listing.
    pub notes: Option<String>,
    /// The job application's status.
    pub status: String,
    /// The stint ID associated with this job application.
    pub stint: Option<i32>,
    /// The title of the role.
    pub title: String,
}

impl Job {
    /// Create a new `Job` instance.
    pub fn new(
        company: String,
        link: Option<String>,
        notes: Option<String>,
        status: String,
        stint: Option<i32>,
        title: String,
    ) -> Self {
        Self {
            id: None,
            company,
            date_added: Local::now().format("%Y/%m/%d %H:%M:%S").to_string(),
            link,
            notes,
            status,
            stint,
            title,
        }
    }
}
