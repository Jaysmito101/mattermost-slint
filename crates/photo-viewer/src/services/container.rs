use super::impls::*;
use super::traits::*;
use crate::error::Result;
use crate::state::Store;
use std::sync::Arc;

/// Service container for dependency injection
#[derive(Clone)]
pub struct ServiceContainer {
    filesystem: Arc<dyn FileSystemService>,
    image: Arc<dyn ImageService>,
    store: Arc<Store>,
}

impl ServiceContainer {
    /// Create a new service container
    pub fn new(store: Arc<Store>) -> Result<Self> {
        let filesystem = Arc::new(FileSystemServiceImpl::new());
        let image = Arc::new(ImageServiceImpl::new());

        Ok(Self {
            filesystem,
            image,
            store,
        })
    }

    // Service accessors
    pub fn filesystem(&self) -> Arc<dyn FileSystemService> {
        self.filesystem.clone()
    }

    pub fn image(&self) -> Arc<dyn ImageService> {
        self.image.clone()
    }

    pub fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
}
