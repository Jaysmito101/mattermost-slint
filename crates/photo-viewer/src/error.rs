use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Image Error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("Slint Error: {0}")]
    SlintError(#[from] slint::PlatformError),

    #[error("WalkDir Error: {0}")]
    WalkDirError(#[from] walkdir::Error),

    #[error("UI Upgrade Failed")]
    UiUpgradeFailed,

    #[error("Invalid Path: {0}")]
    InvalidPath(String),

    #[error("{0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, Error>;
