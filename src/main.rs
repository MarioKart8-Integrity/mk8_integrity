use config::Config;
use file_integrity::FileIntegrity;
mod config;
mod file_integrity;

fn main() {
    match Config::new() {
        Some(cfg) => {
            Config::print_config(&cfg);
            let integrity = FileIntegrity::new(&cfg);
        }
        None => {
            eprintln!("Failed to load configuration.");
            std::process::exit(-84); // Can we also define `const` like c/c++ (const ERROR_STATUS =
                                     // -84)
        }
    }
}
