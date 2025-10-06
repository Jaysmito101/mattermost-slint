# Photo Viewer

A modern photo viewer application built with Slint UI and Rust, using the same scalable architecture as the Mattermost client.

##  Overview 
  - Redux-like state management
  - Async/await service layer
  - Clean MVVM pattern
  - Separated view and model layers

## Quick Start

```bash
# Run the photo viewer
cargo run -p photo-viewer

# Build for release
cargo build --release -p photo-viewer

# Run the binary
./target/release/photo-viewer
```

## Project Structure

```
crates/photo-viewer/
├── src/
│   ├── main.rs           # Entry point
│   ├── lib.rs            # App initialization
│   ├── error.rs          # Error types
|   |── constants.rs      # App constants
│   │
│   ├── state/            # State management (Redux-like)
│   │   ├── mod.rs        # Store, reducers
│   │   └── actions.rs    # State actions
│   │
│   ├── services/         # Service layer
│   │   ├── traits.rs     # Service trait definitions
│   │   ├── container.rs  # DI container
│   │   └── impls/        # Service implementations
│   │       ├── filesystem.rs  # File browsing, loading
│   │       └── image_service.rs  # Image loading, thumbnails
│   │
│   ├── viewmodels/       # Business logic (MVVM)
│   │   ├── mod.rs
│   │   └── import_page.rs
│   │
│   ├── router.rs         # Navigation router
│   └── bridge.rs         # UI Bridge (state → UI sync)
│
└── ui/                   # Slint UI files
    ├── main.slint        # Main window
    ├── store.slint       # Global state definitions
    └── pages/
        ├── welcome.slint
        ├── import.slint
        ├── grid.slint
        └── loupe.slint
```

## Architecture

### **Separation of Concerns**

#### **View Layer (Slint UI)**
- Pure declarative UI
- No business logic
- Receives state from UI Bridge
- Sends events to ViewModels

#### **Model Layer (Rust)**
- **State**: Centralized in Store
- **Services**: File system, image loading
- **ViewModels**: Orchestrate services, dispatch actions
- **Router**: Handle navigation
- **UI Bridge**: Sync state → UI (one-way data flow)

### **Data Flow**

```
User Action (Slint)
    ↓
ViewModel (async handler)
    ↓
Service call (filesystem, image)
    ↓
Dispatch action
    ↓
Store (reducer updates state)
    ↓
UI Bridge (subscribes to state)
    ↓
Slint UI updates
```

## Key Concepts

### 1. **State Management**

All application state lives in one place:

```rust
pub struct AppState {
    pub navigation: NavigationState,  // Current page, history
    pub photos: PhotoState,           // Album, photos list
    pub ui: UiState,                  // Loading, errors
}
```

Update state by dispatching actions:

```rust
// Navigate to grid
store.dispatch(StateAction::navigate_to(Page::Grid));

// Load photos
store.dispatch(StateAction::load_photos_success(photos));

// Show error
store.dispatch(StateAction::show_error("Error message".into()));
```

### 2. **Services**

Async services handle external interactions:

```rust
// File system service
let photos = container.filesystem()
    .load_photos_from_directory(&path)
    .await?;

// Image service
let dimensions = container.image()
    .get_image_dimensions(&path)
    .await?;
```

### 3. **ViewModels**

Orchestrate services and dispatch actions:

```rust
async fn load_photos(
    container: Arc<ServiceContainer>,
    store: Arc<Store>,
    path: PathBuf,
) -> Result<()> {
    store.dispatch(StateAction::show_loading());
    
    let photos = container.filesystem()
        .load_photos_from_directory(&path)
        .await?;
    
    store.dispatch(StateAction::load_photos_success(photos));
    store.dispatch(StateAction::hide_loading());
    
    Ok(())
}
```

### 4. **UI Bridge**

Single point where state syncs to UI:

```rust
store.subscribe(|state| {
    // Automatically sync state to Slint UI
    sync_state_to_ui(&ui, state);
});
```

## 🔧 Adding New Features

### Example: Add a "Slideshow" Feature

**1. Add state:**
```rust
// In state/mod.rs
pub struct UiState {
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub is_slideshow: bool,  // ← New
}
```

**2. Add action:**
```rust
// In state/actions.rs
pub enum UiAction {
    ShowLoading,
    HideLoading,
    StartSlideshow,  // ← New
    StopSlideshow,   // ← New
}
```

**3. Add reducer:**
```rust
// In state/mod.rs
fn reduce_ui(state: &mut UiState, action: UiAction) {
    match action {
        UiAction::StartSlideshow => state.is_slideshow = true,
        UiAction::StopSlideshow => state.is_slideshow = false,
        // ...
    }
}
```

**4. Update UI Bridge:**
```rust
// In bridge.rs
nav_store.set_is_slideshow(state.ui.is_slideshow);
```

**5. Add UI controls:**
```slint
// In pages/loupe.slint
Button {
    text: "Start Slideshow";
    clicked => { /* dispatch action */ }
}
```


## Implementation Details

### View vs Model Separation

**View (Slint)**:
- `ui/` directory
- Pure declarative UI
- No logic, just presentation
- State is read-only

**Model (Rust)**:
- `src/` directory
- All business logic
- State management
- Service calls
- Pure functions

## Learning Path

1. Read the state definitions in `src/state/`
2. Study the service traits in `src/services/traits.rs`
3. Look at ViewModels in `src/viewmodels/`
4. Understand UI Bridge in `src/bridge.rs`
5. See how it all connects in `src/lib.rs`