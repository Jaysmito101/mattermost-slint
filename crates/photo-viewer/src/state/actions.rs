use super::{Page, PhotoInfo};
use std::path::PathBuf;

/// Root action type
#[derive(Clone, Debug)]
pub enum StateAction {
    Navigation(NavigationAction),
    Photos(PhotoAction),
    Ui(UiAction),
}

/// Navigation actions
#[derive(Clone, Debug)]
pub enum NavigationAction {
    NavigateTo(Page),
}

/// Photo actions
#[derive(Clone, Debug)]
pub enum PhotoAction {
    SetAlbumPath(PathBuf),
    LoadPhotosStart,
    LoadPhotosSuccess(Vec<PhotoInfo>),
    LoadPhotosFailure,
    SelectPhoto(usize),
    NextPhoto,
    PreviousPhoto,
    ClearAlbum,
}

/// UI actions
#[derive(Clone, Debug)]
pub enum UiAction {
    ShowLoading,
    HideLoading,
    ShowError(String),
    ClearError,
}

// Convenience constructors
impl StateAction {
    pub fn navigate_to(page: Page) -> Self {
        StateAction::Navigation(NavigationAction::NavigateTo(page))
    }

    pub fn set_album_path(path: PathBuf) -> Self {
        StateAction::Photos(PhotoAction::SetAlbumPath(path))
    }

    pub fn load_photos_start() -> Self {
        StateAction::Photos(PhotoAction::LoadPhotosStart)
    }

    pub fn load_photos_success(photos: Vec<PhotoInfo>) -> Self {
        StateAction::Photos(PhotoAction::LoadPhotosSuccess(photos))
    }

    pub fn load_photos_failure() -> Self {
        StateAction::Photos(PhotoAction::LoadPhotosFailure)
    }

    pub fn select_photo(index: usize) -> Self {
        StateAction::Photos(PhotoAction::SelectPhoto(index))
    }

    pub fn next_photo() -> Self {
        StateAction::Photos(PhotoAction::NextPhoto)
    }

    pub fn previous_photo() -> Self {
        StateAction::Photos(PhotoAction::PreviousPhoto)
    }

    pub fn show_loading() -> Self {
        StateAction::Ui(UiAction::ShowLoading)
    }

    pub fn hide_loading() -> Self {
        StateAction::Ui(UiAction::HideLoading)
    }

    pub fn show_error(message: String) -> Self {
        StateAction::Ui(UiAction::ShowError(message))
    }

    pub fn clear_error() -> Self {
        StateAction::Ui(UiAction::ClearError)
    }
}
