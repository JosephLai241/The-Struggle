//! Contains all models pertaining to stints (application phases).

use chrono::Local;
use diesel::{sqlite::Sqlite, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::stints;

/// Contains all attributes associated with a stint.
#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Selectable, Serialize)]
#[diesel(table_name = stints)]
#[diesel(check_for_backend(Sqlite))]
pub struct Stint {
    /// The ID of this record in SQLite.
    pub id: Option<i32>,
    /// The date the stint was added/created.
    pub date_added: String,
    /// The name of the stint.
    pub stint: String,
}

impl Stint {
    /// Create a new `Stint` instance.
    pub fn new(stint: String) -> Self {
        Self {
            id: None,
            date_added: Local::now().format("%Y/%m/%d %H:%M:%S").to_string(),
            stint,
        }
    }
}
