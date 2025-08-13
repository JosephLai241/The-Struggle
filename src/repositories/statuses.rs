//! Contains the statuses repository abstraction class.

use diesel::insert_into;
use diesel::prelude::*;
use lazy_static::lazy_static;

use crate::errors::FettersError;
use crate::models::{NewStatus, QueriedStatus};

lazy_static! {
    /// Contains all default statuses that will be stored into the `statuses` SQLite table on the
    /// initial run.
    static ref DEFAULT_STATUSES: Vec<&'static str> = vec![
        "GHOSTED",
        "HIRED",
        "IN PROGRESS",
        "NOT HIRING ANYMORE",
        "OFFER RECEIVED",
        "PENDING",
        "REJECTED",
    ];
}

/// Contains all methods pertaining to CRUD operations for the `statuses` table.
pub struct StatusRepository<'a> {
    pub connection: &'a mut SqliteConnection,
}

impl<'a> StatusRepository<'a> {
    /// Retrieves all statuses.
    pub fn get_all_statuses(&mut self) -> Result<Vec<QueriedStatus>, FettersError> {
        use crate::schema::statuses::dsl::*;

        Ok(statuses
            .select(QueriedStatus::as_select())
            .load(self.connection)?)
    }

    /// Stores the default statuses into the `statuses` table if it doesn't already exist.
    pub fn seed_statuses(&mut self) -> Result<(), FettersError> {
        use crate::schema::statuses::dsl::*;

        for status in DEFAULT_STATUSES.to_vec() {
            let exists = statuses
                .filter(name.eq(status))
                .select(QueriedStatus::as_select())
                .first(self.connection)
                .optional()?;

            if let None = exists {
                let new_status = NewStatus { name: status };
                insert_into(statuses)
                    .values(&new_status)
                    .execute(self.connection)?;
            }
        }

        Ok(())
    }
}
