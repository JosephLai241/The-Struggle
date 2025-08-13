//! Contains all models used for `fetters`.

use std::fmt::{self, Display, Formatter};

use diesel::sqlite::Sqlite;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use owo_colors::OwoColorize;
use tabled::Tabled;
use tabled::derive::display;

use crate::schema::{jobs, sprints, statuses, titles};

/// This struct defines the job object returned from querying SQLite.
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = jobs)]
#[diesel(check_for_backend(Sqlite))]
pub struct QueriedJob {
    /// The SQLite ID.
    pub id: i32,
    /// The timestamp at which this job application was created.
    pub created: String,
    /// The name of the company.
    pub company_name: String,
    /// The job title.
    pub title_id: i32,
    /// The application status.
    pub status_id: i32,
    /// The link to the job application.
    pub link: Option<String>,
    /// Any notes about this job application.
    pub notes: Option<String>,
    /// The sprint ID. References the record ID in SQLite.
    pub sprint_id: i32,
}

/// This struct defines a new job application that will be inserted into SQLite.
#[derive(Debug, Insertable)]
#[diesel(table_name = jobs)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewJob<'a> {
    /// The name of the company.
    pub company_name: &'a str,
    /// The timestamp at which this job application was created.
    pub created: String,
    /// The job title ID. References the record ID in SQLite.
    pub title_id: i32,
    /// The application status ID. References the record ID in SQLite.
    pub status_id: i32,
    /// The link to the job application.
    pub link: Option<&'a str>,
    /// Any notes about this job application.
    pub notes: Option<&'a str>,
    /// The sprint ID. References the record ID in SQLite.
    pub sprint_id: i32,
}

/// This struct defines an updated job application that will overwrite an existing one in SQLite.
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = jobs)]
#[diesel(check_for_backend(Sqlite))]
pub struct JobUpdate<'a> {
    /// The name of the company.
    pub company_name: Option<&'a str>,
    /// The job title ID. References the record ID in SQLite.
    pub title_id: Option<i32>,
    /// The application status ID. References the record ID in SQLite.
    pub status_id: Option<i32>,
    /// The link to the job application.
    pub link: Option<&'a str>,
    /// Any notes about this job application.
    pub notes: Option<&'a str>,
    /// The sprint ID. References the record ID in SQLite.
    pub sprint_id: Option<i32>,
}

/// This struct defines a job application with the title, status, and sprint name after querying
/// SQLite for those fields based on their record IDs and is used when displaying job applications
/// in tables.
#[derive(Clone, Debug, Queryable, Tabled)]
pub struct TabledJob {
    /// The SQLite ID.
    #[tabled(rename = "ID")]
    pub id: i32,
    /// The timestamp at which this job application was created.
    #[tabled(rename = "Created")]
    pub created: String,
    /// The name of the company.
    #[tabled(rename = "Company Name")]
    pub company_name: String,
    /// The job title.
    #[tabled(rename = "Title")]
    #[tabled(display("display::option", "N/A"))]
    pub title: Option<String>,
    /// The application status.
    #[tabled(rename = "Status")]
    #[tabled(display("display::option", "N/A"))]
    pub status: Option<String>,
    /// The link to the job application.
    #[tabled(rename = "Link")]
    #[tabled(display("display::option", "N/A"))]
    pub link: Option<String>,
    /// Any notes about this job application.
    #[tabled(rename = "Notes")]
    #[tabled(display("display::option", "N/A"))]
    pub notes: Option<String>,
}

impl TabledJob {
    /// Colorize a string based on the `status` field of the job application.
    fn colorize_field(&self, field_name: &str) -> String {
        if let Some(ref status) = self.status {
            match status {
                val if val == "GHOSTED" => {
                    return field_name.white().bold().to_string();
                }
                val if val == "HIRED" => return field_name.green().bold().to_string(),
                val if val == "IN PROGRESS" => return field_name.yellow().bold().to_string(),
                val if val == "NOT HIRING ANYMORE" => {
                    return field_name.fg_rgb::<201, 201, 201>().to_string();
                }
                val if val == "OFFER RECEIVED" => return field_name.magenta().bold().to_string(),
                val if val == "PENDING" => return field_name.blue().bold().to_string(),
                val if val == "REJECTED" => return field_name.red().bold().to_string(),
                _ => return field_name.to_string(),
            }
        }

        return field_name.to_string();
    }
}

impl Display for TabledJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {} | Company: {} | Title: {} | Status: {}",
            self.id.white().bold(),
            self.colorize_field(&self.company_name),
            self.colorize_field(&self.title.clone().unwrap_or("".to_string())),
            self.colorize_field(&self.status.clone().unwrap_or("".to_string()))
        )
    }
}

/// This struct defines a new job title that will be written to the `titles` table in SQLite.
#[derive(Debug, Insertable)]
#[diesel(table_name = titles)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewTitle<'a> {
    /// The job title.
    pub name: &'a str,
}

/// This struct defines the title object returned from querying SQLite.
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = titles)]
#[diesel(check_for_backend(Sqlite))]
pub struct QueriedTitle {
    /// The SQLite ID.
    pub id: i32,
    /// The job title.
    pub name: String,
}

/// Implementing `Display` allows this struct to be displayed in the `Select` Inquire menu.
impl Display for QueriedTitle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// This struct defines a new sprint title that will be written to the `sprints` table in SQLite.
#[derive(Debug, Insertable)]
#[diesel(table_name = sprints)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewSprint<'a> {
    /// The sprint title.
    pub name: &'a str,
    /// The start date for this sprint.
    pub start_date: &'a str,
    /// The end date for this sprint.
    pub end_date: Option<&'a str>,
}

/// This struct defines the sprint object returned from querying SQLite.
#[derive(Debug, Queryable, Selectable, Tabled)]
#[diesel(table_name = sprints)]
#[diesel(check_for_backend(Sqlite))]
pub struct QueriedSprint {
    /// The SQLite ID.
    #[tabled(skip)]
    pub id: i32,
    /// The sprint title.
    #[tabled(rename = "Sprint Name")]
    pub name: String,
    /// The start date for this sprint.
    #[tabled(rename = "Start Date")]
    pub start_date: String,
    /// The end date for this sprint.
    #[tabled(rename = "End Date")]
    #[tabled(display("display::option", "N/A"))]
    pub end_date: Option<String>,
}

impl Display for QueriedSprint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Start Date: {}, End Date: {:?})",
            self.name, self.start_date, self.end_date
        )
    }
}

/// This struct defines an updated sprint that will overwrite an existing one in SQLite.
#[derive(Debug, Default, AsChangeset)]
#[diesel(table_name = sprints)]
#[diesel(check_for_backend(Sqlite))]
pub struct SprintUpdate<'a> {
    /// The sprint title.
    pub name: Option<&'a str>,
    /// The start date for this sprint.
    pub start_date: Option<&'a str>,
    /// The end date for this sprint.
    pub end_date: Option<&'a str>,
}

/// This struct defines a new status that will be written to the `sprints` table in SQLite.
#[derive(Debug, Insertable)]
#[diesel(table_name = statuses)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewStatus<'a> {
    /// The status name.
    pub name: &'a str,
}

/// This struct defines the status object returned from querying SQLite.
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = statuses)]
#[diesel(check_for_backend(Sqlite))]
pub struct QueriedStatus {
    /// The SQLite ID.
    pub id: i32,
    /// The status title.
    pub name: String,
}

/// Implementing `Display` allows this struct to be displayed in the `Select` Inquire menu.
impl Display for QueriedStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
