//! Contains functions pertaining to querying the SQLite instance.

use rusqlite::{params, Connection, Error};

use crate::{
    errors::FettersError,
    models::{job::Job, stint::Stint},
};

/// Retrieve all stored jobs from SQLite.
pub fn get_all_jobs(connection: &Connection) -> Result<Vec<Job>, FettersError> {
    let mut statement = connection.prepare("SELECT * FROM job_data")?;

    let jobs = statement
        .query_map(params![], |row| {
            Ok(Job {
                id: Some(row.get(0)?),
                company: row.get(1)?,
                date_added: row.get(2)?,
                link: row.get(3)?,
                notes: row.get(4)?,
                status: row.get(5)?,
                stint: row.get(6)?,
                title: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<Job>, Error>>()?;

    Ok(jobs)
}

/// Retrieve all stored stints from SQLite.
pub fn get_all_stints(connection: &Connection) -> Result<Vec<Stint>, FettersError> {
    let mut statement = connection.prepare("SELECT * FROM stints")?;

    let stints = statement
        .query_map(params![], |row| {
            Ok(Stint {
                id: Some(row.get(0)?),
                date_added: row.get(1)?,
                stint: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<Stint>, Error>>()?;

    Ok(stints)
}

/// Write a new job to the SQLite instance.
pub fn write_job(connection: &Connection, job: Job) -> Result<(), FettersError> {
    let attributes = "(company, date_added, link, notes, status, stint, title)";
    let value_parameters = "(?1, ?2, ?3, ?4, ?5, ?6, ?7)";
    let values = (
        &job.company,
        &job.date_added,
        &job.link,
        &job.notes,
        &job.status,
        &job.stint,
        &job.title,
    );

    connection.execute(
        &format!("INSERT INTO job_data {attributes} VALUES {value_parameters}"),
        values,
    )?;

    Ok(())
}

/// Write a new stint to the SQLite instance.
pub fn write_stint(connection: &Connection, stint: String) -> Result<(), FettersError> {
    let stint = Stint::new(stint);

    let attributes = "(date_added, stint)";
    let value_parameters = "(?1, ?2)";
    let values = (&stint.date_added, &stint.stint);

    connection.execute(
        &format!("INSERT INTO stints {attributes} VALUES {value_parameters}"),
        values,
    )?;

    Ok(())
}
