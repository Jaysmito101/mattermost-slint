mod common;
pub use common::*;

slint::include_modules!();

pub async fn initialize() -> Result<(), crate::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();

    Ok(())
}

pub async fn run() -> Result<(), crate::Error> {
    log::warn!("Hello World!");

    let ui = Main::new().map_err(crate::Error::SlintError)?;

    ui.run().map_err(crate::Error::SlintError)?;

    Ok(())
}
