use super::impls::*;
use super::traits::*;
use std::sync::Arc;

/// Service container for dependency injection
#[derive(Clone)]
pub struct ServiceContainer {
    filesystem: Arc<dyn FileSystemService>,
    image: Arc<dyn ImageService>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        let filesystem = Arc::new(FileSystemServiceImpl::new());
        let image = Arc::new(ImageServiceImpl::new());

        Self { filesystem, image }
    }

    // Service accessors
    pub fn filesystem(&self) -> Arc<dyn FileSystemService> {
        self.filesystem.clone()
    }

    pub fn image(&self) -> Arc<dyn ImageService> {
        self.image.clone()
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}
