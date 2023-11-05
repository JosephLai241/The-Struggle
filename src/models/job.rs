//! Contains all models pertaining to job listings.

use chrono::Local;
use diesel::{query_builder::AsChangeset, sqlite::Sqlite, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::job_data;

/// Contains all attributes associated with a job application.
#[derive(
    AsChangeset,
    Clone,
    Debug,
    Deserialize,
    Eq,
    Insertable,
    Ord,
    PartialEq,
    PartialOrd,
    Queryable,
    Selectable,
    Serialize,
)]
#[diesel(table_name = job_data)]
#[diesel(check_for_backend(Sqlite))]
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

/// Contains fields that may be updatable via the `update` subcommand.
#[derive(Debug)]
pub struct UpdateJob {
    /// The new company name.
    pub company: Option<String>,
    /// The new link associated with a job listing.
    pub link: Option<String>,
    /// New notes associated with a job listing.
    pub notes: Option<String>,
    /// The new job listing status.
    pub status: Option<String>,
    /// The new stint name.
    pub stint: Option<String>,
    /// The new job title.
    pub title: Option<String>,
}
