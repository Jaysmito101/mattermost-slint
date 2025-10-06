use slint::{ComponentHandle, Weak};
use std::sync::Arc;

use crate::error::Result;
use crate::state::{Page, StateAction, Store};

/// Welcome Page ViewModel - wires UI callbacks to state actions
pub struct WelcomePageManager;

impl WelcomePageManager {
    pub fn new(ui: Weak<crate::Main>, store: Arc<Store>) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let welcome_store = main.global::<crate::WelcomePageStore>();

        // Handle start button - navigate to import page
        welcome_store.on_start_clicked(move || {
            tracing::info!("Start clicked from welcome");
            store.dispatch(StateAction::navigate_to(Page::Import));
        });

        tracing::info!("WelcomePageManager initialized");

        Ok(Self)
    }
}
