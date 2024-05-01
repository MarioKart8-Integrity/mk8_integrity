use std::{collections::HashSet, io::Write};

use thiserror::Error;
use anyhow::Result;
use time::OffsetDateTime;

/// A specific report of a file's checksum results.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChecksumReport {
    file_name: String,
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
    pub fn set_file_checksum(&mut self, checksum_report: &ChecksumReport) {
        self.files_checksums.insert(checksum_report.clone());
    }

    /// Writes the report to disk.
    pub fn generate_report(&self) -> Result<(), ReportError> {
        const REPORT_FOLDER: &str = "./results/";

        let utc_date = OffsetDateTime::now_utc().to_string();
        let file_path = format!("{}{}", REPORT_FOLDER, utc_date); 

        std::fs::create_dir_all(REPORT_FOLDER)?;

        let mut file = std::fs::File::create(file_path)?;
        file.write("hello world".as_bytes())?;

        Ok(())

    }
}

/// An error that can occur when generating a report.
#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Failed to build report due to IO error: {0}")]
    IoError(#[from] std::io::Error),
}
