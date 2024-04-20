use config::Config;

mod config;

fn main() {
    match Config::new() {
        Some(cfg) => {
            Config::print_config(&cfg);
        }
        None => {
            eprintln!("Failed to load configuration.");
            std::process::exit(-84); // Can we also define `const` like c/c++ (const ERROR_STATUS =
                                     // -84)
        }
    }
}
