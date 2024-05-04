use std::{collections::HashSet, io::Write, path::PathBuf};

use anyhow::Result;
use thiserror::Error;
use time::OffsetDateTime;

use crate::file_integrity::Checksum;

/// A specific report of a file's checksum results.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChecksumReport {
    pub file_path: PathBuf,
    /// Whether or not the checksums are matching. If false, assume the user is a cheating loser.
    pub is_matching: bool,
    pub got: Checksum,
}

/// A report of all the tool's analysis results.
/// TODO: Add PC specs, connected hardware, PID of opened programs, etc.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    incorrect_file_checksums: HashSet<ChecksumReport>,
}

impl Report {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self {
            incorrect_file_checksums: HashSet::new(),
        }
    }

    /// Sets the `incorrect_file_checksums` field from a Vec<ChecksumReport>.
    pub fn set_incorrect_file_checksums(&mut self, incorrect_file_checksums: Vec<ChecksumReport>) {
        self.incorrect_file_checksums = incorrect_file_checksums.into_iter().collect();
    }

    /// Writes the report to disk.
    /// TODO: choose the generated style (markdown, json, etc)
    pub fn generate_report(&self) -> Result<(), ReportError> {
        const REPORT_FOLDER: &str = "./results/";

        let utc_date = OffsetDateTime::now_utc();
        let utc_string = {
            let mut s = String::new();

            // date
            s.push_str(&utc_date.date().to_string());
            s.push('_');

            // time
            let time = utc_date.time();
            s.push_str(&time.hour().to_string());
            s.push(':');
            s.push_str(&time.minute().to_string());

            s
        };

        let file_path = format!("{}{}.report", REPORT_FOLDER, utc_string);

        std::fs::create_dir_all(REPORT_FOLDER)?;

        let mut file = std::fs::File::create(file_path)?;

        writeln!(file, "Report generated on: {}\n\n## Checksums", utc_date)?;

        for checksum_report in &self.incorrect_file_checksums {
            writeln!(
                file,
                "[incorrect] --> {}: {}",
                checksum_report.file_path.to_string_lossy(),
                checksum_report.got
            )?;
        }

        Ok(())
    }
}

/// An error that can occur when generating a report.
#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Failed to build report due to IO error: {0}")]
    IoError(#[from] std::io::Error),
}
