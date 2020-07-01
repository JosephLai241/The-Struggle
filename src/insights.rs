use crate::model::Job;

use prettytable::*;

use std::collections::BTreeMap;

/// A struct that will contain data for each job status.
pub struct JobStats {
    pending: f64,
    in_progress: f64,
    offers: f64,
    hired: f64,
    rejected: f64,

    total: f64
}

/// Implementation for calculating job stats using data from the JobStats struct.
impl JobStats {
    fn calculate(&self) -> JobStats {
        JobStats {
            pending: self.pending / self.total,
            in_progress: self.in_progress / self.total,
            offers: self.offers / self.total,
            hired: self.hired / self.total,
            rejected: self.rejected / self.total,

            total: self.total
        }
    }
}

/// Populate JobStats struct with data from the spreadsheet, then return a new
/// JobStruct containing the calculations for each job status.
pub fn get_stats(master: BTreeMap<u16, Job>) -> JobStats {
    let mut current_stats = JobStats {
        pending: 0.0,
        in_progress: 0.0,
        offers: 0.0,
        hired: 0.0,
        rejected: 0.0,

        total: master.len() as f64
    };

    for i in 0u16..master.len() as u16 {
        match master.get_key_value(&i).unwrap().1.status.as_str() {
            "PENDING" => current_stats.pending += 1.0,
            "IN PROGRESS" => current_stats.in_progress += 1.0,
            "OFFER RECEIVED" => current_stats.offers += 1.0,
            "HIRED" => current_stats.hired += 1.0,
            "REJECTED" => current_stats.rejected += 1.0,
            _ => ()
        }
    }

    current_stats.calculate()
}

/// Add job statistics calculations to a PrettyTable
pub fn display_insights(current_stats: JobStats) {
    let mut insights = Table::new();

    let header = format!("{} TRACKED JOB APPLICATIONS", current_stats.total);
    insights.set_titles(Row::new(vec![
        Cell::new(&header).style_spec("bcH5")
    ]));

    insights.add_row(
        row![
            bc => 
            "PENDING JOBS", 
            "IN PROGRESS", 
            "OFFERS RECEIVED", 
            "HIRES", 
            "REJECTIONS"
        ]);

    let pending = format!("{:.2}% of all jobs", current_stats.pending * 100.0);
    let in_progress = format!("{:.2}% of all jobs", current_stats.in_progress * 100.0);
    let offers = format!("{:.2}% of all jobs", current_stats.offers * 100.0);
    let hired = format!("{:.2}% of all jobs", current_stats.hired * 100.0);
    let rejected = format!("{:.2}% of all jobs", current_stats.rejected * 100.0);

    insights.add_row(Row::new(vec![
        Cell::new(&pending).style_spec("Fbc"),
        Cell::new(&in_progress).style_spec("Fyc"),
        Cell::new(&offers).style_spec("Fmc"),
        Cell::new(&hired).style_spec("Fgc"),
        Cell::new(&rejected).style_spec("Frc"),
        ]));

    insights.printstd();
}