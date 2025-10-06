use slint::{ComponentHandle, Weak};
use std::sync::Arc;

use crate::error::Result;
use crate::services::ServiceContainer;
use crate::state::{Page, StateAction, Store};

/// Loupe Page ViewModel
pub struct LoupePageManager {
    _container: Arc<ServiceContainer>,
    _store: Arc<Store>,
}

impl LoupePageManager {
    pub async fn new(
        ui: Weak<crate::Main>,
        container: Arc<ServiceContainer>,
        store: Arc<Store>,
    ) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let loupe_store = main.global::<crate::LoupePageStore>();

        // Handle back button - navigate to grid
        let store_back = store.clone();
        loupe_store.on_back_clicked(move || {
            tracing::info!("Back clicked from loupe");
            store_back.dispatch(StateAction::navigate_to(Page::Grid));
        });

        // Handle previous photo
        let store_prev = store.clone();
        loupe_store.on_prev_clicked(move || {
            let state = store_prev.get_state();
            let current = state.photos.current_index;

            if current > 0 {
                tracing::info!("Previous photo: {} → {}", current, current - 1);
                store_prev.dispatch(StateAction::select_photo(current - 1));
            } else {
                tracing::debug!("Already at first photo");
            }
        });

        // Handle next photo
        let store_next = store.clone();
        loupe_store.on_next_clicked(move || {
            let state = store_next.get_state();
            let current = state.photos.current_index;
            let total = state.photos.photos.len();

            if current < total.saturating_sub(1) {
                tracing::info!("Next photo: {} → {}", current, current + 1);
                store_next.dispatch(StateAction::select_photo(current + 1));
            } else {
                tracing::debug!("Already at last photo");
            }
        });

        tracing::info!("LoupePageManager initialized");

        Ok(Self {
            _container: container,
            _store: store,
        })
    }
}
