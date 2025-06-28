# Claude Memory for egui-test Project

## Project Overview
This is an egui-based Rust application called "Ev2" that implements a drag-and-drop audio control matrix. The app allows users to create various audio control widgets (knobs, sliders, buttons, etc.) and organize them in panels or on the main canvas.

## Current Version
- Version: 0.2.1 (tracked in Cargo.toml)
- Version display: Shows in window title and widget palette
- Version management: Uses bump_version.sh script and CHANGELOG.md

## Core Files
- `src/drag_drop_canvas.rs` - Main implementation of the widget system and canvas
- `src/app.rs` - Main application structure and UI layout
- `src/main.rs` - Entry point
- `src/audio_controls.rs` - Audio control widget implementations
- `src/canvas/` - Module organization for canvas-related code
  - `src/canvas/widgets/rendering.rs` - Widget rendering implementations
  - `src/canvas/widgets/types.rs` - Widget type definitions
  - `src/canvas/constants.rs` - Color and layout constants
  - `src/canvas/panels.rs` - Panel management logic

## ✅ RESOLVED: Widget Spawning System Refactor
The widget spawning system has been completely refactored for single-click direct placement.

### Major Changes
1. **Removed pending_widget system**: Widgets now spawn directly on click without two-step process
2. **Right-to-left grid positioning**: Widgets spawn from top-right corner, filling leftward then down
3. **Dynamic canvas resizing**: Widgets automatically reposition when canvas is resized
4. **Fixed emoji rendering**: Removed Unicode variant selector from gear emoji that was causing square artifacts
5. **Improved spacing**: Consistent 0.5px spacing between widgets with proper boundary constraints

### Current Widget Placement System
1. **Direct Spawning**: Click any widget in palette to spawn it immediately
2. **Smart Positioning**: Automatic grid placement starting from top-right
3. **Panel Awareness**: Widgets spawn in selected panel (cyan highlight) or main canvas
4. **No Overlaps**: Collision detection ensures widgets don't overlap (except inside panels)
5. **Boundary Constraints**: Widgets cannot spawn outside canvas or panel boundaries

### Key Implementation Details
- Grid positioning uses `find_next_canvas_position()` and `find_next_panel_position()`
- Canvas resize detection triggers `reposition_canvas_widgets()`
- Removed main canvas yellow selection border for cleaner UI
- Simplified logic for more reliable widget placement

## Development Commands
- `cargo run` - Run the application
- `./bump_version.sh` - Increment version
- `./update_system_app.sh` - Update system installation

## Repository
- GitHub: https://github.com/PerformanceSuite/rust-canvas

## Current Status
✅ Single-click widget spawning fully functional
✅ Right-to-left grid positioning working correctly
✅ Dynamic canvas resizing with automatic repositioning
✅ Gear emoji rendering issue fixed
✅ Clean, predictable widget placement system