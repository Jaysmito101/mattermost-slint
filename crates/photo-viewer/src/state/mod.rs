use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

mod actions;
pub use actions::*;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub navigation: NavigationState,
    pub photos: PhotoState,
    pub ui: UiState,
}

#[derive(Clone, Debug, Default)]
pub struct NavigationState {
    pub current_page: Page,
}

#[derive(Clone, Debug, Default)]
pub struct PhotoState {
    pub album_path: Option<PathBuf>,
    pub photos: Vec<PhotoInfo>,
    pub current_index: usize,
}

#[derive(Clone, Debug)]
pub struct PhotoInfo {
    pub path: PathBuf,
    pub filename: String,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Default)]
pub struct UiState {
    pub is_loading: bool,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Welcome,
    Import,
    Grid,
    Loupe,
}

impl From<&Page> for crate::AppPage {
    fn from(page: &Page) -> Self {
        match page {
            Page::Welcome => crate::AppPage::Welcome,
            Page::Import => crate::AppPage::Import,
            Page::Grid => crate::AppPage::Grid,
            Page::Loupe => crate::AppPage::Loupe,
        }
    }
}

/// Subscription handle for cleanup
/// When dropped, automatically unsubscribes
pub struct Subscription {
    id: usize,
    store: Arc<StoreInner>,
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.store.unsubscribe(self.id);
    }
}

pub struct Store {
    inner: Arc<StoreInner>,
}

struct StoreInner {
    state: RwLock<Arc<AppState>>,
    subscribers: RwLock<HashMap<usize, Subscriber>>,
    next_id: AtomicUsize,
}

type Subscriber = Arc<dyn Fn(Arc<AppState>) + Send + Sync>;

impl Store {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(StoreInner {
                state: RwLock::new(Arc::new(AppState::default())),
                subscribers: RwLock::new(HashMap::new()),
                next_id: AtomicUsize::new(0),
            }),
        }
    }

    pub fn dispatch(&self, action: StateAction) {
        // Clone current state Arc for mutation
        let new_state = {
            let current = self.inner.state.read();
            let mut new_state = (**current).clone();

            // Apply reducers to mutable copy
            match action {
                StateAction::Navigation(nav_action) => {
                    Self::reduce_navigation(&mut new_state.navigation, nav_action);
                }
                StateAction::Photos(photo_action) => {
                    Self::reduce_photos(&mut new_state.photos, photo_action);
                }
                StateAction::Ui(ui_action) => {
                    Self::reduce_ui(&mut new_state.ui, ui_action);
                }
            }

            Arc::new(new_state)
        };

        // Update store with new immutable state
        *self.inner.state.write() = new_state.clone();

        // Clone subscribers to release lock before calling them
        // This prevents deadlock if a subscriber calls dispatch()
        let subscribers: Vec<_> = {
            let subs = self.inner.subscribers.read();
            subs.values().cloned().collect()
        };

        // Notify all subscribers without holding any locks
        for subscriber in subscribers {
            subscriber(new_state.clone());
        }
    }

    /// Subscribe to state changes. Returns a Subscription handle that auto-unsubscribes on drop.
    pub fn subscribe(
        &self,
        callback: impl Fn(Arc<AppState>) + Send + Sync + 'static,
    ) -> Subscription {
        let id = self.inner.next_id.fetch_add(1, Ordering::SeqCst);
        self.inner
            .subscribers
            .write()
            .insert(id, Arc::new(callback));

        Subscription {
            id,
            store: self.inner.clone(),
        }
    }

    /// Get current state as an Arc
    pub fn get_state(&self) -> Arc<AppState> {
        self.inner.state.read().clone()
    }

    // Private reducer functions
    fn reduce_navigation(state: &mut NavigationState, action: NavigationAction) {
        match action {
            NavigationAction::NavigateTo(page) => {
                if state.current_page != page {
                    tracing::info!("Navigation: {:?} → {:?}", state.current_page, page);
                    state.current_page = page;
                } else {
                    tracing::debug!("Navigation: Already on {:?}, skipping", page);
                }
            }
        }
    }

    fn reduce_photos(state: &mut PhotoState, action: PhotoAction) {
        match action {
            PhotoAction::SetAlbumPath(path) => {
                state.album_path = Some(path);
                state.photos.clear();
                state.current_index = 0;
            }
            PhotoAction::LoadPhotosStart => {}
            PhotoAction::LoadPhotosSuccess(photos) => {
                state.photos = photos;
                state.current_index = 0;
            }
            PhotoAction::LoadPhotosFailure => {}
            PhotoAction::SelectPhoto(index) => {
                if index < state.photos.len() {
                    state.current_index = index;
                }
            }
            PhotoAction::NextPhoto => {
                if !state.photos.is_empty() && state.current_index < state.photos.len() - 1 {
                    tracing::info!(
                        "Next photo: {} → {}",
                        state.current_index,
                        state.current_index + 1
                    );
                    state.current_index += 1;
                } else {
                    tracing::debug!("Already at last photo (index: {})", state.current_index);
                }
            }
            PhotoAction::PreviousPhoto => {
                if state.current_index > 0 {
                    tracing::info!(
                        "Previous photo: {} → {}",
                        state.current_index,
                        state.current_index - 1
                    );
                    state.current_index -= 1;
                } else {
                    tracing::debug!("Already at first photo");
                }
            }
            PhotoAction::ClearAlbum => {
                state.album_path = None;
                state.photos.clear();
                state.current_index = 0;
            }
        }
    }

    fn reduce_ui(state: &mut UiState, action: UiAction) {
        match action {
            UiAction::ShowLoading => {
                state.is_loading = true;
            }
            UiAction::HideLoading => {
                state.is_loading = false;
            }
            UiAction::ShowError(message) => {
                state.error_message = Some(message);
            }
            UiAction::ClearError => {
                state.error_message = None;
            }
        }
    }
}

impl StoreInner {
    fn unsubscribe(&self, id: usize) {
        self.subscribers.write().remove(&id);
        tracing::debug!("Unsubscribed: id={}", id);
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
