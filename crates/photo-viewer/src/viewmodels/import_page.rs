use crate::error::Result;
use crate::services::{photos, ServiceContainer};
use crate::state::{Page, StateAction, Store};
use slint::{ComponentHandle, Weak};
use std::sync::Arc;

/// Import Page ViewModel - wires UI callbacks to service workflows
pub struct ImportPageManager;

impl ImportPageManager {
    pub fn new(
        ui: Weak<crate::Main>,
        container: Arc<ServiceContainer>,
        store: Arc<Store>,
    ) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let import_store = main.global::<crate::ImportPageStore>();

        // Handle browse button - spawn async workflow
        import_store.on_browse_clicked({
            let container = container.clone();
            let store = store.clone();
            move || {
                let container = container.clone();
                let store = store.clone();
                tokio::spawn(async move {
                    // Service workflows handle their own errors and dispatch to state
                    if let Err(e) = photos::browse_and_load_photos(container, store).await {
                        tracing::error!("Unexpected error in browse workflow: {:?}", e);
                    }
                });
            }
        });

        // Handle load button - spawn async workflow with validation
        import_store.on_load_clicked({
            let container = container.clone();
            let store = store.clone();
            move |album_path: slint::SharedString| {
                let container = container.clone();
                let store = store.clone();
                let path_str = album_path.as_str();

                // Basic validation before spawning expensive operation
                if path_str.trim().is_empty() {
                    tracing::warn!("Empty path provided");
                    store.dispatch(StateAction::show_error(
                        "Please enter a valid path".to_string(),
                    ));
                    return;
                }

                let path = std::path::PathBuf::from(path_str);

                tokio::spawn(async move {
                    // Service workflows handle their own errors and dispatch to state
                    if let Err(e) = photos::load_photos_from_path(container, store, path).await {
                        tracing::error!("Unexpected error in load workflow: {:?}", e);
                    }
                });
            }
        });

        // Handle back button - navigate to welcome page
        import_store.on_back_clicked({
            let store = store.clone();
            move || {
                tracing::info!("Back clicked from import");
                store.dispatch(StateAction::navigate_to(Page::Welcome));
            }
        });

        tracing::info!("ImportPageManager initialized");

        Ok(Self)
    }
}
