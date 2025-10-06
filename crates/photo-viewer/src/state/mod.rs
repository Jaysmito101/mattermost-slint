use parking_lot::RwLock;
use std::path::PathBuf;
use std::sync::Arc;

mod actions;
pub use actions::*;

#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub navigation: NavigationState,
    pub photos: PhotoState,
    pub ui: UiState,
}

#[derive(Clone, Debug)]
pub struct NavigationState {
    pub current_page: Page,
    pub history: Vec<Page>,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self {
            current_page: Page::Welcome,
            history: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PhotoState {
    pub album_path: Option<PathBuf>,
    pub photos: Vec<PhotoInfo>,
    pub current_index: usize,
    pub is_loading: bool,
}

#[derive(Clone, Debug)]
pub struct PhotoInfo {
    pub path: PathBuf,
    pub filename: String,
    pub size_bytes: u64,
    pub dimensions: Option<(u32, u32)>,
}

#[derive(Clone, Debug, Default)]
pub struct UiState {
    pub is_loading: bool,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Page {
    Welcome,
    Import,
    Grid,
    Loupe,
}

impl Default for Page {
    fn default() -> Self {
        Page::Welcome
    }
}

pub struct Store {
    state: Arc<RwLock<AppState>>,
    subscribers: Arc<RwLock<Vec<Subscriber>>>,
}

type Subscriber = Box<dyn Fn(&AppState) + Send + Sync>;

impl Store {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(AppState::default())),
            subscribers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn dispatch(&self, action: StateAction) {
        let mut state = self.state.write();

        match action {
            StateAction::Navigation(nav_action) => {
                Self::reduce_navigation(&mut state.navigation, nav_action);
            }
            StateAction::Photos(photo_action) => {
                Self::reduce_photos(&mut state.photos, photo_action);
            }
            StateAction::Ui(ui_action) => {
                Self::reduce_ui(&mut state.ui, ui_action);
            }
        }

        let current_state = state.clone();
        drop(state);

        let subscribers = self.subscribers.read();
        for subscriber in subscribers.iter() {
            subscriber(&current_state);
        }
    }

    pub fn subscribe(&self, callback: impl Fn(&AppState) + Send + Sync + 'static) {
        self.subscribers.write().push(Box::new(callback));
    }

    pub fn get_state(&self) -> AppState {
        self.state.read().clone()
    }

    fn reduce_navigation(state: &mut NavigationState, action: NavigationAction) {
        match action {
            NavigationAction::NavigateTo(page) => {
                if state.current_page != page {
                    tracing::info!("Navigation: {:?} → {:?}", state.current_page, page);
                    state.history.push(state.current_page.clone());
                    state.current_page = page;
                } else {
                    tracing::debug!("Navigation: Already on {:?}, skipping", page);
                }
            }
            NavigationAction::GoBack => {
                if let Some(page) = state.history.pop() {
                    tracing::info!("Navigation: Going back to {:?}", page);
                    state.current_page = page;
                }
            }
            NavigationAction::ClearHistory => {
                tracing::info!("Navigation: Clearing history");
                state.history.clear();
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
            PhotoAction::LoadPhotosStart => {
                state.is_loading = true;
            }
            PhotoAction::LoadPhotosSuccess(photos) => {
                state.photos = photos;
                state.is_loading = false;
                state.current_index = 0;
            }
            PhotoAction::LoadPhotosFailure => {
                state.is_loading = false;
            }
            PhotoAction::SelectPhoto(index) => {
                if index < state.photos.len() {
                    state.current_index = index;
                }
            }
            PhotoAction::NextPhoto => {
                if !state.photos.is_empty() && state.current_index < state.photos.len() - 1 {
                    state.current_index += 1;
                }
            }
            PhotoAction::PreviousPhoto => {
                if state.current_index > 0 {
                    state.current_index -= 1;
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

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
