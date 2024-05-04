use config::Config;
use file_integrity::FileIntegrity;
use report::Report;
mod config;
mod file_integrity;
mod report;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    match FileIntegrity::new(&Config::new()?) {
        Ok(mut app) => {
            let mut report = Report::new();

            match app.check() {
                Ok(_) => {
                    // we will not do anything, just continue program
                }
                Err(checksum_reports) => {
                    report.set_incorrect_file_checksums(checksum_reports);
                }
            }
            report.generate_report()?;
        }
        Err(e) => {
            tracing::error!("Failed to initialize the application. Error: {:?}", e);
            return Err(anyhow::anyhow!("Couldn't initialize the application."));
        }
    }

    Ok(())
}
