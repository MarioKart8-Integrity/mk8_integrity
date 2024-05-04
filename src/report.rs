use std::{collections::HashSet, io::Write};

use anyhow::Result;
use thiserror::Error;
use time::OffsetDateTime;

/// A specific report of a file's checksum results.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChecksumReport {
    file_path: String,
    matching: bool,
    got: String,
}

/// A report of all the tool's analysis results.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report {
    files_checksums: HashSet<ChecksumReport>,
}

impl Report {
    /// Creates an empty report.
    pub fn new() -> Self {
        Self {
            files_checksums: HashSet::new(),
        }
    }

    /// Adds a file's checksum results to the report.
    pub fn set_file_checksum(&mut self, file_path: String, matching: bool, got: String) {
        self.files_checksums.insert(ChecksumReport {
            file_path,
            matching,
            got,
        });
    }

    /// Writes the report to disk.
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

        for checksum_report in &self.files_checksums {
            writeln!(
                file,
                "[incorrect] --> {}: {}",
                checksum_report.file_path, checksum_report.got
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
