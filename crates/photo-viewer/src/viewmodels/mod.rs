use slint::Weak;
use std::sync::Arc;

mod grid_page;
mod import_page;
mod loupe_page;
mod welcome_page;

pub use grid_page::GridPageManager;
pub use import_page::ImportPageManager;
pub use loupe_page::LoupePageManager;
pub use welcome_page::WelcomePageManager;

use crate::error::Result;
use crate::services::ServiceContainer;
use crate::state::Store;

/// Collection of all ViewModels in the application
pub struct ViewModels {
    pub welcome_page: WelcomePageManager,
    pub import_page: ImportPageManager,
    pub grid_page: GridPageManager,
    pub loupe_page: LoupePageManager,
}

/// Initialize all ViewModels and wire up their callbacks
///
/// ViewModels are stateless - they just wire UI callbacks to actions/workflows.
/// The returned struct exists only to keep the managers in scope.
pub fn initialize(
    ui: Weak<crate::Main>,
    container: Arc<ServiceContainer>,
    store: Arc<Store>,
) -> Result<ViewModels> {
    tracing::info!("Initializing ViewModels...");

    // Initialize all page ViewModels
    let welcome_page = WelcomePageManager::new(ui.clone(), store.clone())?;
    let import_page = ImportPageManager::new(ui.clone(), container.clone(), store.clone())?;
    let grid_page = GridPageManager::new(ui.clone(), container.clone(), store.clone())?;
    let loupe_page = LoupePageManager::new(ui.clone(), container.clone(), store.clone())?;

    tracing::info!("All ViewModels initialized");

    Ok(ViewModels {
        welcome_page,
        import_page,
        grid_page,
        loupe_page,
    })
}
