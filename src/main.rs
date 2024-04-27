use config::Config;
use file_integrity::FileIntegrity;
mod config;
mod file_integrity;

const ERROR_STATUS: i32 = -84;

fn main() {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    match Config::new() {
        Ok(cfg) => {
            Config::print_config(&cfg);
        }
        Err(e) => {
            tracing::error!("Failed to load configuration. Error: {:?}", e);
            panic!("Couldn't load configuration.");
        }
    }
}
