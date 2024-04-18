use serde::Deserialize;
use std::fs;

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

    pub fn new() -> Config {
        let file_path = "config/config.toml";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let config: Config =
            toml::from_str(&contents).expect("The TOML should be properly formatted");

        config
    }
}
