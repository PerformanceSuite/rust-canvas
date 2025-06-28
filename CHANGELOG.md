# Changelog

All notable changes to Ev2 - Audio Control Matrix will be documented in this file.

## [0.2.1] - 2025-06-28

### Major Refactor: Widget Spawning System
- **Removed pending_widget system** - Widgets now spawn directly on single click
- **Right-to-left grid positioning** - Widgets spawn from top-right corner, filling leftward then down
- **Dynamic canvas resizing** - Widgets automatically reposition when canvas is resized
- **Smart collision detection** - No widget overlaps with 0.5px precise spacing
- **Panel-aware spawning** - Widgets spawn in selected panel or main canvas
- **Boundary constraints** - Widgets cannot spawn outside canvas or panel boundaries

### Fixed
- **Gear emoji rendering** - Removed Unicode variant selector causing square artifacts
- **Grid positioning reliability** - Simplified logic for consistent widget placement
- **Canvas resize handling** - Automatic widget repositioning on canvas size changes

### UI/UX Improvements  
- **Removed main canvas selection border** - Cleaner interface
- **Immediate visual feedback** - No two-step placement process
- **Predictable grid behavior** - Consistent right-to-left spawning pattern

### Technical
- **Module organization** - Separated rendering, types, and constants into `src/canvas/` modules
- **Performance optimization** - More efficient positioning algorithms
- **Code cleanup** - Removed unused positioning methods

## [0.2.0] - 2025-06-18

### Added
- Version tracking system with automatic version display
- Version shown in window title and widget palette
- `bump_version.sh` script for easy version management
- Status script now shows current version
- Drag-and-drop from widget palette
- Visual preview when dragging widgets from palette
- Group Panel widget containment system
- Settings Panel with black background and colored border
- Edge snapping for Settings Panels (Left/Right/Top/Bottom)
- Smooth dragging without flickering for Settings Panels
- Group Panel collapse functionality
- Widget visibility management (hidden when panel collapsed)
- Size constraints for Group Panels within Settings Panels

### Fixed
- Settings Panel drag flashing/buggy behavior
- Group Panel collapse affecting parent Settings Panel
- Widget containment in collapsed panels
- Font consistency across interface

### Changed
- Settings Panel now has solid black background (no gradient)
- Removed content text from Settings Panel canvas area
- Group Panels now properly constrain to Settings Panel width

## [0.1.0] - 2025-06-17

### Added
- Initial release
- Basic drag-and-drop canvas
- Widget palette with various controls (Knobs, Toggles, Buttons, etc.)
- Audio control widgets matching React app styling
- macOS app bundle creation
- Custom Ev2 icon and branding