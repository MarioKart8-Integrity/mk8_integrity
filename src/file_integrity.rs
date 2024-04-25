use sha2::{Digest, Sha256};
use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::config::Config;

/// TODO: how to retrieve the actual pure checksum (reference value)

/// Fictional struct for the moment / prob use an external library
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Checksum {
    value: Vec<u8>,
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
        let hex_to_str: String = res.to_vec().iter().map(|b| format!("{:02x}", b)).collect();
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
    pub fn new(cfg: &Config) -> io::Result<Self> {
        let mut game_files = Vec::new();
        let game_path = Path::new(cfg.get_mk8_folder());

        // based on the game path, creates the GameFiles by opening every folder and adding every
        // file found in it
        Self::add_files_recursively(game_path, &mut game_files)?;
        Ok(FileIntegrity { game_files })
    }

    /// Prints the `FileIntegrity` struct to debug output
    pub fn print_file_integrity(&self) {
        dbg!(&self);
    }

    /// Reads all the sub-directories and add found path to a file to the `game_files` vector
    fn add_files_recursively(path: &Path, game_files: &mut Vec<GameFile>) -> io::Result<()> {
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
            println!("Path does not exist: {}", path.display());
        }

        Ok(())
    }

    /// Checking if the integrity of the files is good.
    pub fn check(&self) -> bool {
        for f in self.game_files.iter() {
            if !f.checksums_match() {
                return false;
            }
        }
        true
    }
}
