use crate::constants::{MAX_DIRECTORY_DEPTH, SUPPORTED_IMAGE_EXTENSIONS};
use crate::error::{Error, Result};
use crate::services::traits::FileSystemService;
use crate::state::PhotoInfo;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct FileSystemServiceImpl {
    // TODO: Add thread pool for I/O operations
    // pool: Arc<ThreadPool>,
}

impl FileSystemServiceImpl {
    pub fn new() -> Self {
        // TODO: Initialize thread pool
        Self {
            // pool
        }
    }

    fn is_supported_image(path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            SUPPORTED_IMAGE_EXTENSIONS.contains(&ext.as_str())
        } else {
            false
        }
    }

    /// Execute blocking filesystem operation on dedicated thread pool
    async fn execute_blocking<F, T>(_f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T> + Send + 'static,
        T: Send + 'static,
    {
        // TODO: Implement with thread pool, for now just run it inline
        _f()
    }

    /// Internal blocking directory scan implementation
    fn blocking_load_photos(path: &Path) -> Result<Vec<PhotoInfo>> {
        if !path.exists() {
            return Err(Error::InvalidPath(format!(
                "Path does not exist: {:?}",
                path
            )));
        }

        if !path.is_dir() {
            return Err(Error::InvalidPath(format!(
                "Path is not a directory: {:?}",
                path
            )));
        }

        tracing::info!("Loading photos from: {:?}", path);

        let mut photos = Vec::new();

        for entry in WalkDir::new(path)
            .max_depth(MAX_DIRECTORY_DEPTH)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();

            if entry_path.is_file() && Self::is_supported_image(entry_path) {
                let metadata = entry.metadata()?;

                // Get filename, skip if invalid
                let filename = match entry_path.file_name() {
                    Some(name) => name.to_string_lossy().to_string(),
                    None => {
                        tracing::warn!("Skipping file with invalid name: {:?}", entry_path);
                        continue;
                    }
                };

                let photo_info = PhotoInfo {
                    path: entry_path.to_path_buf(),
                    filename,
                    size_bytes: metadata.len(),
                };

                photos.push(photo_info);
            }
        }

        photos.sort_by(|a, b| a.filename.cmp(&b.filename));

        tracing::info!("Found {} photos", photos.len());
        Ok(photos)
    }
}

#[async_trait]
impl FileSystemService for FileSystemServiceImpl {
    async fn browse_directory(&self) -> Result<Option<PathBuf>> {
        tracing::info!("Opening file dialog to browse for directory");

        let result = rfd::AsyncFileDialog::new()
            .set_title("Select Photo Album Folder")
            .pick_folder()
            .await;

        if let Some(folder) = result {
            let path = folder.path().to_path_buf();
            tracing::info!("User selected directory: {:?}", path);
            Ok(Some(path))
        } else {
            tracing::info!("User cancelled directory selection");
            Ok(None)
        }
    }

    async fn load_photos_from_directory(&self, path: &Path) -> Result<Vec<PhotoInfo>> {
        let path = path.to_path_buf();

        // Execute directory scan on thread pool
        Self::execute_blocking(move || Self::blocking_load_photos(&path)).await
    }

    async fn is_valid_directory(&self, path: &Path) -> bool {
        let path = path.to_path_buf();
        path.exists() && path.is_dir()
    }
}

impl Default for FileSystemServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
