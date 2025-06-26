# Panel Selection and Widget Placement System Requirements

## Context
You are working on an egui-based Rust application that implements a drag-and-drop audio control matrix. The app has a widget palette on the left side and a main canvas area. Users can create panels (containers) and place widgets either in panels or on the main canvas.

## Current Issues
1. Widgets always spawn in the last added panel instead of respecting user selection
2. Panel selection system exists but doesn't control widget placement
3. Drag and drop functionality breaks for widgets inside group panels
4. Confusing test buttons that don't serve the intended workflow

## Required Functional Behavior

### Panel Selection System
1. **Visual Feedback**: When a panel is selected, it should have a visible highlight border (cyan color)
2. **Main Canvas Selection**: When no panel is selected, the main canvas should have a highlight border (yellow color)
3. **Selection Persistence**: Panel selection should persist until explicitly changed by user action
4. **Selection Methods**: Users can select panels by clicking on them

### Widget Placement Workflow
1. **Primary Workflow**: Click widget from palette → Click where to place it
   - If a panel is selected: Widget goes inside that panel automatically
   - If no panel selected: Widget goes on main canvas at clicked location
   - Panel selection is preserved after widget placement

2. **Alternative Workflow**: Click widget from palette → Click directly on a panel
   - Widget goes inside the clicked panel
   - That panel becomes the new selected panel

3. **Deselection**: Clicking on empty canvas space (with no pending widget) deselects any selected panel

### Panel Types and Behavior
1. **Settings Panel**: Can be minimized/expanded, snaps to canvas edges
2. **Group Panel**: Can be collapsed/expanded, widgets inside are hidden when collapsed
3. **Regular Panel**: Basic container functionality

### Widget Containment Rules
1. **Contained Widgets**: When placed inside a panel, widgets are added to that panel's `contained_widgets` list
2. **Position Management**: Widgets inside panels have positions relative to the panel's coordinate system
3. **Visibility**: Widgets inside collapsed/minimized panels are hidden but remain in the widgets list
4. **Drag and Drop**: Widgets inside panels should remain draggable when the panel is open

### Key Data Structures
```rust
pub struct DragDropCanvas {
    pub selected_panel: Option<usize>, // ID of currently selected panel
    pub pending_widget: Option<WidgetType>, // Widget selected from palette waiting for placement
    pub widgets: Vec<DraggableWidget>, // All widgets including panels
    // ... other fields
}

pub enum WidgetType {
    Panel { 
        title: String, 
        collapsed: bool, 
        is_group: bool, 
        contained_widgets: Vec<usize>, // IDs of widgets inside this panel
        // ... other fields 
    },
    SettingsPanel { 
        title: String, 
        minimized: bool, 
        contained_widgets: Vec<usize>,
        // ... other fields 
    },
    // ... other widget types
}
```

### Specific Implementation Requirements

1. **Palette Button Behavior**: 
   - Clicking a widget button sets `pending_widget` (does NOT immediately place widget)
   - Shows message like "→ Knob widget selected from palette - click to place"

2. **Widget Placement Logic in `add_widget()`**:
   - First priority: Use `selected_panel` if it exists and is valid (not collapsed/minimized)
   - Second priority: Place on main canvas
   - NO fallback to "last available panel" - this was the core bug

3. **Click Handling Logic**:
   - Clicking on panel WITH pending widget: Place widget in that panel, keep panel selected
   - Clicking on panel WITHOUT pending widget: Select that panel for future placements
   - Clicking on empty space WITH pending widget: Place according to current selection
   - Clicking on empty space WITHOUT pending widget: Deselect panel

4. **Visual Highlighting**:
   - Selected panel: 3px cyan border around the panel
   - Main canvas (when no panel selected): 2px yellow border around canvas area
   - Update highlighting every frame in the render loop

### Error Cases to Handle
1. **Invalid Selection**: If selected panel ID doesn't exist or panel is collapsed/minimized, fall back to canvas placement
2. **Widget Overlap**: When placing on canvas, offset position if it would overlap existing widgets
3. **Borrowing Issues**: Use deferred placement pattern to avoid mutable borrow conflicts during iteration

### Success Criteria
1. User can select a panel and all subsequent widgets from palette go into that panel
2. User can deselect by clicking empty canvas and widgets go on main canvas
3. Panel selection is visually clear with highlighting
4. Widgets inside group panels can be dragged when panel is open
5. No confusing test buttons or fallback behaviors
6. Workflow is intuitive: select where → select what → place

### Testing Workflow
1. Add a Settings Panel
2. Select it (should show cyan highlight)
3. Click knob from palette, then click anywhere → knob goes in settings panel
4. Click toggle from palette, then click anywhere → toggle goes in settings panel  
5. Click empty canvas to deselect (should show yellow canvas highlight)
6. Click slider from palette, then click anywhere → slider goes on main canvas
7. Add a Group Panel, place widgets inside, verify they can be dragged when panel is open

---

This document provides clear, specific requirements for implementing a robust panel selection and widget placement system that respects user intent and provides intuitive workflow.