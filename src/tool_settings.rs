use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    config: ToolsSettings,
}

#[derive(Deserialize, Debug)]
pub struct ToolsSettings {
    game_path: String,
    cemu_path: String,
    // add other settings
}

impl ToolsSettings {
    pub fn new() -> Self {
        let file_path = "config/config.toml";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let config: Config =
            toml::from_str(&contents).expect("The TOML should be properly formatted");

        // Debug print
        println!("{:?}", config.config);

        config.config
    }
}
