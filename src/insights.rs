//! Returning a PrettyTable of job application statistics.

use crate::format::format_table;
use crate::model::Job;

use prettytable::*;

use std::collections::BTreeMap;

/// A struct that will contain data for each job status.
#[derive(Debug, PartialEq)]
pub struct JobStats {
    pending: f64,
    in_progress: f64,
    offers: f64,
    hired: f64,
    rejected: f64,

    total: f64
}

impl JobStats {
    /// Calculate job stats using data from the JobStats struct.
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

/// Return a JobStruct containing data for each job status.
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
            "PENDING" => { current_stats.pending += 1.0; },
            "IN PROGRESS" => { current_stats.in_progress += 1.0; },
            "OFFER RECEIVED" => { current_stats.offers += 1.0; },
            "HIRED" => { current_stats.hired += 1.0; },
            "REJECTED" => { current_stats.rejected += 1.0; },
            _ => ()
        }
    }

    current_stats
}

/// Adding PrettyTable cells for the number of jobs and the ratio within each 
/// job status in the PrettyTable.
fn get_job_count(current_stats: &JobStats, insights: &mut Table, is_percent: bool) {
    let stats: Vec<(f64, &str)> = vec![
        (current_stats.pending, "Fbc"),
        (current_stats.in_progress, "Fyc"),
        (current_stats.offers, "Fmc"),
        (current_stats.hired, "Fgc"),
        (current_stats.rejected, "Frc")
    ];

    let mut table_values: Vec<Cell> = Vec::new();

    for stat in stats {
        match is_percent {
            true => {
                table_values.push(Cell::new(
                    &format!("{:.2}% of all jobs", stat.0 * 100.0))
                        .style_spec(stat.1)
                );
            },
            false => {
                let plurality = if &stat.0 == &1.0 { "job" } else { "jobs" };
                table_values.push(Cell::new(
                    &format!("{} {}", stat.0, plurality))
                        .style_spec(stat.1)
                );
            }
        }
    }

    insights.add_row(Row::new(table_values));
}

/// Add job statistics calculations to the insights PrettyTable, then display
/// the table.
pub fn display_insights(current_stats: JobStats) {
    let mut insights = Table::new();

    insights.set_titles(Row::new(vec![
        Cell::new(&format!("{} TRACKED JOB APPLICATIONS", current_stats.total))
            .style_spec("bcH5")
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

    get_job_count(&current_stats, &mut insights, false);
    
    let current_stats = current_stats.calculate();
    get_job_count(&current_stats, &mut insights, true);

    insights.set_format(format_table());
    insights.printstd();
}

#[cfg(test)]
mod test_insights {
    use super::*;

    #[test]
    fn test_jobstats_struct() {
        let test_stats = JobStats {
            pending: 10.0,
            in_progress: 1.0,
            offers: 3.0,
            hired: 0.0,
            rejected: 8.0,

            total: 22.0
        };

        assert_eq!(test_stats.pending, 10.0);
        assert_eq!(test_stats.in_progress, 1.0);
        assert_eq!(test_stats.offers, 3.0);
        assert_eq!(test_stats.hired, 0.0);
        assert_eq!(test_stats.rejected, 8.0);
        assert_eq!(test_stats.total, 22.0);
    }

    #[test]
    fn test_calculate_jobstats() {
        let test_stats = JobStats {
            pending: 4.0,
            in_progress: 2.0,
            offers: 2.0,
            hired: 3.0,
            rejected: 3.0,

            total: 14.0
        };

        let test_stats = test_stats.calculate();

        assert_eq!((test_stats.pending * 100.0).round(), 29.0);
        assert_eq!((test_stats.in_progress * 100.0).round(), 14.0);
        assert_eq!((test_stats.offers * 100.0).round(), 14.0);
        assert_eq!((test_stats.hired * 100.0).round(), 21.0);
        assert_eq!((test_stats.rejected * 100.0).round(), 21.0);
    }

    #[test]
    fn test_get_stats() {
        let mut test_master: BTreeMap<u16, Job> = BTreeMap::new();

        let test_job = Job {
            date: "".to_string(),
            company: "".to_string(),
            title: "".to_string(),
            status: "HIRED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(0, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "".to_string(),
            title: "".to_string(),
            status: "HIRED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(1, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "".to_string(),
            title: "".to_string(),
            status: "REJECTED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(2, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "".to_string(),
            title: "".to_string(),
            status: "IN PROGRESS".to_string(),
            notes: "".to_string()
        };
        test_master.insert(3, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "".to_string(),
            title: "".to_string(),
            status: "OFFER RECEIVED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(4, test_job);

        let test_job = Job {
            date: "".to_string(),
            company: "".to_string(),
            title: "".to_string(),
            status: "OFFER RECEIVED".to_string(),
            notes: "".to_string()
        };
        test_master.insert(5, test_job);

        let current_stats = get_stats(test_master);

        assert_eq!(JobStats {
            pending: 0.0,
            in_progress: 1.0,
            offers: 2.0,
            hired: 2.0,
            rejected: 1.0,
            total: 6.0
        }, current_stats);
    }
}
