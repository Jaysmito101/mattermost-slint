use crate::error::Result;
use crate::state::{Page, StateAction, Store};
use std::sync::Arc;

/// Router for managing navigation
pub struct Router {
    store: Arc<Store>,
}

impl Router {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    /// Navigate to a page
    pub async fn navigate_to(&self, page: Page) -> Result<()> {
        tracing::info!("Router: Navigating to {:?}", page);
        self.store.dispatch(StateAction::navigate_to(page));
        Ok(())
    }

    /// Go back in navigation history
    pub async fn go_back(&self) -> Result<()> {
        tracing::info!("Router: Going back");
        self.store.dispatch(StateAction::go_back());
        Ok(())
    }

    /// Get current page
    pub fn current_page(&self) -> Page {
        self.store.get_state().navigation.current_page
    }

    /// Check if can go back
    pub fn can_go_back(&self) -> bool {
        !self.store.get_state().navigation.history.is_empty()
    }
}
