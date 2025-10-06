use std::collections::HashMap;
use std::ops::Range;

use super::{ItemKey, Viewport, VirtualItem};

/// Visibility zone for items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisibilityZone {
    /// Item is in the visible viewport
    Visible,
    /// Item is in the overscan (soft) zone
    Overscan,
    /// Item is outside all zones
    Outside,
}

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Forward,  // Scrolling down/right
    Backward, // Scrolling up/left
}

/// Change event for virtual grid state
#[derive(Debug, Clone)]
pub enum VirtualGridChange {
    /// Item entered a visibility zone
    ItemEntered {
        item: VirtualItem,
        zone: VisibilityZone,
    },
    /// Item exited a visibility zone
    ItemExited {
        item: VirtualItem,
        zone: VisibilityZone,
    },
    /// Scroll position changed
    ScrollChanged {
        offset: f64,
        direction: Option<ScrollDirection>,
    },
    /// Viewport size changed
    ViewportChanged { width: f64, height: f64 },
    /// Zoom level changed
    ZoomChanged { zoom: f64 },
    /// Total size changed
    TotalSizeChanged { size: f64 },
}

/// Range extractor function type
/// Takes a range and returns the actual indices to render
/// Reference: https://tanstack.com/virtual/latest/docs/api/virtualizer#rangeextractor
pub type RangeExtractor = fn(Range<usize>, usize) -> Vec<usize>;

/// Default range extractor - returns all indices in range
pub fn default_range_extractor(range: Range<usize>, _count: usize) -> Vec<usize> {
    range.collect()
}

/// Options for virtual grid configuration
/// Inspired by TanStack Virtual's VirtualizerOptions
/// Reference: https://tanstack.com/virtual/latest/docs/api/virtualizer
pub struct VirtualGridOptions {
    /// Total number of items
    pub count: usize,

    /// Number of columns in the grid
    pub columns: usize,

    /// Gap between items (in pixels)
    pub gap: f64,

    /// Estimate size of each item (before measurement)
    pub estimate_size: Box<dyn Fn(usize) -> f64 + Send + Sync>,

    /// Get custom key for an item (optional)
    pub get_item_key: Option<Box<dyn Fn(usize) -> ItemKey + Send + Sync>>,

    /// Number of items to render above/below visible area (overscan)
    pub overscan: usize,

    /// Custom range extractor
    pub range_extractor: RangeExtractor,

    /// Enable debug logging
    pub debug: bool,
}

impl VirtualGridOptions {
    pub fn new(count: usize, columns: usize) -> Self {
        Self {
            count,
            columns,
            gap: 8.0,
            estimate_size: Box::new(|_| 200.0),
            get_item_key: None,
            overscan: 3,
            range_extractor: default_range_extractor,
            debug: false,
        }
    }

    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_estimate_size<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> f64 + Send + Sync + 'static,
    {
        self.estimate_size = Box::new(f);
        self
    }

    pub fn with_item_key<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> ItemKey + Send + Sync + 'static,
    {
        self.get_item_key = Some(Box::new(f));
        self
    }

    pub fn with_overscan(mut self, overscan: usize) -> Self {
        self.overscan = overscan;
        self
    }

    pub fn with_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
}

/// Core virtual grid implementation
///
/// UI-agnostic virtual scrolling logic for grid layouts.
/// Calculates visible items based on viewport and handles zoom.
///
/// Inspired by TanStack Virtual
/// Reference: https://tanstack.com/virtual/latest/docs/api/virtualizer
pub struct VirtualGrid {
    options: VirtualGridOptions,
    viewport: Viewport,

    // Measured sizes (index -> actual size)
    measured_sizes: HashMap<usize, f64>,

    // Cached virtual items
    cached_items: Vec<VirtualItem>,

    // Previous visible range for change detection
    prev_visible_range: Range<usize>,

    // Previous overscan range
    prev_overscan_range: Range<usize>,

    // Scroll state
    prev_scroll_offset: f64,
    is_scrolling: bool,
    scroll_direction: Option<ScrollDirection>,
}

impl VirtualGrid {
    pub fn new(options: VirtualGridOptions, viewport: Viewport) -> Self {
        let mut grid = Self {
            options,
            viewport,
            measured_sizes: HashMap::new(),
            cached_items: Vec::new(),
            prev_visible_range: 0..0,
            prev_overscan_range: 0..0,
            prev_scroll_offset: 0.0,
            is_scrolling: false,
            scroll_direction: None,
        };

        // Initial calculation
        grid.recalculate();
        grid
    }

    /// Update viewport and recalculate visible items
    pub fn set_viewport(&mut self, viewport: Viewport) -> Vec<VirtualGridChange> {
        let mut changes = Vec::new();

        // Detect size change
        if viewport.rect != self.viewport.rect {
            changes.push(VirtualGridChange::ViewportChanged {
                width: viewport.rect.width,
                height: viewport.rect.height,
            });
        }

        // Detect zoom change
        if (viewport.zoom - self.viewport.zoom).abs() > 0.001 {
            changes.push(VirtualGridChange::ZoomChanged {
                zoom: viewport.zoom,
            });
        }

        // Detect scroll change
        if (viewport.scroll_offset - self.viewport.scroll_offset).abs() > 0.1 {
            let direction = if viewport.scroll_offset > self.prev_scroll_offset {
                Some(ScrollDirection::Forward)
            } else {
                Some(ScrollDirection::Backward)
            };

            changes.push(VirtualGridChange::ScrollChanged {
                offset: viewport.scroll_offset,
                direction,
            });

            self.scroll_direction = direction;
            self.is_scrolling = true;
        }

        self.prev_scroll_offset = viewport.scroll_offset;
        self.viewport = viewport;

        // Recalculate and detect item visibility changes
        let visibility_changes = self.recalculate_with_changes();
        changes.extend(visibility_changes);

        changes
    }

    /// Measure an item's actual size (after rendering)
    pub fn measure_item(&mut self, index: usize, size: f64) -> Vec<VirtualGridChange> {
        if self.measured_sizes.get(&index) == Some(&size) {
            return Vec::new(); // No change
        }

        self.measured_sizes.insert(index, size);

        if self.options.debug {
            tracing::debug!("Measured item {} size: {}", index, size);
        }

        // Recalculate layout
        self.recalculate_with_changes()
    }

    /// Get all virtual items that should be rendered
    pub fn get_virtual_items(&self) -> &[VirtualItem] {
        &self.cached_items
    }

    /// Get indices of visible items (without overscan)
    pub fn get_visible_indices(&self) -> Vec<usize> {
        self.cached_items
            .iter()
            .filter(|item| {
                item.is_visible(self.viewport.visible_start(), self.viewport.visible_end())
            })
            .map(|item| item.index)
            .collect()
    }

    /// Get total scrollable size
    pub fn get_total_size(&self) -> f64 {
        let row_count = (self.options.count + self.options.columns - 1) / self.options.columns;
        let mut total = 0.0;

        for row in 0..row_count {
            let row_height = self.get_row_height(row);
            total += row_height + self.options.gap;
        }

        total - self.options.gap // Remove last gap
    }

    /// Scroll to specific index
    pub fn scroll_to_index(&mut self, index: usize, align: ScrollAlign) -> f64 {
        if index >= self.options.count {
            return self.viewport.scroll_offset;
        }

        let row = index / self.options.columns;
        let row_start = self.get_row_start(row);
        let row_height = self.get_row_height(row);

        let offset = match align {
            ScrollAlign::Start => row_start,
            ScrollAlign::Center => row_start - (self.viewport.rect.height - row_height) / 2.0,
            ScrollAlign::End => row_start - self.viewport.rect.height + row_height,
            ScrollAlign::Auto => {
                // Scroll only if not visible
                if row_start < self.viewport.visible_start() {
                    row_start
                } else if row_start + row_height > self.viewport.visible_end() {
                    row_start - self.viewport.rect.height + row_height
                } else {
                    return self.viewport.scroll_offset; // Already visible
                }
            }
        };

        offset
            .max(0.0)
            .min(self.get_total_size() - self.viewport.rect.height)
    }

    /// Get visibility zone for an item
    pub fn get_item_zone(&self, item: &VirtualItem) -> VisibilityZone {
        let visible_start = self.viewport.visible_start();
        let visible_end = self.viewport.visible_end();

        // Check if in visible area
        if item.is_visible(visible_start, visible_end) {
            return VisibilityZone::Visible;
        }

        // Check if in overscan area
        let overscan_pixels = self.calculate_overscan_pixels();
        let overscan_start = visible_start - overscan_pixels;
        let overscan_end = visible_end + overscan_pixels;

        if item.is_visible(overscan_start, overscan_end) {
            return VisibilityZone::Overscan;
        }

        VisibilityZone::Outside
    }

    /// Reset scroll state (call after scroll ends)
    pub fn reset_scroll_state(&mut self) {
        self.is_scrolling = false;
    }

    /// Get current viewport
    pub fn get_viewport(&self) -> Viewport {
        self.viewport
    }

    /// Get grid options
    pub fn get_options(&self) -> &VirtualGridOptions {
        &self.options
    }

    // Private methods

    fn recalculate(&mut self) {
        self.cached_items.clear();

        let visible_row_range = self.calculate_visible_row_range();
        let overscan_row_range = self.expand_range_with_overscan(visible_row_range.clone());

        // Convert row ranges to item indices
        let visible_range = self.row_range_to_item_range(visible_row_range);
        let overscan_range = self.row_range_to_item_range(overscan_row_range);

        // Use custom range extractor
        let indices = (self.options.range_extractor)(overscan_range.clone(), self.options.count);

        // Generate virtual items
        for &index in &indices {
            if index >= self.options.count {
                continue;
            }

            let row = index / self.options.columns;
            let column = index % self.options.columns;

            let start = self.get_row_start(row);
            let width = self.get_row_width(row);
            let height = self.get_row_height(row);

            let key = if let Some(ref key_fn) = self.options.get_item_key {
                key_fn(index)
            } else {
                ItemKey::from_index(index)
            };

            let item = VirtualItem::new(index, start, width, height, row, column).with_key(key);
            self.cached_items.push(item);
        }

        self.prev_visible_range = visible_range;
        self.prev_overscan_range = overscan_range;

        if self.options.debug {
            tracing::debug!(
                "Recalculated: {} items, visible rows: {:?}",
                self.cached_items.len(),
                self.prev_visible_range
            );
        }
    }

    fn recalculate_with_changes(&mut self) -> Vec<VirtualGridChange> {
        let old_items: HashMap<_, _> = self
            .cached_items
            .iter()
            .map(|item| (item.index, self.get_item_zone(item)))
            .collect();

        let old_total_size = self.get_total_size();

        self.recalculate();

        let mut changes = Vec::new();

        // Detect item visibility changes
        for item in &self.cached_items {
            let new_zone = self.get_item_zone(item);
            let old_zone = old_items.get(&item.index).copied();

            match (old_zone, new_zone) {
                (None, VisibilityZone::Visible)
                | (Some(VisibilityZone::Outside), VisibilityZone::Visible) => {
                    changes.push(VirtualGridChange::ItemEntered {
                        item: item.clone(),
                        zone: VisibilityZone::Visible,
                    });
                }
                (None, VisibilityZone::Overscan)
                | (Some(VisibilityZone::Outside), VisibilityZone::Overscan) => {
                    changes.push(VirtualGridChange::ItemEntered {
                        item: item.clone(),
                        zone: VisibilityZone::Overscan,
                    });
                }
                (Some(VisibilityZone::Visible), VisibilityZone::Outside) => {
                    changes.push(VirtualGridChange::ItemExited {
                        item: item.clone(),
                        zone: VisibilityZone::Visible,
                    });
                }
                (Some(VisibilityZone::Overscan), VisibilityZone::Outside) => {
                    changes.push(VirtualGridChange::ItemExited {
                        item: item.clone(),
                        zone: VisibilityZone::Overscan,
                    });
                }
                _ => {}
            }
        }

        // Check for items that were removed
        for (index, old_zone) in old_items {
            if !self.cached_items.iter().any(|item| item.index == index) {
                // Item was in old but not in new
                if old_zone != VisibilityZone::Outside {
                    // Create a stub item for the exit event
                    let row = index / self.options.columns;
                    let column = index % self.options.columns;
                    let start = self.get_row_start(row);
                    let width = self.get_row_width(row);
                    let height = self.get_row_height(row);
                    let item = VirtualItem::new(index, start, width, height, row, column);

                    changes.push(VirtualGridChange::ItemExited {
                        item,
                        zone: old_zone,
                    });
                }
            }
        }

        // Check total size change
        let new_total_size = self.get_total_size();
        if (new_total_size - old_total_size).abs() > 0.1 {
            changes.push(VirtualGridChange::TotalSizeChanged {
                size: new_total_size,
            });
        }

        changes
    }

    fn calculate_visible_row_range(&self) -> Range<usize> {
        let start = self.viewport.visible_start();
        let end = self.viewport.visible_end();

        let start_row = self.find_row_at_offset(start);
        let end_row = self.find_row_at_offset(end);

        start_row..end_row.saturating_add(1)
    }

    fn expand_range_with_overscan(&self, range: Range<usize>) -> Range<usize> {
        let overscan = self.options.overscan;
        let start = range.start.saturating_sub(overscan);
        let end = (range.end + overscan)
            .min((self.options.count + self.options.columns - 1) / self.options.columns);
        start..end
    }

    fn calculate_overscan_pixels(&self) -> f64 {
        // Calculate average row height for overscan
        let avg_row_height = if self.measured_sizes.is_empty() {
            (self.options.estimate_size)(0)
        } else {
            self.measured_sizes.values().sum::<f64>() / self.measured_sizes.len() as f64
        };

        avg_row_height * self.options.overscan as f64
    }

    fn row_range_to_item_range(&self, row_range: Range<usize>) -> Range<usize> {
        let start = row_range.start * self.options.columns;
        let end = (row_range.end * self.options.columns).min(self.options.count);
        start..end
    }

    fn find_row_at_offset(&self, offset: f64) -> usize {
        let row_count = (self.options.count + self.options.columns - 1) / self.options.columns;
        let mut current_offset = 0.0;

        for row in 0..row_count {
            let row_height = self.get_row_height(row);
            if current_offset + row_height > offset {
                return row;
            }
            current_offset += row_height + self.options.gap;
        }

        row_count.saturating_sub(1)
    }

    fn get_row_start(&self, row: usize) -> f64 {
        let mut offset = 0.0;
        for r in 0..row {
            offset += self.get_row_height(r) + self.options.gap;
        }
        offset
    }

    fn get_row_width(&self, row: usize) -> f64 {
        let start_index = row * self.options.columns;
        let end_index = (start_index + self.options.columns).min(self.options.count);

        let mut max_width: f64 = 0.0;
        for index in start_index..end_index {
            let width = self
                .measured_sizes
                .get(&index)
                .copied()
                .unwrap_or_else(|| (self.options.estimate_size)(index));
            let zoomed_width = width * self.viewport.zoom;
            max_width = max_width.max(zoomed_width);
        }

        max_width
    }

    fn get_row_height(&self, row: usize) -> f64 {
        // Find the tallest item in this row
        let start_index = row * self.options.columns;
        let end_index = (start_index + self.options.columns).min(self.options.count);

        let mut max_height: f64 = 0.0;
        for index in start_index..end_index {
            let height = self
                .measured_sizes
                .get(&index)
                .copied()
                .unwrap_or_else(|| (self.options.estimate_size)(index));

            // Apply zoom
            let zoomed_height = height * self.viewport.zoom;
            max_height = max_height.max(zoomed_height);
        }

        max_height
    }
}

/// Scroll alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAlign {
    Start,  // Align to top
    Center, // Center in viewport
    End,    // Align to bottom
    Auto,   // Scroll only if not visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_grid_basic() {
        let options = VirtualGridOptions::new(100, 4);
        let viewport = Viewport::new(800.0, 600.0);
        let grid = VirtualGrid::new(options, viewport);

        let items = grid.get_virtual_items();
        assert!(!items.is_empty());

        let total_size = grid.get_total_size();
        assert!(total_size > 0.0);
    }

    #[test]
    fn test_virtual_grid_zoom() {
        let options = VirtualGridOptions::new(100, 4);
        let viewport = Viewport::new(800.0, 600.0);
        let mut grid = VirtualGrid::new(options, viewport);

        let size_1x = grid.get_total_size();

        // Zoom to 2x
        let viewport_2x = viewport.with_zoom(2.0);
        grid.set_viewport(viewport_2x);

        let size_2x = grid.get_total_size();

        // Size should roughly double with zoom
        assert!(size_2x > size_1x * 1.8);
    }

    #[test]
    fn test_visibility_detection() {
        let options = VirtualGridOptions::new(100, 4).with_overscan(2);
        let viewport = Viewport::new(800.0, 600.0);
        let grid = VirtualGrid::new(options, viewport);

        let visible = grid.get_visible_indices();
        assert!(!visible.is_empty());

        // All rendered items should be in visible or overscan
        for item in grid.get_virtual_items() {
            let zone = grid.get_item_zone(item);
            assert!(zone != VisibilityZone::Outside);
        }
    }
}
