use std::fmt;

/// Unique identifier for an item
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemKey(String);

impl ItemKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    pub fn from_index(index: usize) -> Self {
        Self(index.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<usize> for ItemKey {
    fn from(index: usize) -> Self {
        Self::from_index(index)
    }
}

impl From<String> for ItemKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for ItemKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a single virtual item in the grid
///
/// Inspired by TanStack Virtual's VirtualItem
/// Reference: https://tanstack.com/virtual/latest/docs/api/virtual-item
#[derive(Debug, Clone)]
pub struct VirtualItem {
    /// Unique key for this item
    pub key: ItemKey,

    /// Index in the original data array
    pub index: usize,

    /// Start position (top for vertical scrolling)
    pub start: f64,

    pub width: f64,

    pub height: f64,

    /// Row index in the grid layout
    pub row: usize,

    /// Column index in the grid layout
    pub column: usize,

    /// Lane (for masonry/variable height layouts)
    pub lane: usize,
}

impl VirtualItem {
    pub fn new(
        index: usize,
        start: f64,
        width: f64,
        height: f64,
        row: usize,
        column: usize,
    ) -> Self {
        Self {
            key: ItemKey::from_index(index),
            index,
            start,
            width,
            height,
            row,
            column,
            lane: 0,
        }
    }

    /// Create with custom key
    pub fn with_key(mut self, key: ItemKey) -> Self {
        self.key = key;
        self
    }

    /// Create with lane
    pub fn with_lane(mut self, lane: usize) -> Self {
        self.lane = lane;
        self
    }

    /// Check if item is visible in viewport
    pub fn is_visible(&self, viewport_start: f64, viewport_end: f64) -> bool {
        self.start < viewport_end && self.start + self.height > viewport_start
    }

    /// Get the center position of this item
    pub fn center(&self) -> f64 {
        self.start + self.height / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_item_visibility() {
        let item = VirtualItem::new(0, 100.0, 50.0, 50.0, 0, 0);

        // Item fully visible
        assert!(item.is_visible(0.0, 200.0));

        // Item above viewport
        assert!(!item.is_visible(200.0, 400.0));

        // Item below viewport
        assert!(!item.is_visible(0.0, 50.0));

        // Item partially visible
        assert!(item.is_visible(120.0, 200.0));
    }

    #[test]
    fn test_item_key() {
        let key1 = ItemKey::from_index(5);
        let key2 = ItemKey::new("custom-key");

        assert_eq!(key1.as_str(), "5");
        assert_eq!(key2.as_str(), "custom-key");
    }
}
