use crate::model::Job;

use csv::{Reader, WriterBuilder};
use serde::Serialize;

use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;
use std::string::String;

const FILENAME: &str = "job_applications.csv";

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
    fn serialize_ow(i: u16, master: &BTreeMap<u16, Job>) -> Listing {
        // Serialize job with struct for easy spreadsheet overwriting
        Listing {
            date: master.get(&i).unwrap().date.to_string(),
            company: master.get(&i).unwrap().company.to_string(),
            title: master.get(&i).unwrap().title.to_string(),
            status: master.get(&i).unwrap().status.to_string(),
            notes: master.get(&i).unwrap().notes.to_string()
        }
    }

    fn serialize_a(job: &Job) -> Listing {
        // Serialize job with struct for easy spreadsheet writing
        Listing {
            date: job.date.to_string(),
            company: job.company.to_string(),
            title: job.title.to_string(),
            status: job.status.to_string(),
            notes: job.notes.to_string()
        }
    }
}

fn existence() -> bool {
    // Check if "job_applications.csv" file exists in the current directory.
    return Path::new(FILENAME).exists();
}

fn create(file: File, job: &Job) -> Result<(), Box<dyn Error>> {
    // Create new spreadsheet and add headers and job listing

    let mut writer = WriterBuilder::new().has_headers(true).from_writer(file);
    writer.serialize(Listing::serialize_a(&job))?;

    Ok(())
}

fn append(file: File, job: &Job) -> Result<(), Box<dyn Error>> {
    // Append new job listing to spreadsheet

    let mut writer = WriterBuilder::new().from_writer(file);
    writer.serialize((
        job.date.to_string(), job.company.to_string(), job.title.to_string(), 
        job.status.to_string(), job.notes.to_string()
    ))?;

    Ok(())
}

pub fn write_new_job(job: &Job) -> Result<(), Box<dyn Error>> {
    // Write jobs to spreadsheet. Add header if file does not already exist, else
    // just append the job to the spreadsheet.
    
    if existence() == true {
        let file = OpenOptions::new().append(true).open(FILENAME)?;
        if let Err(err) = append(file, &job) {
            println!("An error has occured! {}", err);
            process::exit(1);
        } else {
            println!("\nADDED NEW LISTING.");
        }
    } else {
        let file = OpenOptions::new().create(true).write(true).open(FILENAME)?;
        if let Err(err) = create(file, &job) {
            println!("An error has occured! {}", err);
            process::exit(1);
        } else {
            println!("\nCREATED SPREADSHEET AND ADDED NEW LISTING.");
        }
    };

    Ok(())
}

fn rewrite(file: File, master: BTreeMap<u16, Job>) -> Result<(), Box<dyn Error>> {
    // Write jobs stored in master BTreeMap to spreadsheet.

    let mut writer = WriterBuilder::new().has_headers(true).from_writer(file);
    for i in 0u16..master.len() as u16 {
        writer.serialize(Listing::serialize_ow(i, &master))?;
    }

    Ok(())
}

pub fn overwrite(master: BTreeMap<u16, Job>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new().write(true).open(FILENAME)?;
    if let Err(err) = rewrite(file, master) {
        println!("An error has occured! {}", err);
        process::exit(1);
    } else {
        println!("\nUPDATED LISTING.")
    }

    Ok(())
}

pub fn get_jobs() -> Result<BTreeMap<u16, Job>, Box<dyn Error>> {
    // Get jobs from the spreadsheet and return BTreeMap of Job objects and its
    // index (location in the spreadsheet). Throw an error if there are problems
    // with parsing the spreadsheet.

    let mut master: BTreeMap<u16, Job> = BTreeMap::new();

    let file = File::open(FILENAME)?;
    let mut read = Reader::from_reader(file);

    let mut n = 0;
    for record in read.records() {
        let job = record?;
        let listing = Job::new_job(
            job.get(0).unwrap().to_string(), job.get(1).unwrap().to_string(), 
            job.get(2).unwrap().to_string(), job.get(3).unwrap().to_string(), 
            job.get(4).unwrap().to_string());

        master.insert(n, listing);
        n += 1;
    }

    Ok(master)
}

pub fn get_jobs_handler() -> BTreeMap<u16, Job> {
    match get_jobs() {
        Ok(master) => {
            return master;
        },
        Err(e) => {
            println!("{:?}", e);
            process::exit(1);
        }
    };
}