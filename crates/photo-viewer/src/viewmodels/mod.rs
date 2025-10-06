use slint::Weak;
use std::sync::Arc;

mod grid_page;
mod import_page;
mod loupe_page;

pub use grid_page::GridPageManager;
pub use import_page::ImportPageManager;
pub use loupe_page::LoupePageManager;

use crate::error::Result;
use crate::services::ServiceContainer;
use crate::state::Store;

/// Collection of all ViewModels in the application
pub struct ViewModels {
    pub import_page: ImportPageManager,
    pub grid_page: GridPageManager,
    pub loupe_page: LoupePageManager,
}

/// Initialize all ViewModels and wire up their callbacks
pub async fn initialize(
    ui: Weak<crate::Main>,
    container: Arc<ServiceContainer>,
    store: Arc<Store>,
) -> Result<Arc<ViewModels>> {
    tracing::info!("Initializing ViewModels...");

    // Initialize all page ViewModels
    let import_page = ImportPageManager::new(ui.clone(), container.clone(), store.clone()).await?;
    let grid_page = GridPageManager::new(ui.clone(), container.clone(), store.clone()).await?;
    let loupe_page = LoupePageManager::new(ui.clone(), container.clone(), store.clone()).await?;

    tracing::info!("All ViewModels initialized");

    Ok(Arc::new(ViewModels {
        import_page,
        grid_page,
        loupe_page,
    }))
}
