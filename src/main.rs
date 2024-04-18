use config::Config;

mod config;

fn main() {
    let cfg = Config::new();
    Config::print_config(&cfg)
}
