use serde::Deserialize;
use std::fs;
use thiserror::Error;

/// ## Paths
///
/// The path to the folder where `MK8` and `CEMU` are installed
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
    /// Prints the config of the program as a debug message
    pub fn print_config(&self) {
        dbg!(&self);
    }

    pub fn new() -> Option<Config> {
        let file_path = "config/config.toml";
        let contents = fs::read_to_string(file_path);

        if let Ok(str_contents) = contents {
            let config: Result<Config, toml::de::Error> = toml::from_str(&str_contents);
            if let Ok(parsed_config) = config {
                Some(parsed_config)
            } else {
                eprintln!("Error paersing TOML: {:?}", config.unwrap_err());
                None
            }
        } else {
            eprintln!("Error reading file: {:?}", contents.unwrap_err());
            None
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
