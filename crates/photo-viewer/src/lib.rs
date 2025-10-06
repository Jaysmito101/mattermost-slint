slint::include_modules!();

pub mod bridge;
pub mod constants;
pub mod error;
pub mod models;
pub mod router;
pub mod services;
pub mod state;
pub mod viewmodels;

use bridge::UiBridge;
use router::Router;
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

    // Create service container
    let container = Arc::new(ServiceContainer::new(store.clone())?);
    tracing::info!("Service container created");

    // Create router
    let _router = Arc::new(Router::new(store.clone()));
    tracing::info!("Router created");

    // Create UI bridge (handles state → UI syncing)
    let ui_bridge = UiBridge::new(ui.as_weak(), store.clone());
    ui_bridge.initialize()?;
    tracing::info!("UI bridge initialized");

    // Initialize ViewModels
    let _view_models =
        viewmodels::initialize(ui.as_weak(), container.clone(), store.clone()).await?;
    tracing::info!("ViewModels initialized");

    tracing::info!("Photo Viewer ready!");

    // Run UI event loop
    ui.run()?;
    Ok(())
}

/// Application struct for better organization
pub struct App {
    ui: Main,
    container: Arc<ServiceContainer>,
    store: Arc<Store>,
    router: Arc<Router>,
    _view_models: Arc<viewmodels::ViewModels>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let ui = Main::new()?;
        let store = Arc::new(Store::new());
        let container = Arc::new(ServiceContainer::new(store.clone())?);
        let router = Arc::new(Router::new(store.clone()));

        let ui_bridge = UiBridge::new(ui.as_weak(), store.clone());
        ui_bridge.initialize()?;

        let view_models =
            viewmodels::initialize(ui.as_weak(), container.clone(), store.clone()).await?;

        Ok(Self {
            ui,
            container,
            store,
            router,
            _view_models: view_models,
        })
    }

    pub fn run(self) -> Result<()> {
        self.ui.run()?;
        Ok(())
    }

    // Expose useful accessors
    pub fn container(&self) -> Arc<ServiceContainer> {
        self.container.clone()
    }

    pub fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    pub fn router(&self) -> Arc<Router> {
        self.router.clone()
    }
}
