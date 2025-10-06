slint::include_modules!();

pub mod bridge;
pub mod constants;
pub mod error;
pub mod services;
pub mod state;
pub mod viewmodels;

use bridge::UiBridge;
use services::ServiceContainer;
use state::Store;
use std::sync::Arc;

pub use error::{Error, Result};

/// Initialize application logging
pub async fn initialize() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    tracing::info!("Photo Viewer initializing...");
    Ok(())
}

/// Run the application
pub async fn run() -> Result<()> {
    // Create UI
    let ui = Main::new()?;
    tracing::info!("UI created");

    // Create state store
    let store = Arc::new(Store::new());
    tracing::info!("State store created");

    // Create service container (stateless services)
    let container = Arc::new(ServiceContainer::new());
    tracing::info!("Service container created");

    // Create UI bridge (handles state → UI syncing)
    let ui_bridge = UiBridge::new(ui.as_weak(), store.clone());
    ui_bridge.initialize()?;
    tracing::info!("UI bridge initialized");

    // Initialize ViewModels
    let _view_models = viewmodels::initialize(ui.as_weak(), container.clone(), store.clone())?;
    tracing::info!("ViewModels initialized");

    tracing::info!("Photo Viewer ready!");

    // Run UI event loop
    ui.run()?;
    Ok(())
}
