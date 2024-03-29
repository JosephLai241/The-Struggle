//! Modifying the `job_applications.csv` spreadsheet.

use crate::model::Job;

use ansi_term::*;
use csv::{
    Reader,
    WriterBuilder
};
use serde::Serialize;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

/// This is the set filename of the spreadsheet in which job applications will be 
/// stored.
const FILENAME: &str = "job_applications.csv";

/// This struct uses Serde for easier CSV writing. 
#[derive(Serialize)]
struct Listing {
    #[serde(rename = "DATE ADDED")]
    date: String,
    #[serde(rename = "COMPANY")]
    company: String,
    #[serde(rename = "JOB TITLE")]
    title: String,
    #[serde(rename = "STATUS")]
    status: String,
    #[serde(rename = "NOTES")]
    notes: String
}

impl Listing {
    /// Serialize a job listing with the Listing struct.
    fn serialize(job: &Job) -> Listing {
        Listing {
            date: job.date.to_string(),
            company: job.company.to_string(),
            title: job.title.to_string(),
            status: job.status.to_string(),
            notes: job.notes.to_string()
        }
    }
}

/// Check if "job_applications.csv" file exists in the current directory.
fn existence() -> bool {
    Path::new(FILENAME).exists()
}

/// Write a job listing to the spreadsheet. Add a header if the file does not 
/// already exist. If the file already exists, just append the job to the 
/// spreadsheet.
pub fn write_new_job(job: &Job) -> Result<(), Box<dyn Error>> {
    let mut writer = if existence() == true { 
        let file = OpenOptions::new().append(true).open(FILENAME)?;
        WriterBuilder::new().has_headers(false).from_writer(file)
    } else {
        let file = OpenOptions::new().create(true).write(true).open(FILENAME)?;
        WriterBuilder::new().has_headers(true).from_writer(file)
    };

    writer.serialize(Listing::serialize(&job))?;

    Ok(println!("\n{}\n", Colour::Green.bold().paint("ADDED NEW LISTING.")))
}

/// Overwrite the spreadsheet after updating or deleting a job listing.
pub fn overwrite(master: &mut BTreeMap<u16, Job>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).truncate(true).open(FILENAME)?;

    let mut writer = WriterBuilder::new().has_headers(true).from_writer(file);
    for i in 0u16..master.len() as u16 {
        let job = master.get(&i).unwrap();
        writer.serialize(Listing::serialize(job))?;
    }

    Ok(println!("\n{}\n", Colour::Green.bold().paint("UPDATED SPREADSHEET.")))
}

/// Get jobs from the spreadsheet and return The BTreeMap of Job objects and its
/// index, which is its location in the spreadsheet.
pub fn get_jobs() -> Result<BTreeMap<u16, Job>, Box<dyn Error>> {
    let mut master: BTreeMap<u16, Job> = BTreeMap::new();

    let file = File::open(FILENAME)?;
    let mut read = Reader::from_reader(file);

    for (index, record) in read.records().enumerate() {
        let index = index as u16;
        let job = record?;

        let listing = Job::new_job(
            job.get(0).unwrap().to_string(), 
            job.get(1).unwrap().to_string(), 
            job.get(2).unwrap().to_string(), 
            job.get(3).unwrap().to_string(), 
            job.get(4).unwrap().to_string()
        );

        master.insert(index, listing);
    }

    Ok(master)
}
