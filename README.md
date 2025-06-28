# 🎛️ Ev2 - Audio Control Matrix

A professional Rust/egui implementation of the React audio control interface with full drag-and-drop functionality. Now available as a native macOS application with custom "Ev2" icon and dock integration.

## 📱 Native App (Recommended)

### Build & Install
```bash
./build_app.sh    # Creates Ev2.app bundle
./install_app.sh  # Installs to /Applications/
```

### Launch Methods
```bash
# Double-click Ev2.app in Finder
open Ev2.app           # Command line
./launch_ev2.sh        # Launch script
# Or find "Ev2" in Spotlight
```

## 🔨 Development Mode

### Quick Start
```bash
./start.sh      # Auto-detects app bundle or runs dev mode
./run_app.sh    # Start with feature description
./stop.sh       # Force stop any running instances
cargo run       # Manual development mode
```

## ✨ Features

### Widget Library
- **🎛️ Knobs** - Smooth delta-based mouse tracking
- **🔘 Toggle Switches** - With glow effects (like React app)
- **🔳 Push Buttons** - Interactive button controls
- **📊 VU Meters** - Real-time level indicators
- **━ Horizontal/Vertical Sliders** - Precise value controls
- **▭▭▭ Level Indicators** - Multi-segment displays
- **🏷️ Text Labels** - Customizable text
- **📦 Panels** - Resizable containers with gradient backgrounds
- **📁 Group Panels** - Collapsible panels for organizing widgets with nested behavior
- **📊 Status Bar** - Resizable system statistics display
- **Icon Buttons** - Power, Play, Pause, Settings, Mic, etc.

### Interaction Features
- **Single-Click Widget Spawning** - Click any widget in palette to spawn immediately (no two-step process)
- **Right-to-Left Grid Positioning** - Widgets spawn from top-right corner, filling leftward then down
- **Dynamic Canvas Resizing** - Widgets automatically reposition when canvas is resized
- **Smart Collision Detection** - Widgets won't overlap when placed automatically
- **Panel-Aware Spawning** - Widgets spawn in selected panel (cyan highlight) or main canvas
- **Drag & Drop** - Drag widgets from palette to canvas for manual positioning
- **Smart Settings Panel** - Full-edge panels with minimized/expanded states
- **Edge Snapping** - Settings panels automatically snap to closest canvas edge and occupy entire edge
- **Full Edge Occupation** - Side panels (Left/Right) stretch full height, Top/Bottom panels stretch full width
- **Minimize/Expand** - Click settings panel icon to expand, click minimize button (−) to collapse
- **Edge-Specific Resize** - Side panels resize width, Top/Bottom panels resize height
- **Edge Indicators** - Color-coded lines show which edge the panel is snapped to
- **Custom Widget Organization** - Organize widgets within Settings Panels
- **Group Panel Collapse** - Click Group Panels to collapse/expand and hide contained widgets
- **Nested Panel Behavior** - Panels can contain other panels for complex organization
- **Layout Management** - Save Layout and Clear Canvas buttons
- **Panel & Status Bar Resizing** - Click and drag the diagonal lines in corners
- **Alignment Guides** - Pink lines for canvas center, yellow for widget alignment
- **Right-click Editing** - Edit widget properties
- **Interactive Controls** - Click toggles, adjust knobs/sliders
- **0.5px Precise Spacing** - Consistent spacing between widgets with boundary constraints
- **No Visual Borders** - Clean interface without distracting borders

### Styling
- **Exact React Colors** - Matches the original React app color palette
- **Transparent Icon Buttons** - All icon buttons have transparent backgrounds
- **Gradient Backgrounds** - Panels have subtle color gradients
- **Glow Effects** - Toggle switches glow when active
- **Dark Theme** - Black canvas background matching React app

## 🎮 How to Use

1. **Start the app** using `./start.sh`
2. **Single-Click Widget Spawning** - Click any widget in the palette to spawn it immediately:
   - **Smart Grid Placement**: Widgets automatically position in right-to-left grid starting from top-right
   - **No Overlaps**: Collision detection ensures widgets don't overlap
   - **Dynamic Resizing**: Canvas resize automatically repositions widgets to maintain grid
3. **Panel Selection** - Click any panel to select it (cyan highlight) for targeted widget placement
4. **Add Settings Panel** by clicking "📜 Settings Panel" in the left widget palette
5. **Drag Settings Panel** toward any edge (Left/Right/Top/Bottom) - it will automatically snap and occupy the entire edge
6. **Full Edge Panels** - Once snapped:
   - **Side panels** (Left/Right) stretch full canvas height
   - **Top/Bottom panels** stretch full canvas width
7. **Minimize/Expand** - Click the settings icon (⚙) to expand, click the minimize button (−) to collapse
8. **Resize Panels** - Drag the resize handle to adjust:
   - **Side panels**: Adjust width (height is always full)
   - **Top/Bottom panels**: Adjust height (width is always full)
9. **Edge Indicators** - Watch for colored lines showing which edge the panel is snapped to:
   - **Cyan** = Left edge
   - **Pink** = Right edge  
   - **Green** = Top edge
   - **Yellow** = Bottom edge
10. **Smart Widget Placement** - When adding widgets:
    - **Panel Selected**: Widgets spawn inside the selected panel automatically
    - **No Panel Selected**: Widgets spawn on main canvas using grid positioning
11. **Group Organization** - Use Group Panels (📁) for collapsible widget organization:
    - **▼ Expanded**: Shows all contained widgets
    - **▶ Collapsed**: Hides contained widgets, shows only title
    - **Click title**: Toggle collapse/expand state
    - **Widget count**: Shows number of organized widgets
12. **Manual Positioning** - Drag widgets around the canvas for custom positioning
13. **Resize Panels & Status Bars** by dragging the corner handles (diagonal lines)
14. **Edit Properties** by right-clicking on widgets
15. **Interact** with controls - click toggles, drag knobs, adjust sliders
16. **Use Alignment** - pink/yellow guide lines appear when dragging
17. **Save Your Layout** - Click "💾 Save Layout" to preserve your setup
18. **Clear Canvas** - Click "🗑️ Clear Canvas" to remove all widgets

## 🛠️ Development

```bash
cargo build     # Build the project
cargo run       # Run the application
cargo test      # Run tests (if any)
```

## 📁 Project Structure

- `src/drag_drop_canvas.rs` - Main widget system and canvas
- `src/audio_controls.rs` - Original audio control widgets
- `src/app.rs` - Main application window and layout
- `src/lib.rs` - Library entry point
- `start.sh` - Easy start script
- `stop.sh` - Force stop script
- `run_app.sh` - Start with feature info

## 🔍 App Features

### 👍 Dock Integration
- **Custom "Ev2" Icon** - Professional app icon with "Ev2" branding
- **Keep in Dock** - Right-click dock icon → Options → Keep in Dock
- **Spotlight Search** - Type "Ev2" to find and launch
- **Applications Folder** - Install with `./install_app.sh`

### 🔧 Easy Management
```bash
./build_app.sh     # Build native app bundle
./install_app.sh   # Install to Applications
./launch_ev2.sh    # Quick launch script
./start.sh         # Auto-detect app or dev mode
./stop.sh          # Force stop all instances
```

---

**🎛️ Controls:** Drag widgets from palette → position on canvas → resize panels → right-click to edit

**🔄 Updates:** Run `./build_app.sh` after making code changes to rebuild the app bundle