slint::include_modules!();

mod common;
pub use common::*;

pub mod services;
pub mod viewmodels;

pub async fn initialize() -> Result<(), crate::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();

    Ok(())
}

pub async fn run() -> Result<(), crate::Error> {
    let ui = Main::new().map_err(crate::Error::SlintError)?;

    let app_services = crate::services::initialize(ui.as_weak()).await?;
    let _app_view_models = crate::viewmodels::initialize(ui.as_weak(), app_services.api().clone()).await?;

    ui.run().map_err(crate::Error::SlintError)?;
    Ok(())
}
