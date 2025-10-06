use crate::constants::GRID_ITEM_SIZE_ESTIMATE;
use crate::constants::{DEFAULT_VIEWPORT_HEIGHT, GRID_GAP};
use crate::constants::{DEFAULT_VIEWPORT_WIDTH, GRID_COLUMNS, OVERSCAN_ROWS};
use crate::error::Result;
use crate::models::{Viewport, VirtualGrid, VirtualGridChange, VirtualGridOptions, VisibilityZone};
use crate::services::ServiceContainer;
use crate::state::{Page, StateAction, Store};
use slint::{ComponentHandle, Weak};
use std::sync::{Arc, Mutex};

/// Grid Page ViewModel
pub struct GridPageManager {
    virtual_grid: Arc<Mutex<VirtualGrid>>,
    ui: Weak<crate::Main>,
    _container: Arc<ServiceContainer>,
    store: Arc<Store>,
}

impl GridPageManager {
    pub async fn new(
        ui: Weak<crate::Main>,
        container: Arc<ServiceContainer>,
        store: Arc<Store>,
    ) -> Result<Self> {
        let main = ui.upgrade().ok_or(crate::error::Error::UiUpgradeFailed)?;
        let grid_store = main.global::<crate::GridPageStore>();

        // Initialize virtual grid with default viewport
        let options = VirtualGridOptions::new(0, GRID_COLUMNS)
            .with_gap(8.0)
            .with_estimate_size(|_| GRID_ITEM_SIZE_ESTIMATE)
            .with_overscan(OVERSCAN_ROWS);

        let viewport = Viewport::new(DEFAULT_VIEWPORT_WIDTH, DEFAULT_VIEWPORT_HEIGHT);
        let virtual_grid = Arc::new(Mutex::new(VirtualGrid::new(options, viewport)));

        // Handle photo clicked - navigate to loupe view
        let store_photo = store.clone();
        grid_store.on_photo_clicked(move |index| {
            tracing::info!("Photo clicked: index={}", index);
            store_photo.dispatch(StateAction::select_photo(index as usize));
            store_photo.dispatch(StateAction::navigate_to(Page::Loupe));
        });

        // Handle back button
        let store_back = store.clone();
        grid_store.on_back_clicked(move || {
            tracing::info!("Back clicked from grid");
            store_back.dispatch(StateAction::navigate_to(Page::Import));
        });

        // Handle reimport button
        let store_reimport = store.clone();
        grid_store.on_reimport_clicked(move || {
            tracing::info!("Reimport clicked from grid");
            store_reimport.dispatch(StateAction::navigate_to(Page::Import));
        });

        // Handle scroll events
        let grid_scroll = virtual_grid.clone();
        let ui_scroll = ui.clone();
        grid_store.on_scroll_changed(move |offset| {
            let mut grid = grid_scroll.lock().unwrap();
            let current_viewport = grid.get_viewport();
            let new_viewport = current_viewport.with_scroll(offset as f64);

            let changes = grid.set_viewport(new_viewport);

            // Update UI with visible items
            Self::sync_visible_items_to_ui(&ui_scroll, &grid);

            // Handle visibility changes
            Self::handle_visibility_changes(changes);
        });

        // Handle zoom events
        let grid_zoom = virtual_grid.clone();
        let ui_zoom = ui.clone();
        grid_store.on_zoom_changed(move |zoom| {
            tracing::info!("Zoom changed to: {:.2}x", zoom);

            let mut grid = grid_zoom.lock().unwrap();
            let current_viewport = grid.get_viewport();
            let new_viewport = current_viewport.with_zoom(zoom as f64);

            let changes = grid.set_viewport(new_viewport);

            // Update UI
            Self::sync_visible_items_to_ui(&ui_zoom, &grid);

            // Update total size
            if let Some(main) = ui_zoom.upgrade() {
                let store = main.global::<crate::GridPageStore>();
                store.set_total_size(grid.get_total_size() as f32);
            }

            // Handle visibility changes
            Self::handle_visibility_changes(changes);
        });

        // Handle viewport size changes
        let grid_viewport = virtual_grid.clone();
        let ui_viewport = ui.clone();
        grid_store.on_viewport_changed(move |width, height| {
            tracing::debug!("Viewport size changed: {}x{}", width, height);

            let mut grid = grid_viewport.lock().unwrap();
            let current_viewport = grid.get_viewport();
            let new_viewport = current_viewport.with_size(width as f64, height as f64);

            grid.set_viewport(new_viewport);
            Self::sync_visible_items_to_ui(&ui_viewport, &grid);
        });

        // Subscribe to state changes to update photo count
        let grid_state = virtual_grid.clone();
        let ui_state = ui.clone();
        store.subscribe(move |state| {
            let photo_count = state.photos.photos.len();

            let mut grid = grid_state.lock().unwrap();
            let current_options = grid.get_options();

            // Only update if photo count changed
            if current_options.count != photo_count {
                tracing::info!("Updating virtual grid with {} photos", photo_count);

                // Create new options with updated count
                let new_options = VirtualGridOptions::new(photo_count, GRID_COLUMNS)
                    .with_gap(GRID_GAP)
                    .with_estimate_size(|_| GRID_ITEM_SIZE_ESTIMATE)
                    .with_overscan(OVERSCAN_ROWS);

                // Recreate grid (we need to add an update method to VirtualGrid)
                // For now, recreate it
                let viewport = grid.get_viewport();
                *grid = VirtualGrid::new(new_options, viewport);

                Self::sync_visible_items_to_ui(&ui_state, &grid);

                // Update total size
                if let Some(main) = ui_state.upgrade() {
                    let store = main.global::<crate::GridPageStore>();
                    store.set_total_size(grid.get_total_size() as f32);
                }
            }
        });

        tracing::info!("GridPageManager initialized with virtual scrolling");

        Ok(Self {
            virtual_grid,
            ui,
            _container: container,
            store,
        })
    }

    /// Sync visible items from VirtualGrid to Slint UI
    fn sync_visible_items_to_ui(ui: &Weak<crate::Main>, grid: &VirtualGrid) {
        if let Some(main) = ui.upgrade() {
            let store = main.global::<crate::GridPageStore>();

            let virtual_items = grid.get_virtual_items();

            // Convert to Slint model
            let slint_items: Vec<crate::VirtualItemData> = virtual_items
                .iter()
                .map(|item| crate::VirtualItemData {
                    index: item.index as i32,
                    start: item.start as f32,
                    width: item.width as f32,
                    height: item.height as f32,
                    row: item.row as i32,
                    column: item.column as i32,
                })
                .collect();

            store.set_visible_items(slint::ModelRc::new(slint::VecModel::from(slint_items)));

            tracing::debug!("Synced {} visible items to UI", virtual_items.len());
        }
    }

    /// Handle visibility changes (for image loading/unloading)
    fn handle_visibility_changes(changes: Vec<VirtualGridChange>) {
        for change in changes {
            match change {
                VirtualGridChange::ItemEntered { item, zone } => {
                    match zone {
                        VisibilityZone::Visible => {
                            tracing::debug!("Item {} entered visible zone", item.index);
                            // TODO: Trigger high-priority image loading
                        }
                        VisibilityZone::Overscan => {
                            tracing::debug!("Item {} entered overscan zone", item.index);
                            // TODO: Trigger low-priority image preloading
                        }
                        _ => {}
                    }
                }
                VirtualGridChange::ItemExited { item, zone } => {
                    tracing::debug!("Item {} exited {:?} zone", item.index, zone);
                    // TODO: Cancel image loading or unload image
                }
                VirtualGridChange::ScrollChanged { offset, direction } => {
                    tracing::debug!(
                        "Scroll changed: offset={:.0}, direction={:?}",
                        offset,
                        direction
                    );
                }
                _ => {}
            }
        }
    }
}
