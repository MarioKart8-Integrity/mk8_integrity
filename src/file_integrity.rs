use sha2::{Digest, Sha256};
use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

use crate::{config::Config, report::Report};

/// TODO: how to retrieve the actual pure checksum (reference value)

/// Fictional struct for the moment / prob use an external library
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Checksum {
    value: Vec<u8>,
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
}

impl GameFile {
    pub fn compute_real_checksum(&self) -> Checksum {
        let mut hasher = Sha256::new();
        let bytes = match fs::read(&self.path) {
            Ok(bytes) => bytes,
            Err(_) => return Checksum { value: vec![] },
        };

        hasher.update(&bytes);

        let res = hasher.finalize();

        // dbg variables
        let hex_to_str: String = res.to_vec().iter().fold(String::new(), |mut out, b| {
            write!(out, "{:02x}", b).expect("Checksum hex string");
            out
        });
        dbg!(&self.path, hex_to_str);
        // end of dbg

        Checksum {
            value: res.to_vec(),
        }
    }

    pub fn checksums_match(&self) -> bool {
        self.expected_checksum == self.compute_real_checksum()
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
                        expected_checksum: Checksum { value: vec![] }, // Placeholder
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
    pub fn check(&self, report: &mut Report) -> bool {
        let mut res = true;

        for f in self.game_files.iter() {
            if !f.checksums_match() {
                report.set_file_checksum(
                    f.path.to_str().unwrap().to_string(),
                    false,
                    f.compute_real_checksum().to_string(), // wouldn't it be better to change
                                                           // return type instead of calling twice this fn?
                );
                res = false;
                return res; // to be removed later,  this is to stop the loop
            }
        }
        res
    }
}

#[derive(Debug, Error)]
pub enum IntegrityError {
    #[error("An expected file was not found. Error: {0}")]
    MissingFile(#[from] std::io::Error),
    #[error("A file's checksum does not match any expected ones: {path} shouldn't have the signature of: `{checksum}`")]
    _ChecksumMismatch { path: String, checksum: Checksum },
}
