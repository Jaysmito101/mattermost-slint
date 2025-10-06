/// Rectangle representing dimensions and position
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

/// Viewport represents the visible scrolling area
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// Current viewport dimensions
    pub rect: Rect,

    /// Current scroll offset (vertical for grid)
    pub scroll_offset: f64,

    pub zoom: f64,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            rect: Rect::new(width, height),
            scroll_offset: 0.0,
            zoom: 1.0,
        }
    }

    /// Get the start of the visible area
    pub fn visible_start(&self) -> f64 {
        self.scroll_offset
    }

    /// Get the end of the visible area
    pub fn visible_end(&self) -> f64 {
        self.scroll_offset + self.rect.height
    }

    /// Check if a range intersects with the viewport
    pub fn intersects(&self, start: f64, end: f64) -> bool {
        start < self.visible_end() && end > self.visible_start()
    }

    /// Update viewport size
    pub fn with_size(&self, width: f64, height: f64) -> Self {
        Self {
            rect: Rect::new(width, height),
            ..*self
        }
    }

    /// Update scroll offset
    pub fn with_scroll(&self, offset: f64) -> Self {
        Self {
            scroll_offset: offset.max(0.0),
            ..*self
        }
    }

    /// Update zoom level
    pub fn with_zoom(&self, zoom: f64) -> Self {
        Self {
            zoom: zoom.max(0.1).min(10.0),
            ..*self
        }
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(800.0, 600.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_intersection() {
        let viewport = Viewport::new(800.0, 600.0).with_scroll(100.0);

        // Item fully visible
        assert!(viewport.intersects(200.0, 400.0));

        // Item above viewport
        assert!(!viewport.intersects(0.0, 50.0));

        // Item below viewport
        assert!(!viewport.intersects(800.0, 900.0));

        // Item partially visible (top)
        assert!(viewport.intersects(50.0, 150.0));

        // Item partially visible (bottom)
        assert!(viewport.intersects(600.0, 800.0));
    }
}
