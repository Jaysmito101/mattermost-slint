use crate::error::Result;
use crate::services::ServiceContainer;
use crate::state::{Page, StateAction, Store};
use slint::{ComponentHandle, Weak};
use std::path::PathBuf;
use std::sync::Arc;

pub struct ImportPageManager {
    _container: Arc<ServiceContainer>,
    _store: Arc<Store>,
}

impl ImportPageManager {
    pub async fn new(
        ui: Weak<crate::Main>,
        container: Arc<ServiceContainer>,
        store: Arc<Store>,
    ) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let import_store = main.global::<crate::ImportPageStore>();

        let container_browse = container.clone();
        let store_browse = store.clone();
        import_store.on_browse_clicked(move || {
            let container = container_browse.clone();
            let store = store_browse.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_browse(container, store).await {
                    tracing::error!("Browse failed: {:?}", e);
                }
            });
        });

        let container_load = container.clone();
        let store_load = store.clone();
        import_store.on_load_clicked(move || {
            let container = container_load.clone();
            let store = store_load.clone();

            tokio::spawn(async move {
                let state = store.get_state();
                if let Some(path) = state.photos.album_path {
                    if let Err(e) = Self::load_photos(container, store, path).await {
                        tracing::error!("Failed to load photos: {:?}", e);
                    }
                }
            });
        });

        Ok(Self {
            _container: container,
            _store: store,
        })
    }

    pub async fn handle_browse(container: Arc<ServiceContainer>, store: Arc<Store>) -> Result<()> {
        tracing::info!("Browse button clicked");

        match container.filesystem().browse_directory().await? {
            Some(path) => {
                tracing::info!("Directory selected: {:?}", path);
                store.dispatch(StateAction::navigate_to(Page::Import));
                store.dispatch(StateAction::set_album_path(path.clone()));
                Self::load_photos(container, store, path).await?;
            }
            None => {
                tracing::info!("No directory selected");
            }
        }

        Ok(())
    }

    async fn load_photos(
        container: Arc<ServiceContainer>,
        store: Arc<Store>,
        path: PathBuf,
    ) -> Result<()> {
        tracing::info!("Loading photos from: {:?}", path);

        store.dispatch(StateAction::load_photos_start());
        store.dispatch(StateAction::show_loading());

        match container
            .filesystem()
            .load_photos_from_directory(&path)
            .await
        {
            Ok(photos) => {
                tracing::info!("Loaded {} photos", photos.len());

                store.dispatch(StateAction::load_photos_success(photos.clone()));
                store.dispatch(StateAction::hide_loading());

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
}
