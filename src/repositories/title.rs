//! Contains the title repository abstraction class.

use diesel::insert_into;
use diesel::prelude::*;

use crate::errors::FettersError;
use crate::models::title::{NewTitle, QueriedTitle};

/// Contains all methods pertaining to CRUD operations for the `titles` table.
pub struct TitleRepository<'a> {
    pub connection: &'a mut SqliteConnection,
}

impl<'a> TitleRepository<'a> {
    /// Adds a new job title into the `titles` table.
    /// TODO: UPDATE TO IGNORE IF ALREADY EXISTS. ALSO STANDARDIZE CAPITALIZATION?
    pub fn add_title(&mut self, new_title: NewTitle) -> Result<QueriedTitle, FettersError> {
        use crate::schema::titles::dsl::*;

        Ok(insert_into(titles)
            .values(&new_title)
            .returning(QueriedTitle::as_returning())
            .get_result(self.connection)?)
    }

    /// Retrieves an existing job title by ID.
    pub fn get_title(&mut self, title_id: i32) -> Result<QueriedTitle, FettersError> {
        use crate::schema::titles::dsl::*;

        Ok(titles
            .find(title_id)
            .select(QueriedTitle::as_select())
            .first(self.connection)?)
    }

    /// Retrieves all job titles.
    pub fn get_all_titles(&mut self) -> Result<Vec<QueriedTitle>, FettersError> {
        use crate::schema::titles::dsl::*;

        Ok(titles
            .select(QueriedTitle::as_select())
            .load(self.connection)?)
    }
}
