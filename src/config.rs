use serde::Deserialize;
use std::fs;
use thiserror::Error;

/// ## Paths
///
/// The path to the folder where `MK8` and `CEMU` are installed
#[allow(unused)] // TODO: remove when we use `cemu_folder`
#[derive(Deserialize, Debug)]
pub struct Paths {
    mk8_folder: String,
    cemu_folder: String,
    // add other paths
}

/// ## Config
///
/// The configuration of the program
#[derive(Deserialize, Debug)]
pub struct Config {
    paths: Paths,
    // add other config options (moss, ...)
}

impl Config {
    /// Attempts to load the tool's configuration file.
    pub fn new() -> Result<Config, ConfigError> {
        let file_path = "config/config.toml";

        // try to read the file
        let contents = fs::read_to_string(file_path)
            .map_err(|e| ConfigError::NotFound(format!("{}: {e}", file_path)))?;

        // Attempt to parse the actual file
        match toml::from_str(&contents) {
            Ok(conf) => Ok(conf),
            Err(e) => Err(ConfigError::ParseFailed(e)),
        }
    }

    pub fn get_mk8_folder(&self) -> &str {
        &self.paths.mk8_folder
    }
}

/// An error that occurs while loading the tool's configuration file.
#[derive(Clone, Debug, Error)]
pub enum ConfigError {
    #[error("Couldn't find the given config file at given path: {0}")]
    NotFound(String),
    #[error("Confguration file uses an invalid schema: {0}")]
    ParseFailed(#[from] toml::de::Error),
}
