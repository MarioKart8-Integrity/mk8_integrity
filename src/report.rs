use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct ChecksumReport {
    file_name: String,
    matching: bool,
    got: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Report {
    files_checksums: HashSet<ChecksumReport>,
}

impl Report {
    pub fn new() -> Self {
        Self {
            files_checksums: HashSet::new(),
        }
    }

    pub fn set_file_checksum(&mut self, checksum_report: ChecksumReport) {
        self.files_checksums
    }
}
