/// TODO: how to retrieve the actual pure checksum (reference value)

/// Fictional struct for the moment / prob use an external library
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Checksum {
    value: u32,
}

/// A MK8 game file and its expected `Checksum` (ref. value).
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
struct GameFile {
    path: &'static str,
    expected_checksum: Checksum,
}

impl GameFile {
    pub fn compute_real_checksum(&self) -> Checksum {
        // TODO: compute real checksum using `self.path`
        Checksum { value: 42 }
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
