use crate::error::Result;
use crate::state::PhotoInfo;
use async_trait::async_trait;
use std::path::{Path, PathBuf};

/// Trait for filesystem operations
#[async_trait]
pub trait FileSystemService: Send + Sync {
    /// Browse for a directory
    async fn browse_directory(&self) -> Result<Option<PathBuf>>;

    /// Load photo information from a directory
    async fn load_photos_from_directory(&self, path: &Path) -> Result<Vec<PhotoInfo>>;

    /// Check if path is a valid directory
    async fn is_valid_directory(&self, path: &Path) -> bool;
}

/// Trait for image operations
#[async_trait]
pub trait ImageService: Send + Sync {
    /// Load image data for display
    async fn load_image(&self, path: &Path) -> Result<Vec<u8>>;
}
