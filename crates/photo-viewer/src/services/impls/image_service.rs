use crate::error::Result;
use crate::services::traits::ImageService;
use async_trait::async_trait;
use image::GenericImageView;
use std::path::Path;

pub struct ImageServiceImpl;

impl ImageServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ImageService for ImageServiceImpl {
    async fn load_image(&self, path: &Path) -> Result<Vec<u8>> {
        tracing::debug!("Loading image: {:?}", path);

        // Load image
        let img = image::open(path)?;

        // Convert to RGBA8 for Slint
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        tracing::debug!("Image loaded: {}x{}", width, height);

        // Return raw RGBA bytes
        Ok(rgba.into_raw())
    }

    async fn get_image_dimensions(&self, path: &Path) -> Result<(u32, u32)> {
        // Use image reader to get dimensions without loading full image
        let reader = image::ImageReader::open(path)?;
        let dimensions = reader.into_dimensions()?;
        Ok(dimensions)
    }

    async fn generate_thumbnail(&self, path: &Path, max_size: u32) -> Result<Vec<u8>> {
        tracing::debug!("Generating thumbnail for: {:?}", path);

        // Load image
        let img = image::open(path)?;

        // Calculate thumbnail size maintaining aspect ratio
        let (width, height) = img.dimensions();
        let ratio = width as f32 / height as f32;

        let (thumb_width, thumb_height) = if width > height {
            (max_size, (max_size as f32 / ratio) as u32)
        } else {
            ((max_size as f32 * ratio) as u32, max_size)
        };

        // Resize image
        let thumbnail = img.resize(
            thumb_width,
            thumb_height,
            image::imageops::FilterType::Lanczos3,
        );

        // Convert to RGBA8
        let rgba = thumbnail.to_rgba8();

        tracing::debug!("Thumbnail created: {}x{}", thumb_width, thumb_height);

        Ok(rgba.into_raw())
    }
}

impl Default for ImageServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
