use crate::error::Result;
use crate::state::{AppState, Store, Subscription};
use slint::ComponentHandle;
use std::sync::Arc;

/// Bridge between Rust state and Slint UI
pub struct UiBridge {
    ui: slint::Weak<crate::Main>,
    store: Arc<Store>,
    _subscription: Subscription,
}

impl UiBridge {
    pub fn new(ui: slint::Weak<crate::Main>, store: Arc<Store>) -> Self {
        // Subscribe to state changes and update UI
        let subscription = store.subscribe({
            let ui = ui.clone();
            move |state: Arc<AppState>| {
                let ui = ui.clone();

                // Use invoke_from_event_loop to ensure UI updates happen on the correct thread
                let _ = ui.upgrade_in_event_loop(move |ui_handle| {
                    if let Err(e) = Self::sync_state_to_ui_internal(&ui_handle, &state) {
                        tracing::error!("Failed to sync state to UI: {:?}", e);
                    }
                });
            }
        });

        Self {
            ui: ui.clone(),
            store: store.clone(),
            _subscription: subscription,
        }
    }

    /// Sync state to UI (single source of truth)
    fn sync_state_to_ui(ui: &slint::Weak<crate::Main>, state: &AppState) -> Result<()> {
        let ui_handle = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        Self::sync_state_to_ui_internal(&ui_handle, state)
    }

    /// Internal sync that takes an upgraded UI handle
    fn sync_state_to_ui_internal(ui: &crate::Main, state: &AppState) -> Result<()> {
        // Update navigation state
        let nav_store = ui.global::<crate::NavStore>();

        let slint_page = crate::AppPage::from(&state.navigation.current_page);

        tracing::debug!(
            "UI Bridge: Syncing page to {:?}",
            state.navigation.current_page
        );
        nav_store.set_current_page(slint_page);

        // Update UI state
        nav_store.set_is_loading(state.ui.is_loading);
        if let Some(ref error) = state.ui.error_message {
            nav_store.set_error_message(error.clone().into());
        } else {
            nav_store.set_error_message("".into());
        }

        // Update photo state
        let photo_store = ui.global::<crate::PhotoStore>();

        // Update album info
        if let Some(ref path) = state.photos.album_path {
            photo_store.set_album(crate::AlbumData {
                path: path.to_string_lossy().to_string().into(),
                photo_count: state.photos.photos.len() as i32,
            });
        }

        // TODO: In production, use VirtualList or ModelNotify for efficient updates
        // Update photos list
        let photos: Vec<crate::PhotoData> = state
            .photos
            .photos
            .iter()
            .enumerate()
            .map(|(idx, photo)| crate::PhotoData {
                filename: photo.filename.clone().into(),
                path: photo.path.to_string_lossy().to_string().into(),
                index: idx as i32,
            })
            .collect();

        photo_store.set_photos(slint::ModelRc::new(slint::VecModel::from(photos)));
        photo_store.set_current_index(state.photos.current_index as i32);

        Ok(())
    }

    /// Initialize UI by syncing current state
    pub fn initialize(&self) -> Result<()> {
        let state = self.store.get_state();
        Self::sync_state_to_ui(&self.ui, &state)
    }
}
