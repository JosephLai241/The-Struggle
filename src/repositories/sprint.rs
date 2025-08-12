//! Contains the job sprint repository abstraction class.

use chrono::Local;
use diesel::insert_into;
use diesel::insert_or_ignore_into;
use diesel::prelude::*;

use crate::errors::FettersError;
use crate::models::{NewSprint, QueriedSprint};

/// Contains all methods pertaining to CRUD operations for the `sprints` table.
pub struct SprintRepository<'a> {
    pub connection: &'a mut SqliteConnection,
}

impl<'a> SprintRepository<'a> {
    /// Adds a new job sprint into the `sprints` table.
    pub fn add_job_sprint(&mut self, new_sprint: NewSprint) -> Result<QueriedSprint, FettersError> {
        use crate::schema::sprints::dsl::*;

        Ok(insert_into(sprints)
            .values(&new_sprint)
            .returning(QueriedSprint::as_returning())
            .get_result(self.connection)?)
    }

    /// Retrieves the current sprint's ID.
    pub fn get_current_sprint(&mut self, sprint_name: &str) -> Result<QueriedSprint, FettersError> {
        use crate::schema::sprints::dsl::*;

        sprints
            .filter(name.eq(sprint_name))
            .select(QueriedSprint::as_select())
            .first::<QueriedSprint>(self.connection)
            .optional()?
            .map_or_else(
                || {
                    let new_sprint = NewSprint {
                        name: sprint_name,
                        start_date: &Local::now().date_naive().format("%Y-%m-%d").to_string(),
                        end_date: None,
                    };
                    self.add_job_sprint(new_sprint)
                },
                |sprint| Ok(sprint),
            )
    }

    /// Retrieves an existing job sprint by ID.
    pub fn get_sprint(&mut self, sprint_id: i32) -> Result<QueriedSprint, FettersError> {
        use crate::schema::sprints::dsl::*;

        Ok(sprints
            .find(sprint_id)
            .select(QueriedSprint::as_select())
            .first(self.connection)?)
    }

    /// Retrieves all job sprint titles.
    pub fn get_all_titles(&mut self) -> Result<Vec<QueriedSprint>, FettersError> {
        use crate::schema::sprints::dsl::*;

        Ok(sprints
            .select(QueriedSprint::as_select())
            .load(self.connection)?)
    }
}
