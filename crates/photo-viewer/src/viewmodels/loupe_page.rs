use slint::{ComponentHandle, Weak};
use std::sync::Arc;

use crate::error::Result;
use crate::services::ServiceContainer;
use crate::state::{Page, StateAction, Store};

/// Loupe Page ViewModel - wires UI callbacks to state actions
pub struct LoupePageManager;

impl LoupePageManager {
    pub fn new(
        ui: Weak<crate::Main>,
        _container: Arc<ServiceContainer>,
        store: Arc<Store>,
    ) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let loupe_store = main.global::<crate::LoupePageStore>();

        // Handle back button - navigate to grid
        loupe_store.on_back_clicked({
            let store = store.clone();
            move || {
                tracing::info!("Back clicked from loupe");
                store.dispatch(StateAction::navigate_to(Page::Grid));
            }
        });

        // Handle previous photo - let reducer handle bounds checking
        loupe_store.on_prev_clicked({
            let store = store.clone();
            move || {
                tracing::debug!("Previous photo clicked");
                store.dispatch(StateAction::previous_photo());
            }
        });

        // Handle next photo - let reducer handle bounds checking
        loupe_store.on_next_clicked({
            let store = store.clone();
            move || {
                tracing::debug!("Next photo clicked");
                store.dispatch(StateAction::next_photo());
            }
        });

        tracing::info!("LoupePageManager initialized");

        Ok(Self)
    }
}
