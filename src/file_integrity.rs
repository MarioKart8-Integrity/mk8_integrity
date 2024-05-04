use sha2::{Digest, Sha256};
use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::{config::Config, report::ChecksumReport};

/// TODO: how to retrieve the actual pure checksum (reference value)

/// Fictional struct for the moment / prob use an external library
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Checksum {
    value: Vec<u8>,
}

impl Checksum {
    /// A checksum with no stored data.
    pub const UNIT_CHECKSUM: Checksum = Checksum { value: vec![] };
}

impl std::fmt::Display for Checksum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.value))
    }
}

/// A MK8 game file and its expected `Checksum` (ref. value).
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
struct GameFile {
    path: PathBuf,
    expected_checksum: Checksum,
    computed_checksum: Checksum,
}

impl GameFile {
    pub fn compute_real_checksum(&mut self) -> Checksum {
        let mut hasher = Sha256::new();
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(_) => return Checksum::UNIT_CHECKSUM,
        };

        hasher.update(&bytes);

        // the checksum we get from the file
        let checksum_bytes = hasher.finalize();

        // dbg variables
        let hex_to_str: String =
            checksum_bytes
                .to_vec()
                .iter()
                .fold(String::new(), |mut out, b| {
                    write!(out, "{:02x}", b).expect("Checksum hex string");
                    out
                });
        dbg!(&self.path, hex_to_str);
        // end of dbg

        self.computed_checksum = Checksum {
            value: checksum_bytes.to_vec(),
        };

        Checksum {
            value: checksum_bytes.to_vec(),
        }
    }

    /// Makes a `ChecksumReport` for this GameFile.
    pub fn get_report(&mut self) -> ChecksumReport {
        // make the report
        ChecksumReport {
            file_path: self.path.clone(),
            is_matching: self.checksums_match(),
            got: self.computed_checksum.clone(),
        }
    }

    /// Checks if the checksums are equal + compute the checksum of the file
    pub fn checksums_match(&mut self) -> bool {
        let real_checksum = self.compute_real_checksum();
        self.expected_checksum == real_checksum
    }
}

/// Are all those derive useful?
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct FileIntegrity {
    game_files: Vec<GameFile>,
}

/// Apparently does not need to have a `new` because we can
/// serde::Deserialize it from the config.
impl FileIntegrity {
    pub fn new(cfg: &Config) -> Result<Self, IntegrityError> {
        let mut game_files = Vec::new();
        let game_path = Path::new(cfg.get_mk8_folder());

        // based on the game path, creates the GameFiles by opening every
        // folder and adding every file found in it
        Self::add_files_recursively(game_path, &mut game_files)?;
        Ok(FileIntegrity { game_files })
    }

    /// Reads all the sub-directories and add found path to a file to the `game_files` vector
    fn add_files_recursively(
        path: &Path,
        game_files: &mut Vec<GameFile>,
    ) -> Result<(), IntegrityError> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let e = entry?;
                let p = e.path();

                if p.is_dir() {
                    Self::add_files_recursively(&p, game_files)?;
                } else {
                    let g_file: GameFile = GameFile {
                        path: p.to_path_buf(),
                        expected_checksum: Checksum::UNIT_CHECKSUM, // Placeholder
                        computed_checksum: Checksum::UNIT_CHECKSUM, // Placeholder
                    };
                    game_files.push(g_file);
                }
            }
        } else {
            tracing::debug!("Path does not exist: {}", path.display());
        }

        Ok(())
    }

    /// Checking if EVERY file is actully matching the expected checksum
    pub fn check(&mut self) -> Result<(), Vec<ChecksumReport>> {
        let mut failed_files = vec![];

        // TODO: remove the [..10] slice
        for f in self.game_files[..10].iter_mut() {
            if !f.checksums_match() {
                failed_files.push(f.get_report());
            }
        }

        if failed_files.is_empty() {
            Ok(()) // no failures :3
        } else {
            Err(failed_files)
        }
    }
}

#[derive(Debug, Error)]
pub enum IntegrityError {
    #[error("An expected file was not found. Error: {0}")]
    MissingFile(#[from] std::io::Error),
    #[error("A file's checksum does not match any expected ones: {path} shouldn't have the signature of: `{checksum}`")]
    _ChecksumMismatch { path: String, checksum: Checksum },
}
