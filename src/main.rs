use config::Config;
use file_integrity::FileIntegrity;
mod config;
mod file_integrity;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    match FileIntegrity::new(&Config::new()?) {
        Ok(app) => {
            let res = app.check();
            println!("File check result: {}", res);
        }
        Err(e) => {
            tracing::error!("Failed to initialize the application. Error: {:?}", e);
            return Err(anyhow::anyhow!("Couldn't initialize the application."));
        }
    }

    Ok(())
}
