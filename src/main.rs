use config::Config;
use file_integrity::FileIntegrity;
mod config;
mod file_integrity;

const ERROR_STATUS: i32 = -84;

fn main() {
    match Config::new() {
        Some(cfg) => {
            Config::print_config(&cfg);
            match FileIntegrity::new(&cfg) {
                Ok(integrity) => {
                    // No error, prints the struct
                    FileIntegrity::print_file_integrity(&integrity);
                }
                Err(e) => {
                    // If an error occurs while reading directories/files, handle it here
                    eprintln!("Failed to initialize file integrity checks: {}", e);
                    std::process::exit(ERROR_STATUS);
                }
            }
        }
        None => {
            eprintln!("Failed to load configuration.");
            std::process::exit(ERROR_STATUS);
        }
    }
}
