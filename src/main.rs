use anyhow::Result;
use tracing::{Level, error, info, span};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    println!("Starting cgol-rs binary");
    println!("Configuring logging");

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    info!("Logging configured, entering root span main");
    let span = span!(Level::INFO, "main");
    let _enter = span.enter();
    info!("Starting cgol-rs library");
    match cgol_rs::main() {
        Ok(_) => {
            info!("Finished cgol-rs library");
        }
        Err(err) => {
            error!("Error running cgol-rs library: {}", err);
        }
    }
    info!("Finished cgol-rs binary successfully");
    Ok(())
}
