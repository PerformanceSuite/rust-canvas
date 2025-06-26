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

## ✅ RESOLVED: Panel Selection System
The panel selection and widget placement system has been successfully implemented and is working correctly.

### What Was Fixed
1. **Panel Selection State**: Added `selected_panel: Option<usize>` and `pending_widget: Option<WidgetType>` fields to track user selection
2. **Widget Placement Logic**: 
   - Palette buttons now set `pending_widget` instead of immediately placing widgets
   - Click handling properly respects panel selection
   - Drag-and-drop from palette also respects selection
3. **Visual Feedback**: 
   - Cyan highlighting for selected panels
   - Yellow highlighting for main canvas when no panel selected
4. **Widget Rendering**: Fixed widgets inside panels not appearing by properly transforming their coordinates from relative to absolute positions
5. **Boundary Constraints**: Fixed settings panel from moving outside window bounds

### Current Workflow (Working)
1. **Panel Selection**: Click any panel to select it (cyan highlight appears)
2. **Widget Selection**: Click a widget button in the palette
3. **Widget Placement**: Click anywhere to place the widget in the selected panel
4. **Canvas Placement**: Click empty space to deselect panels, then widgets go on main canvas (yellow highlight)
5. **Drag Alternative**: Drag widgets from palette directly onto panels

### Key Implementation Details
- Panel selection preserved until explicitly changed
- No fallback logic - widgets only go where user specifies
- Both click-to-place and drag-to-place workflows supported
- Visual feedback matches requirements in PANEL_SELECTION_REQUIREMENTS.md

## Development Commands
- `cargo run` - Run the application
- `./bump_version.sh` - Increment version
- `./update_system_app.sh` - Update system installation

## Current Status
✅ Panel selection system fully functional
✅ Widget placement respecting user intent
✅ Visual feedback working correctly
✅ All requirements from PANEL_SELECTION_REQUIREMENTS.md implemented