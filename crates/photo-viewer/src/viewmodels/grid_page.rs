use crate::error::Result;
use crate::services::ServiceContainer;
use crate::state::{Page, StateAction, Store};
use slint::{ComponentHandle, Weak};
use std::sync::Arc;

/// Grid Page ViewModel - wires UI callbacks to state actions
pub struct GridPageManager;

impl GridPageManager {
    pub fn new(
        ui: Weak<crate::Main>,
        _container: Arc<ServiceContainer>,
        store: Arc<Store>,
    ) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let grid_store = main.global::<crate::GridPageStore>();

        // Handle photo clicked - navigate to loupe view
        grid_store.on_photo_clicked({
            let store = store.clone();
            move |index| {
                tracing::info!("Photo clicked: index={}", index);
                store.dispatch(StateAction::select_photo(index as usize));
                store.dispatch(StateAction::navigate_to(Page::Loupe));
            }
        });

        // Handle back button
        grid_store.on_back_clicked({
            let store = store.clone();
            move || {
                tracing::info!("Back clicked from grid");
                store.dispatch(StateAction::navigate_to(Page::Import));
            }
        });

        // Handle reimport button
        grid_store.on_reimport_clicked({
            let store = store.clone();
            move || {
                tracing::info!("Reimport clicked from grid");
                store.dispatch(StateAction::navigate_to(Page::Import));
            }
        });

        tracing::info!("GridPageManager initialized");

        Ok(Self)
    }
}
