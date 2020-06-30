use crate::model::Job;

use ansi_term::*;
use csv::{Reader, WriterBuilder};
use serde::Serialize;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::string::String;

/// This is the set filename of the spreadsheet in which tracked job applications 
/// will be stored.
const FILENAME: &str = "job_applications.csv";

/// This struct uses the crate Serde for easier CSV writing. 
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

/// Implementations for the Listing struct. These implementations make writing to
/// the spreadsheet easier.
impl Listing {
    /// Serialize job with the Listing struct for spreadsheet overwriting. This 
    /// is used when updating or deleting a job from the spreadsheet.
    fn serialize_ow(i: u16, master: &BTreeMap<u16, Job>) -> Listing {
        Listing {
            date: master.get(&i).unwrap().date.to_string(),
            company: master.get(&i).unwrap().company.to_string(),
            title: master.get(&i).unwrap().title.to_string(),
            status: master.get(&i).unwrap().status.to_string(),
            notes: master.get(&i).unwrap().notes.to_string()
        }
    }

    /// Serialize job with the Listing struct for spreadsheet writing. This is
    /// used when adding a job to the spreadsheet.
    fn serialize_a(job: &Job) -> Listing {
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

/// Create new spreadsheet and add headers and job listing.
fn create(file: File, job: &Job) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);
    
    writer.serialize(Listing::serialize_a(&job))?;

    Ok(())
}

/// Append the new job listing to the spreadsheet.
fn append(file: File, job: &Job) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_writer(file);

    writer.serialize((
        job.date.to_string(), 
        job.company.to_string(), 
        job.title.to_string(), 
        job.status.to_string(), 
        job.notes.to_string()
    ))?;

    Ok(())
}

/// Write jobs to the spreadsheet. Add a header if the file does not already exist.
/// If the file already exists, just append the job to the spreadsheet.
pub fn write_new_job(job: &Job) -> Result<(), Box<dyn Error>> {
    
    if existence() == true {
        let file = OpenOptions::new()
            .append(true)
            .open(FILENAME)?;

        append(file, &job)?;
    } else {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(FILENAME)?;

        create(file, &job)?;
    };

    Ok(println!(
        "\n{}\n", 
        Colour::Green.bold().paint("ADDED NEW LISTING.")
    ))
}

/// Overwrite the spreadsheet after updating or deleting a job listing.
pub fn overwrite(master: BTreeMap<u16, Job>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).open(FILENAME)?;

    let mut writer = WriterBuilder::new().has_headers(true).from_writer(file);
    for i in 0u16..master.len() as u16 {
        writer.serialize(Listing::serialize_ow(i, &master))?;
    }

    Ok(println!(
        "\n{}\n", 
        Colour::Green.bold().paint("UPDATED SPREADSHEET.")
    ))
}

/// Get jobs from the spreadsheet and return The BTreeMap of Job objects and its
/// index, which is its location in the spreadsheet. Throws an error if there are
/// problems parsing the spreadsheet.
pub fn get_jobs() -> Result<BTreeMap<u16, Job>, Box<dyn Error>> {
    let mut master: BTreeMap<u16, Job> = BTreeMap::new();

    let file = File::open(FILENAME)?;
    let mut read = Reader::from_reader(file);

    let mut n = 0;
    for record in read.records() {
        let job = record?;
        let listing = Job::new_job(
            job.get(0).unwrap().to_string(), 
            job.get(1).unwrap().to_string(), 
            job.get(2).unwrap().to_string(), 
            job.get(3).unwrap().to_string(), 
            job.get(4).unwrap().to_string()
        );

        master.insert(n, listing);
        n += 1;
    }

    Ok(master)
}
