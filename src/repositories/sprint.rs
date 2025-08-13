//! Contains the job sprint repository abstraction class.

use chrono::Local;
use diesel::dsl::update;
use diesel::insert_into;
use diesel::prelude::*;

use crate::errors::FettersError;
use crate::models::sprint::{NewSprint, QueriedSprint, SprintUpdate};
use crate::schema::sprints;

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
                        num_jobs: &0,
                    };
                    self.add_job_sprint(new_sprint)
                },
                |sprint| Ok(sprint),
            )
    }

    /// Update an existing sprint with new changes.
    pub fn update_sprint(
        &mut self,
        sprint_id: i32,
        changes: SprintUpdate,
    ) -> Result<QueriedSprint, FettersError> {
        use crate::schema::sprints::dsl::*;

        Ok(update(sprints.find(sprint_id))
            .set(&changes)
            .returning(QueriedSprint::as_returning())
            .get_result(self.connection)?)
    }

    /// Retrieves all job sprints.
    pub fn get_all_sprints(&mut self) -> Result<Vec<QueriedSprint>, FettersError> {
        use crate::schema::sprints::dsl::*;

        Ok(sprints
            .select(QueriedSprint::as_select())
            .load(self.connection)?)
    }

    /// Increment the `num_jobs` count for a particular sprint.
    pub fn increment_num_jobs(&mut self, sprint_id: i32) -> Result<(), FettersError> {
        update(sprints::table.filter(sprints::id.eq(sprint_id)))
            .set(sprints::num_jobs.eq(sprints::num_jobs + 1))
            .execute(self.connection)?;

        Ok(())
    }

    /// Decrement the `num_jobs` count for a particular sprint.
    pub fn decrement_num_jobs(&mut self, sprint_id: i32) -> Result<(), FettersError> {
        update(sprints::table.filter(sprints::id.eq(sprint_id)))
            .set(sprints::num_jobs.eq(sprints::num_jobs - 1))
            .execute(self.connection)?;

        Ok(())
    }
}
