# Changelog

All notable changes to Ev2 - Audio Control Matrix will be documented in this file.

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