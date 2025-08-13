//! Contains all models for job sprints.

use std::fmt::{self, Display, Formatter};

use diesel::sqlite::Sqlite;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use tabled::Tabled;
use tabled::derive::display;

use crate::schema::sprints;

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
    /// The number of jobs in this sprint.
    pub num_jobs: &'a i32,
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
    /// The number of jobs in this sprint.
    #[tabled(rename = "# of Jobs")]
    pub num_jobs: i32,
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
