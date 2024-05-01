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
        Ok(_app) => {
            // let res = app.check();
            // println!("File check result: {}", res);

            let report = Report::new();
            report.generate_report()?;
        }
        Err(e) => {
            tracing::error!("Failed to initialize the application. Error: {:?}", e);
            return Err(anyhow::anyhow!("Couldn't initialize the application."));
        }
    }

    Ok(())
}
