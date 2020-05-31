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
    #[serde(rename = "DATE")]
    date: String,
    #[serde(rename = "COMPANY")]
    company: String,
    #[serde(rename = "TITLE")]
    title: String,
    #[serde(rename = "STATUS")]
    status: String,
    #[serde(rename = "NOTES")]
    notes: String
}

impl Listing {
    fn serialize_job(i: u16, master: &BTreeMap<u16, Job>) -> Listing {
        // Serialize job with struct for easy spreadsheet writing
        Listing {
            date: master.get(&i).unwrap().date.to_string(),
            company: master.get(&i).unwrap().company.to_string(),
            title: master.get(&i).unwrap().title.to_string(),
            status: master.get(&i).unwrap().status.to_string(),
            notes: master.get(&i).unwrap().notes.to_string()
        }
    }
}

pub fn existence() -> bool {
    // Check if "job_applications.csv" file exists in the current directory.
    return Path::new(FILENAME).exists();
}

fn write_job(file: File, master: BTreeMap<u16, Job>) -> Result<(), Box<dyn Error>> {
    // Write jobs from master BTreeMap to spreadsheet.

    let mut writer = WriterBuilder::new().has_headers(true).from_writer(file);
    for i in 0u16..master.len() as u16 {
        writer.serialize(Listing::serialize_job(i, &master))?;
    }

    Ok(())
}

fn check_and_write(file: File, master: BTreeMap<u16, Job>) {
    // Error handling for write_job().

    match write_job(file, master) {
        Ok(()) => (),
        Err(e) => {
            println!("An error has occured! {:?}", e);
            process::exit(1);
        }
    };
}

pub fn add_job(master: BTreeMap<u16, Job>) -> 
                Result<String, Box<dyn Error>> {
    // Write jobs to spreadsheet. Add header if file does not already exist, else
    // just append the job to the spreadsheet.
    
    if existence() == true {
        let file = OpenOptions::new().append(true).open(FILENAME)?;
        check_and_write(file, master);
    } else {
        let file = OpenOptions::new().write(true).open(FILENAME)?;
        check_and_write(file, master);
    };

    Ok(String::from("Spreadsheet successfully written."))
}

///////////////////////////////////////// TODO: Finish overwrite(). Revisit add_job() as well
pub fn overwrite(master: BTreeMap<u16, Job>) {
    return;
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

// pub fn get_jobs_handler(fname: &str) -> BTreeMap<u16, Job> {
//     let master = match get_jobs(fname) {
//         Ok(master) => {
//             return master;
//         },
//         Err(e) => {
//             println!("{:?}", e);
//             process::exit(1);
//         }
//     };
// }