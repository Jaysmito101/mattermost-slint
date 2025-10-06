/// Photo service - business workflows for photo management
use crate::error::{Error, Result};
use crate::services::ServiceContainer;
use crate::state::{Page, StateAction, Store};
use std::path::PathBuf;
use std::sync::Arc;

fn validate_path(path: &PathBuf) -> Result<()> {
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

    Ok(())
}

/// Browse for a directory and load photos from it
pub async fn browse_and_load_photos(
    container: Arc<ServiceContainer>,
    store: Arc<Store>,
) -> Result<()> {
    tracing::info!("Browsing for directory...");

    match container.filesystem().browse_directory().await? {
        Some(path) => {
            tracing::info!("Directory selected: {:?}", path);

            // Update state with selected path
            store.dispatch(StateAction::set_album_path(path.clone()));

            // Load photos from selected directory
            load_photos_from_path(container, store, path).await?;
        }
        None => {
            tracing::info!("No directory selected");
        }
    }

    Ok(())
}

/// Load photos from a specific directory path
pub async fn load_photos_from_path(
    container: Arc<ServiceContainer>,
    store: Arc<Store>,
    path: PathBuf,
) -> Result<()> {
    tracing::info!("Loading photos from: {:?}", path);

    if let Err(e) = validate_path(&path) {
        tracing::warn!("Path validation failed: {:?}", e);
        store.dispatch(StateAction::show_error(format!("Invalid path: {}", e)));
        return Ok(());
    }

    // Show loading state
    store.dispatch(StateAction::load_photos_start());
    store.dispatch(StateAction::show_loading());

    // Call service to load photos
    match container
        .filesystem()
        .load_photos_from_directory(&path)
        .await
    {
        Ok(photos) => {
            tracing::info!("Loaded {} photos", photos.len());

            // Update state with results
            store.dispatch(StateAction::load_photos_success(photos.clone()));
            store.dispatch(StateAction::hide_loading());

            // Navigate based on results
            if !photos.is_empty() {
                store.dispatch(StateAction::navigate_to(Page::Grid));
            } else {
                store.dispatch(StateAction::show_error(
                    "No photos found in the selected directory".to_string(),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to load photos: {:?}", e);

            // Update state with error
            store.dispatch(StateAction::load_photos_failure());
            store.dispatch(StateAction::hide_loading());
            store.dispatch(StateAction::show_error(format!(
                "Failed to load photos: {}",
                e
            )));
        }
    }

    Ok(())
}
