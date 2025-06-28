//! # Drag and Drop Canvas
//! 
//! A comprehensive drag-and-drop audio control widget system built with egui.
//! 
//! This module provides a complete interface for creating, positioning, and managing
//! interactive audio control widgets in a canvas environment. It supports:
//! 
//! - **Multiple Widget Types**: Knobs, sliders, buttons, panels, VU meters, etc.
//! - **Panel Containment**: Widgets can be organized within collapsible panels
//! - **Smart Positioning**: Automatic grid layout with manual positioning override
//! - **Visual Feedback**: Alignment guides and drag hover effects
//! - **Nested Panels**: Panels can contain other panels for complex layouts
//! 
//! ## Example Usage
//! 
//! ```rust
//! let mut canvas = DragDropCanvas::new();
//! 
//! // Add a panel
//! canvas.add_widget(WidgetType::Panel {
//!     title: "MASTER".to_string(),
//!     color: WidgetColor::Cyan,
//!     width: 200.0,
//!     height: 150.0,
//!     collapsed: false,
//!     contained_widgets: Vec::new(),
//! }, Pos2::new(50.0, 50.0));
//! 
//! // Render the canvas
//! canvas.render(ui);
//! ```

use egui::{Color32, Pos2, Rect, Ui, Vec2, FontId, Align2, RichText, Stroke};
use std::f32::consts::PI;
use crate::canvas::constants::*;
use crate::canvas::panels::PanelManager;
use crate::canvas::widgets::types::*;


/// Main canvas for drag-and-drop widget management
/// 
/// Handles all widget positioning, interaction states, and rendering.
/// Supports nested panels, smart positioning, and visual feedback.
pub struct DragDropCanvas {
    pub widgets: Vec<DraggableWidget>,
    pub next_id: usize,
    pub canvas_rect: Rect,
    pub editing_widget: Option<usize>, // Index of widget being edited
    pub show_edit_window: bool,
    
    // Panel selection state
    pub selected_panel: Option<usize>, // ID of currently selected panel for widget placement
    
    // Drag and drop state (cleaned up but kept compatible)
    pub dragging_widget: Option<usize>, // Index of currently dragging widget
    pub drag_offset: Vec2,
    pub interacting_widget: Option<usize>, // Index of widget being interacted with
    pub last_mouse_pos: Option<Pos2>,
    pub resizing_widget: Option<usize>, // Index of widget being resized
    pub resize_start_size: Vec2, // Original size when resize started
    pub palette_dragging: Option<WidgetType>, // Widget type being dragged from palette
    pub palette_drag_pos: Option<Pos2>, // Current position of palette drag
    
    // Visual feedback
    pub alignment_guides: Vec<AlignmentGuide>,
    pub drag_hover_panel: Option<usize>, // Panel being hovered over during drag
    pub needs_repositioning: bool, // Whether canvas widgets need to be repositioned
}

#[derive(Debug, Clone)]
pub struct AlignmentGuide {
    pub start: Pos2,
    pub end: Pos2,
    pub guide_type: AlignmentType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlignmentType {
    CenterHorizontal, // Pink - centered on canvas
    CenterVertical,   // Pink - centered on canvas
    WidgetAlignHorizontal, // Yellow - aligned with other widget
    WidgetAlignVertical,   // Yellow - aligned with other widget
}

impl Default for DragDropCanvas {
    fn default() -> Self {
        Self {
            widgets: Vec::new(),
            next_id: 0,
            canvas_rect: Rect::NOTHING,
            editing_widget: None,
            show_edit_window: false,
            selected_panel: None,
            dragging_widget: None,
            drag_offset: Vec2::ZERO,
            interacting_widget: None,
            last_mouse_pos: None,
            resizing_widget: None,
            resize_start_size: Vec2::ZERO,
            palette_dragging: None,
            palette_drag_pos: None,
            alignment_guides: Vec::new(),
            drag_hover_panel: None,
            needs_repositioning: false,
        }
    }
}

// Panel containment management
impl DragDropCanvas {
    /// Check if a widget is contained within any panel
    fn is_widget_contained(&self, widget_id: usize) -> bool {
        self.widgets.iter().any(|panel| {
            match &panel.widget_type {
                WidgetType::Panel { contained_widgets, .. } => {
                    contained_widgets.contains(&widget_id)
                }
                WidgetType::Settings { contained_widgets, .. } => {
                    contained_widgets.contains(&widget_id)
                }
                _ => false,
            }
        })
    }
    
    /// Get the list of widgets not contained in any panel (canvas widgets)
    fn get_canvas_widgets(&self) -> Vec<usize> {
        self.widgets.iter()
            .enumerate()
            .filter_map(|(idx, widget)| {
                if !self.is_widget_contained(widget.id) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Simple, reliable right-to-left grid positioning
    fn find_next_canvas_position(&self, widget_size: Vec2) -> Pos2 {
        let margin = 20.0;
        let spacing = 0.5;
        
        // Define the grid area
        let grid_left = self.canvas_rect.left() + margin;
        let grid_right = self.canvas_rect.right() - margin;
        let grid_top = self.canvas_rect.top() + margin;
        let grid_bottom = self.canvas_rect.bottom() - margin;
        
        // Create a simple position list: start from top-right, move left, then down
        let mut positions = Vec::new();
        
        // Generate all possible grid positions (right-to-left, top-to-bottom)
        let mut y = grid_top;
        while y + widget_size.y <= grid_bottom {
            let mut x = grid_right - widget_size.x;
            while x >= grid_left {
                positions.push(Pos2::new(x, y));
                x -= widget_size.x + spacing;
            }
            y += widget_size.y + spacing;
        }
        
        // Find the first available position
        for pos in positions {
            let test_rect = Rect::from_min_size(pos, widget_size);
            if !self.position_conflicts_with_widgets(test_rect) {
                return pos;
            }
        }
        
        // Fallback: top-right corner
        Pos2::new(grid_right - widget_size.x, grid_top)
    }
    
    /// Simple panel positioning (same logic as canvas)
    fn find_next_panel_position(&self, panel_id: usize, widget_size: Vec2) -> Option<Pos2> {
        let panel_widget = self.widgets.iter().find(|w| w.id == panel_id)?;
        let panel_rect = panel_widget.get_rect();
        
        let padding = 0.5;
        let spacing = 0.5;
        let header_height = 40.0;
        
        // Define panel content area
        let content_left = panel_rect.left() + padding;
        let content_right = panel_rect.right() - padding;
        let content_top = panel_rect.top() + header_height;
        let content_bottom = panel_rect.bottom() - padding;
        
        // Check if widget fits at all
        if widget_size.x > content_right - content_left || widget_size.y > content_bottom - content_top {
            return None;
        }
        
        // Generate positions (right-to-left, top-to-bottom)
        let mut positions = Vec::new();
        let mut y = content_top;
        while y + widget_size.y <= content_bottom {
            let mut x = content_right - widget_size.x;
            while x >= content_left {
                positions.push(Pos2::new(x, y));
                x -= widget_size.x + spacing;
            }
            y += widget_size.y + spacing;
        }
        
        // Find first available position
        for pos in positions {
            let test_rect = Rect::from_min_size(pos, widget_size);
            if !self.position_conflicts_with_widgets(test_rect) {
                return Some(pos);
            }
        }
        
        None // Panel full
    }
    
    /// Constrain widget position to stay within panel bounds with 0.5px padding
    fn constrain_widget_to_panel(&self, widget_pos: Pos2, widget_size: Vec2, panel_id: usize) -> Pos2 {
        if let Some(panel_widget) = self.widgets.iter().find(|w| w.id == panel_id) {
            let panel_rect = panel_widget.get_rect();
            let padding = 0.5;
            let header_height = 40.0;
            
            let min_x = panel_rect.left() + padding;
            let max_x = panel_rect.right() - padding - widget_size.x;
            let min_y = panel_rect.top() + header_height;
            let max_y = panel_rect.bottom() - padding - widget_size.y;
            
            Pos2::new(
                widget_pos.x.clamp(min_x, max_x),
                widget_pos.y.clamp(min_y, max_y)
            )
        } else {
            widget_pos
        }
    }
    
    /// Check if a rect conflicts with any existing widget (tight grid with 0.5px spacing)
    fn position_conflicts_with_widgets(&self, test_rect: Rect) -> bool {
        for widget in &self.widgets {
            let widget_rect = widget.get_rect();
            
            // Skip minimized panels (they're very small and shouldn't block placement)
            if let WidgetType::Panel { collapsed, minimize_to_settings_icon, .. } = &widget.widget_type {
                if *collapsed && *minimize_to_settings_icon {
                    continue; // Skip gear icon panels - they're tiny
                }
            }
            if let WidgetType::Settings { minimized, .. } = &widget.widget_type {
                if *minimized {
                    continue; // Skip minimized settings panels
                }
            }
            
            // Check for overlap - with tight grid, widgets can be very close (0.5px apart)
            // Only prevent actual overlap, not close proximity
            if test_rect.intersects(widget_rect) {
                return true;
            }
        }
        false
    }
    
    /// Get default size for a widget type
    fn get_widget_default_size(widget_type: &WidgetType) -> Vec2 {
        match widget_type {
            WidgetType::Knob { .. } => Vec2::new(90.0, 110.0),
            WidgetType::ToggleSwitch { .. } => Vec2::new(80.0, 60.0),
            WidgetType::PushButton { .. } => Vec2::new(80.0, 80.0),
            WidgetType::VuMeter { .. } => Vec2::new(40.0, 160.0),
            WidgetType::HorizontalSlider { .. } => Vec2::new(150.0, 40.0),
            WidgetType::VerticalSlider { .. } => Vec2::new(40.0, 120.0),
            WidgetType::LevelIndicator { .. } => Vec2::new(120.0, 40.0),
            WidgetType::TextLabel { .. } => Vec2::new(100.0, 30.0),
            WidgetType::Panel { .. } => Vec2::new(200.0, 150.0),
            WidgetType::StatusBar { .. } => Vec2::new(300.0, 40.0),
            WidgetType::IconButton { .. } => Vec2::new(60.0, 80.0),
            WidgetType::Settings { .. } => Vec2::new(250.0, 300.0),
        }
    }
    
    /// Spawn widget directly (either on canvas or in selected panel)
    fn spawn_widget_directly(&mut self, widget_type: WidgetType) {
        let widget_size = Self::get_widget_default_size(&widget_type);
        
        if let Some(panel_id) = self.selected_panel {
            // Try to place in selected panel
            if let Some(pos) = self.find_next_panel_position(panel_id, widget_size) {
                self.add_widget_to_selected_panel(widget_type, pos);
            } else {
                // Panel is full, place on canvas instead
                let pos = self.find_next_canvas_position(widget_size);
                self.add_widget(widget_type, pos);
            }
        } else {
            // Place on canvas
            let pos = self.find_next_canvas_position(widget_size);
            self.add_widget(widget_type, pos);
        }
    }
    
    /// Simple grid reposition on canvas resize
    fn reposition_canvas_widgets_for_resize(&mut self) {
        let margin = 20.0;
        let spacing = 0.5;
        
        // Get canvas widgets only (not in panels)
        let mut canvas_widgets: Vec<usize> = self.widgets.iter()
            .enumerate()
            .filter_map(|(idx, widget)| {
                if !self.is_widget_contained(widget.id) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by current position (preserve order)
        canvas_widgets.sort_by(|&a, &b| {
            let pos_a = self.widgets[a].position;
            let pos_b = self.widgets[b].position;
            
            // Sort by row first, then by column (right-to-left)
            match pos_a.y.partial_cmp(&pos_b.y) {
                Some(std::cmp::Ordering::Equal) => pos_b.x.partial_cmp(&pos_a.x).unwrap_or(std::cmp::Ordering::Equal),
                other => other.unwrap_or(std::cmp::Ordering::Equal),
            }
        });
        
        // Generate new grid positions
        let grid_left = self.canvas_rect.left() + margin;
        let grid_right = self.canvas_rect.right() - margin;
        let grid_top = self.canvas_rect.top() + margin;
        let grid_bottom = self.canvas_rect.bottom() - margin;
        
        let mut new_positions = Vec::new();
        let mut y = grid_top;
        
        // Assume uniform widget size for simplicity (can be improved later)
        let widget_size = Vec2::new(90.0, 110.0); // Default knob size
        
        while y + widget_size.y <= grid_bottom {
            let mut x = grid_right - widget_size.x;
            while x >= grid_left {
                new_positions.push(Pos2::new(x, y));
                x -= widget_size.x + spacing;
            }
            y += widget_size.y + spacing;
        }
        
        // Apply new positions to widgets
        for (i, &widget_idx) in canvas_widgets.iter().enumerate() {
            if let Some(&new_pos) = new_positions.get(i) {
                if let Some(widget) = self.widgets.get_mut(widget_idx) {
                    widget.position = new_pos;
                }
            }
        }
    }
    
}

impl DragDropCanvas {
    /// Create a new drag and drop canvas
    pub fn new() -> Self {
        Self::default()
    }
    

    fn add_widget_to_selected_panel(&mut self, widget_type: WidgetType, click_pos: Pos2) {
        if let Some(panel_id) = self.selected_panel {
            // Find the panel
            if let Some(panel_idx) = self.widgets.iter().position(|w| w.id == panel_id) {
                // Check if panel can accept widgets (not collapsed/minimized)
                if PanelManager::is_panel_accepting_widgets(&self.widgets[panel_idx]) {
                    let panel_rect = self.widgets[panel_idx].get_rect();
                    let widget_size = DraggableWidget::calculate_size(&widget_type);
                    
                    // Calculate desired position within panel (relative to click)
                    let desired_x = click_pos.x.max(panel_rect.left() + PANEL_MARGIN);
                    let desired_y = click_pos.y.max(panel_rect.top() + PANEL_TITLE_HEIGHT);
                    
                    // Ensure widget stays within panel bounds
                    let max_x = panel_rect.right() - PANEL_MARGIN - widget_size.x;
                    let max_y = panel_rect.bottom() - PANEL_MARGIN - widget_size.y;
                    
                    let mut final_pos = Pos2::new(
                        desired_x.min(max_x),
                        desired_y.min(max_y)
                    );
                    
                    // Check for overlaps with existing widgets in this panel and adjust position
                    let panel_widget_ids = match &self.widgets[panel_idx].widget_type {
                        WidgetType::Panel { contained_widgets, .. } => contained_widgets.clone(),
                        WidgetType::Settings { contained_widgets, .. } => contained_widgets.clone(),
                        _ => Vec::new(),
                    };
                    
                    final_pos = self.find_non_overlapping_position(final_pos, widget_size, &panel_widget_ids, panel_rect);
                    
                    let widget = DraggableWidget::new(self.next_id, widget_type, final_pos);
                    let widget_id = widget.id;
                    self.widgets.push(widget);
                    self.next_id += 1;
                    
                    // Add to panel's contained widgets
                    PanelManager::add_widget_to_panel(&mut self.widgets, panel_idx, widget_id);
                    return; // Successfully placed in panel
                } else {
                    // Panel is collapsed/minimized, clear selection and fall back to canvas
                    self.selected_panel = None;
                }
            } else {
                // Selected panel no longer exists, clear selection and fall back to canvas
                self.selected_panel = None;
            }
        }
        
        // Fallback: place on canvas if no valid selected panel
        self.add_widget(widget_type, click_pos);
    }
    
    fn find_non_overlapping_position(&self, preferred_pos: Pos2, widget_size: Vec2, existing_widget_ids: &[usize], bounds: Rect) -> Pos2 {
        let padding = 1.0; // 1 pixel padding
        let mut test_pos = preferred_pos;
        
        // Check if current position overlaps with any existing widgets
        for &widget_id in existing_widget_ids {
            if let Some(existing_widget) = self.widgets.iter().find(|w| w.id == widget_id) {
                let existing_rect = existing_widget.get_rect().expand(padding);
                let test_rect = Rect::from_min_size(test_pos, widget_size);
                
                if existing_rect.intersects(test_rect) {
                    // Try to the right first
                    test_pos.x = existing_rect.right() + padding;
                    
                    // If that goes outside bounds, try below
                    if test_pos.x + widget_size.x > bounds.right() - PANEL_MARGIN {
                        test_pos.x = preferred_pos.x;
                        test_pos.y = existing_rect.bottom() + padding;
                        
                        // If that goes outside bounds, find first available spot
                        if test_pos.y + widget_size.y > bounds.bottom() - PANEL_MARGIN {
                            test_pos = self.find_first_available_spot(widget_size, existing_widget_ids, bounds);
                        }
                    }
                    break;
                }
            }
        }
        
        test_pos
    }
    
    fn find_first_available_spot(&self, widget_size: Vec2, existing_widget_ids: &[usize], bounds: Rect) -> Pos2 {
        let padding = 1.0;
        let start_x = bounds.left() + PANEL_MARGIN;
        let start_y = bounds.top() + PANEL_TITLE_HEIGHT;
        let step = 20.0; // Grid step for searching
        
        for y in (start_y as i32..(bounds.bottom() - PANEL_MARGIN - widget_size.y) as i32).step_by(step as usize) {
            for x in (start_x as i32..(bounds.right() - PANEL_MARGIN - widget_size.x) as i32).step_by(step as usize) {
                let test_pos = Pos2::new(x as f32, y as f32);
                let test_rect = Rect::from_min_size(test_pos, widget_size);
                
                let mut overlaps = false;
                for &widget_id in existing_widget_ids {
                    if let Some(existing_widget) = self.widgets.iter().find(|w| w.id == widget_id) {
                        if existing_widget.get_rect().expand(padding).intersects(test_rect) {
                            overlaps = true;
                            break;
                        }
                    }
                }
                
                if !overlaps {
                    return test_pos;
                }
            }
        }
        
        // Fallback to preferred position if no spot found
        Pos2::new(start_x, start_y)
    }
    
    pub fn add_widget(&mut self, widget_type: WidgetType, _position: Pos2) {
        // Calculate position using the new right-to-left logic
        let position = if self.canvas_rect != Rect::NOTHING {
            // Canvas size is known, use new right-to-left positioning
            let widget_size = Self::get_widget_default_size(&widget_type);
            self.find_next_canvas_position(widget_size)
        } else {
            // Canvas size unknown, use safe position and mark for later repositioning
            self.needs_repositioning = true;
            Pos2::new(50.0, 50.0)
        };
        
        let widget = DraggableWidget::new(self.next_id, widget_type, position);
        self.widgets.push(widget);
        self.next_id += 1;
    }
    
    fn count_canvas_widgets(&self) -> usize {
        // Count widgets that are on the main canvas (not in any panel)
        self.widgets.iter()
            .filter(|w| {
                // Check if widget is not contained in any panel
                !self.widgets.iter().any(|panel| {
                    match &panel.widget_type {
                        WidgetType::Panel { contained_widgets, .. } => contained_widgets.contains(&w.id),
                        WidgetType::Settings { contained_widgets, .. } => contained_widgets.contains(&w.id),
                        _ => false,
                    }
                })
            })
            .count()
    }

    pub fn render(&mut self, ui: &mut Ui) {
        // Set canvas background to match React app (black)
        ui.style_mut().visuals.extreme_bg_color = BLACK;
        ui.style_mut().visuals.panel_fill = BLACK;
        
        // Get the actual drawing area after UI elements
        let available_rect = ui.available_rect_before_wrap();
        
        // Account for the UI elements that are drawn before this canvas
        // The canvas starts after heading and instructions, so add offset
        let actual_canvas_start_y = available_rect.min.y + 60.0; // Approximate height of heading + separator + instructions
        let actual_canvas_rect = Rect::from_min_max(
            Pos2::new(available_rect.min.x, actual_canvas_start_y),
            available_rect.max
        );
        
        // Check if canvas size changed (for dynamic repositioning)
        let canvas_size_changed = self.canvas_rect != Rect::NOTHING && 
                                 (self.canvas_rect.width() != actual_canvas_rect.width() || 
                                  self.canvas_rect.height() != actual_canvas_rect.height());
        
        self.canvas_rect = actual_canvas_rect;
        
        // Reposition canvas widgets if needed (after canvas size is known)
        if self.needs_repositioning {
            self.reposition_canvas_widgets();
            self.needs_repositioning = false;
        } else if canvas_size_changed {
            // Canvas size changed - reposition widgets to maintain tight grid
            self.reposition_canvas_widgets_for_resize();
        }

        // Draw canvas background
        ui.painter().rect_filled(actual_canvas_rect, 0.0, BLACK);

        // Handle drag and drop input (only when edit window is not open)
        if !self.show_edit_window {
            self.handle_drag_drop(ui);
        }

        // Collect which widgets should be rendered (not in minimized panels)
        let widgets_to_render: Vec<bool> = self.widgets.iter()
            .map(|w| !self.is_widget_in_minimized_panel(w.id))
            .collect();
        
        // Render widgets that should be visible
        for (widget, &should_render) in self.widgets.iter_mut().zip(widgets_to_render.iter()) {
            if should_render {
                widget.render(ui);
            }
        }

        // Draw alignment guides
        let painter = ui.painter();
        for guide in &self.alignment_guides {
            let (color, width) = match guide.guide_type {
                AlignmentType::CenterHorizontal | AlignmentType::CenterVertical => (PINK, 2.0),
                AlignmentType::WidgetAlignHorizontal | AlignmentType::WidgetAlignVertical => (YELLOW, 1.5),
            };
            
            painter.line_segment([guide.start, guide.end], Stroke::new(width, color));
        }

        // Note: Removed visible selection borders around widgets as requested
        
        // Draw hover highlight for panel during drag
        if let Some(hover_panel_id) = self.drag_hover_panel {
            if let Some(hover_panel) = self.widgets.iter().find(|w| w.id == hover_panel_id) {
                let rect = hover_panel.get_rect().expand(2.0);
                let stroke = Stroke::new(3.0, GREEN);
                
                // Draw highlight border using line segments
                painter.line_segment([rect.left_top(), rect.right_top()], stroke);
                painter.line_segment([rect.right_top(), rect.right_bottom()], stroke);
                painter.line_segment([rect.right_bottom(), rect.left_bottom()], stroke);
                painter.line_segment([rect.left_bottom(), rect.left_top()], stroke);
            }
        }
        
        // Draw selection highlight
        if let Some(selected_panel_id) = self.selected_panel {
            // Highlight selected panel with cyan (only if not collapsed/minimized)
            if let Some(selected_panel) = self.widgets.iter().find(|w| w.id == selected_panel_id) {
                let should_show_border = match &selected_panel.widget_type {
                    WidgetType::Panel { collapsed, .. } => !collapsed,
                    WidgetType::Settings { minimized, .. } => !minimized,
                    _ => true, // For any other widget types, show the border
                };
                
                if should_show_border {
                    let rect = selected_panel.get_rect().expand(2.0);
                    let stroke = Stroke::new(3.0, CYAN);
                    
                    // Draw highlight border using line segments
                    painter.line_segment([rect.left_top(), rect.right_top()], stroke);
                    painter.line_segment([rect.right_top(), rect.right_bottom()], stroke);
                    painter.line_segment([rect.right_bottom(), rect.left_bottom()], stroke);
                    painter.line_segment([rect.left_bottom(), rect.left_top()], stroke);
                }
            }
        }
        // No main canvas selection highlighting needed anymore

        // Draw static settings icon in top-left
        self.render_settings_icon(ui);
        
        
        // Draw palette dragging preview
        if let (Some(widget_type), Some(pos)) = (&self.palette_dragging, self.palette_drag_pos) {
            let size = DraggableWidget::calculate_size(widget_type);
            let preview_rect = Rect::from_min_size(pos - size / 2.0, size);
            
            // Draw semi-transparent preview
            let painter = ui.painter();
            painter.rect_filled(preview_rect, 4.0, Color32::from_rgba_unmultiplied(100, 100, 100, 100));
            
            // Draw widget type label
            painter.text(
                preview_rect.center(),
                Align2::CENTER_CENTER,
                match widget_type {
                    WidgetType::Knob { .. } => "Knob",
                    WidgetType::ToggleSwitch { .. } => "Toggle",
                    WidgetType::PushButton { .. } => "Button",
                    WidgetType::VuMeter { .. } => "VU Meter",
                    WidgetType::HorizontalSlider { .. } => "H Slider",
                    WidgetType::VerticalSlider { .. } => "V Slider",
                    WidgetType::LevelIndicator { .. } => "Level",
                    WidgetType::TextLabel { .. } => "Text",
                    WidgetType::Panel { .. } => "Panel",
                    WidgetType::StatusBar { .. } => "Status",
                    WidgetType::IconButton { .. } => "Icon",
                    WidgetType::Settings { .. } => "Settings",
                },
                FontId::monospace(12.0),
                WHITE,
            );
        }
        
        // Show edit window
        if self.show_edit_window {
            self.show_edit_window(ui);
        }
    }

    fn handle_drag_drop(&mut self, ui: &mut Ui) {
        let mouse_pos = ui.ctx().input(|i| i.pointer.interact_pos());
        let mouse_pressed = ui.ctx().input(|i| i.pointer.primary_pressed());
        let mouse_released = ui.ctx().input(|i| i.pointer.primary_released());
        let right_clicked = ui.ctx().input(|i| i.pointer.secondary_pressed());
        let mouse_held = ui.ctx().input(|i| i.pointer.primary_down());
        
        // Handle clicks
        
        // Handle click operations (both widget placement and panel selection)
        if mouse_pressed {
            if let Some(pos) = mouse_pos {
                // Check if on canvas (not on side panel)
                if pos.x > PALETTE_WIDTH { // Beyond the palette width
                    // Check if we clicked on a panel
                    let mut clicked_panel_id = None;
                    for widget in self.widgets.iter().rev() {
                        if widget.get_rect().contains(pos) {
                            match &widget.widget_type {
                                WidgetType::Panel { collapsed, .. } => {
                                    if !collapsed {
                                        clicked_panel_id = Some(widget.id);
                                        break;
                                    }
                                }
                                WidgetType::Settings { minimized, .. } => {
                                    if !minimized {
                                        clicked_panel_id = Some(widget.id);
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    
                    // Handle panel selection (no pending widget logic needed)
                    if let Some(panel_id) = clicked_panel_id {
                        self.selected_panel = Some(panel_id);
                        // Don't return here - let dragging logic run for moving panels
                    } else {
                        // Clicked on empty canvas - deselect panel
                        self.selected_panel = None;
                        // Don't return here - let dragging logic run
                    }
                }
            }
        }
        
        // Handle palette dragging
        if let Some(widget_type) = self.palette_dragging.clone() {
            if let Some(pos) = mouse_pos {
                self.palette_drag_pos = Some(pos);
                
                // If mouse released, drop the widget
                if mouse_released {
                    // Check if dropped on canvas (not on side panel)
                    if pos.x > PALETTE_WIDTH { // Beyond the palette width
                        // Check if we dropped on a panel
                        let mut dropped_on_panel_id = None;
                        for widget in self.widgets.iter().rev() {
                            if widget.get_rect().contains(pos) {
                                match &widget.widget_type {
                                    WidgetType::Panel { collapsed, .. } => {
                                        if !collapsed {
                                            dropped_on_panel_id = Some(widget.id);
                                            break;
                                        }
                                    }
                                    WidgetType::Settings { minimized, .. } => {
                                        if !minimized {
                                            dropped_on_panel_id = Some(widget.id);
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        
                        // Place the widget
                        if let Some(panel_id) = dropped_on_panel_id {
                            // Dropped on a panel - place widget in that panel and select it
                            self.selected_panel = Some(panel_id);
                            self.add_widget_to_selected_panel(widget_type, pos);
                        } else if let Some(panel_id) = self.selected_panel {
                            // Have a selected panel - check if drop is within that panel
                            let drop_in_selected_panel = self.widgets.iter()
                                .find(|w| w.id == panel_id)
                                .map(|w| w.get_rect().contains(pos))
                                .unwrap_or(false);
                            
                            if drop_in_selected_panel {
                                // Drop is inside the selected panel - place widget there
                                self.add_widget_to_selected_panel(widget_type, pos);
                            } else {
                                // Drop is outside the selected panel - place on canvas
                                self.add_widget(widget_type, pos);
                            }
                        } else {
                            // No panel selected - place on canvas
                            self.add_widget(widget_type, pos);
                        }
                    }
                    
                    self.palette_dragging = None;
                    self.palette_drag_pos = None;
                }
            }
            return; // Don't process other drag operations while palette dragging
        }

        // Handle right-click for editing
        if right_clicked {
            if let Some(pos) = mouse_pos {
                for (idx, widget) in self.widgets.iter().enumerate().rev() {
                    if widget.get_rect().contains(pos) {
                        self.editing_widget = Some(idx);
                        self.show_edit_window = true;
                        break;
                    }
                }
            }
        }

        // Handle mouse press
        if mouse_pressed && self.dragging_widget.is_none() && self.interacting_widget.is_none() && self.resizing_widget.is_none() {
            if let Some(pos) = mouse_pos {
                // First, assume we clicked on empty space
                let mut _clicked_widget = false;
                
                for (idx, widget) in self.widgets.iter().enumerate().rev() {
                    if widget.get_rect().contains(pos) {
                        // Check if clicking on panel or status bar resize handle
                        if matches!(widget.widget_type, WidgetType::Panel { .. } | WidgetType::StatusBar { .. }) {
                            let rect = widget.get_rect();
                            let handle_size = 12.0;
                            let handle_rect = Rect::from_min_size(
                                Pos2::new(rect.max.x - handle_size, rect.max.y - handle_size),
                                Vec2::splat(handle_size),
                            );
                            
                            if handle_rect.contains(pos) {
                                self.resizing_widget = Some(idx);
                                self.resize_start_size = widget.size;
                                self.last_mouse_pos = Some(pos);
                                break;
                            }
                        }
                        
                        // Check if clicking on interactive widgets (knobs, toggles, buttons)
                        match widget.widget_type {
                            WidgetType::Knob { .. } => {
                                let knob_center = Pos2::new(
                                    widget.position.x + widget.size.x / 2.0,
                                    widget.position.y + 37.0
                                );
                                let distance = (pos - knob_center).length();
                                if distance <= 32.0 { // Within knob radius
                                    // Check if this widget is inside a panel and preserve panel selection
                                    let widget_panel_id = PanelManager::find_widget_container_panel_id(&self.widgets, widget.id);
                                    if let Some(panel_id) = widget_panel_id {
                                        // Widget is inside a panel - maintain that panel as selected
                                        self.selected_panel = Some(panel_id);
                                    }
                                    
                                    self.interacting_widget = Some(idx);
                                    self.last_mouse_pos = Some(pos);
                                    break;
                                }
                            }
                            WidgetType::ToggleSwitch { .. } | 
                            WidgetType::PushButton { .. } | 
                            WidgetType::IconButton { .. } => {
                                // These widgets can be both clicked and dragged
                                // For now, just allow dragging - interaction will be handled on mouse release without drag
                            }
                            WidgetType::Panel { .. } => {
                                // Check if clicking on collapse triangle
                                let title_area = Rect::from_min_size(
                                    widget.position,
                                    Vec2::new(widget.size.x, 40.0),
                                );
                                if title_area.contains(pos) && pos.x < widget.position.x + 30.0 {
                                    // Handle Panel collapse click - maintain panel selection
                                    self.selected_panel = Some(widget.id);
                                    self.handle_widget_interaction(idx, pos);
                                    return; // Exit early
                                }
                                // Just allow dragging the panel - no area selection
                            }
                            _ => {}
                        }
                        
                        // Check if this widget is inside a panel and preserve panel selection
                        let widget_panel_id = PanelManager::find_widget_container_panel_id(&self.widgets, widget.id);
                        if let Some(panel_id) = widget_panel_id {
                            // Widget is inside a panel - maintain that panel as selected
                            self.selected_panel = Some(panel_id);
                        }
                        
                        // For non-knob widgets or outside knob center, allow for dragging
                        self.dragging_widget = Some(idx);
                        self.drag_offset = pos - widget.position;
                        _clicked_widget = true;
                        break;
                    }
                }
                
                // No special handling needed for empty space clicks
            }
        }

        // Handle widget interactions (knob turning)
        if let Some(idx) = self.interacting_widget {
            if mouse_held {
                if let (Some(current_pos), Some(last_pos)) = (mouse_pos, self.last_mouse_pos) {
                    let delta_y = last_pos.y - current_pos.y; // Invert for natural feel
                    self.handle_knob_interaction(idx, delta_y);
                    self.last_mouse_pos = Some(current_pos);
                }
            } else {
                self.interacting_widget = None;
                self.last_mouse_pos = None;
            }
        }

        // Handle widget dragging
        if let Some(idx) = self.dragging_widget {
            if mouse_held {
                if let Some(pos) = mouse_pos {
                    // Get widget data first
                    let (widget_size, new_pos) = if let Some(widget) = self.widgets.get(idx) {
                        (widget.size, pos - self.drag_offset)
                    } else {
                        return;
                    };
                    
                    // Check if widget is contained in any panel and constrain accordingly with 0.5px padding
                    let mut final_pos = if let Some(container_panel) = PanelManager::find_widget_container_panel(&self.widgets, idx) {
                        // Use the new constraint method with 0.5px padding
                        self.constrain_widget_to_panel(new_pos, widget_size, self.widgets[container_panel].id)
                    } else {
                        // Constrain to canvas bounds (no padding needed for canvas)
                        let max_x = (self.canvas_rect.max.x - widget_size.x).max(self.canvas_rect.min.x);
                        let max_y = (self.canvas_rect.max.y - widget_size.y).max(self.canvas_rect.min.y);
                        Pos2::new(
                            new_pos.x.clamp(self.canvas_rect.min.x, max_x),
                            new_pos.y.clamp(self.canvas_rect.min.y, max_y),
                        )
                    };
                    
                    // Calculate alignment guides and snap if close
                    self.calculate_alignment_guides(idx, final_pos, widget_size);
                    
                    // Apply snapping based on guides
                    final_pos = self.apply_snapping(idx, final_pos, widget_size);
                    
                    // Check for panel hover during drag
                    self.drag_hover_panel = PanelManager::find_panel_under_position(&self.widgets, pos);
                    
                    // Update widget position
                    if let Some(widget) = self.widgets.get_mut(idx) {
                        widget.position = final_pos;
                    }
                }
            } else {
                self.dragging_widget = None;
                self.alignment_guides.clear();
                self.drag_hover_panel = None;
            }
        }

        // Handle widget resizing
        if let Some(idx) = self.resizing_widget {
            if mouse_held {
                if let (Some(current_pos), Some(last_pos)) = (mouse_pos, self.last_mouse_pos) {
                    let delta = current_pos - last_pos;
                    
                    if let Some(widget) = self.widgets.get_mut(idx) {
                        match &mut widget.widget_type {
                            WidgetType::Panel { width, height, .. } => {
                                *width = (*width + delta.x).max(100.0).min(500.0);
                                *height = (*height + delta.y).max(100.0).min(400.0);
                                
                                // Update widget size
                                widget.size = Vec2::new(*width, *height);
                            }
                            WidgetType::StatusBar { .. } => {
                                // Status bars can be resized in width and height
                                let new_width = (widget.size.x + delta.x).max(200.0).min(800.0);
                                let new_height = (widget.size.y + delta.y).max(40.0).min(120.0);
                                
                                // Update widget size
                                widget.size = Vec2::new(new_width, new_height);
                            }
                            _ => {}
                        }
                    }
                    
                    self.last_mouse_pos = Some(current_pos);
                }
            } else {
                self.resizing_widget = None;
                self.last_mouse_pos = None;
            }
        }

        // Handle single clicks for remaining interactive widgets (sliders, status bars)
        if mouse_pressed && self.dragging_widget.is_none() && self.interacting_widget.is_none() && self.resizing_widget.is_none() {
            if let Some(pos) = mouse_pos {
                for i in 0..self.widgets.len() {
                    if self.widgets[i].get_rect().contains(pos) {
                        // Only handle widgets not already handled above
                        match &self.widgets[i].widget_type {
                            WidgetType::Knob { .. } | 
                            WidgetType::ToggleSwitch { .. } | 
                            WidgetType::PushButton { .. } | 
                            WidgetType::IconButton { .. } => {} // Already handled above
                            _ => self.handle_widget_interaction(i, pos),
                        }
                        break;
                    }
                }
            }
        }

        // Stop interactions on mouse release
        if mouse_released {
            // Panel drag operations completed
            
            // Check if widget was dragged out of any panel and remove it from containers
            if let Some(drag_idx) = self.dragging_widget {
                if let Some(widget) = self.widgets.get(drag_idx) {
                    let widget_rect = widget.get_rect();
                    let widget_id = widget.id;
                    
                    // Check if widget is still inside any panel it was originally in
                    let mut should_remove_from_all = true;
                    
                    for panel in &self.widgets {
                        if PanelManager::is_panel_accepting_widgets(panel) && panel.get_rect().contains(widget_rect.center()) {
                            should_remove_from_all = false;
                            break;
                        }
                    }
                    
                    // If widget is no longer inside any panel, remove it from all containers
                    if should_remove_from_all {
                        PanelManager::remove_widget_from_containers(&mut self.widgets, widget_id);
                    }
                }
            }
            
            // Handle click interactions for widgets that were clicked but not dragged
            if let Some(drag_idx) = self.dragging_widget {
                if let Some(pos) = mouse_pos {
                    if let Some(widget) = self.widgets.get(drag_idx) {
                        let original_pos = pos - self.drag_offset;
                        let drag_distance = (widget.position - original_pos).length();
                        
                        // If the widget wasn't actually dragged (very small movement), treat it as a click
                        if drag_distance < 5.0 {
                            match widget.widget_type {
                                WidgetType::ToggleSwitch { .. } | 
                                WidgetType::PushButton { .. } | 
                                WidgetType::IconButton { .. } => {
                                    self.handle_widget_interaction(drag_idx, pos);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            
            self.dragging_widget = None;
            self.interacting_widget = None;
            self.resizing_widget = None;
            self.last_mouse_pos = None;
        }
    }

    fn handle_widget_interaction(&mut self, widget_idx: usize, mouse_pos: Pos2) {
        // Handle panel interaction
        if let Some(widget) = self.widgets.get(widget_idx) {
            if let WidgetType::Panel { collapsed, width, height, minimize_to_settings_icon, .. } = &widget.widget_type {
                let _was_collapsed = *collapsed;
                let current_width = *width;
                let current_height = *height;
                let is_settings_icon = *minimize_to_settings_icon;
                
                // Toggle collapsed state
                if let Some(widget) = self.widgets.get_mut(widget_idx) {
                    if let WidgetType::Panel { collapsed, .. } = &mut widget.widget_type {
                        *collapsed = !*collapsed;
                        
                        // Update widget size when toggling state
                        let new_size = if *collapsed {
                            if is_settings_icon {
                                Vec2::new(40.0, 40.0) // Settings icon size when minimized
                            } else {
                                Vec2::new(current_width, 40.0) // Just title bar height
                            }
                        } else {
                            Vec2::new(current_width, current_height)
                        };
                        
                        widget.size = new_size;
                    }
                }
                return;
            }
        }
        
        // Handle all other widget types
        if let Some(widget) = self.widgets.get_mut(widget_idx) {
            let rect = widget.get_rect();
            match &mut widget.widget_type {
                WidgetType::Knob { value, min, max, .. } => {
                    let center = Pos2::new(rect.center().x, rect.top() + 37.0);
                    let mouse_vec = mouse_pos - center;
                    let angle = mouse_vec.y.atan2(mouse_vec.x);
                    let normalized_angle = (angle + 135.0 * PI / 180.0) / (270.0 * PI / 180.0);
                    *value = (normalized_angle.clamp(0.0, 1.0) * (*max - *min) + *min).clamp(*min, *max);
                }
                WidgetType::ToggleSwitch { on, .. } => {
                    *on = !*on;
                }
                WidgetType::PushButton { active, .. } => {
                    *active = !*active;
                }
                WidgetType::IconButton { active, .. } => {
                    *active = !*active;
                }
                WidgetType::HorizontalSlider { value, min, max, .. } => {
                    let slider_rect = Rect::from_center_size(
                        Pos2::new(rect.center().x + 10.0, rect.center().y),
                        Vec2::new(96.0, 8.0),
                    );
                    if slider_rect.contains(mouse_pos) {
                        let normalized = ((mouse_pos.x - slider_rect.left()) / slider_rect.width()).clamp(0.0, 1.0);
                        *value = normalized * (*max - *min) + *min;
                    }
                }
                WidgetType::VerticalSlider { value, min, max, .. } => {
                    let slider_rect = Rect::from_center_size(
                        Pos2::new(rect.center().x, rect.center().y - 10.0),
                        Vec2::new(8.0, 96.0),
                    );
                    if slider_rect.contains(mouse_pos) {
                        let normalized = 1.0 - ((mouse_pos.y - slider_rect.top()) / slider_rect.height()).clamp(0.0, 1.0);
                        *value = normalized * (*max - *min) + *min;
                    }
                }
                WidgetType::StatusBar { online, .. } => {
                    *online = !*online;
                }
                WidgetType::Panel { collapsed, .. } => {
                    *collapsed = !*collapsed;
                    
                    // Update widget size when toggling state
                    let new_size = if *collapsed {
                        Vec2::new(widget.size.x, 40.0) // Collapsed height
                    } else {
                        // Get original height from widget type
                        if let WidgetType::Panel { height, width, .. } = &widget.widget_type {
                            Vec2::new(*width, *height)
                        } else {
                            widget.size // Fallback
                        }
                    };
                    
                    // Update the widget's actual size
                    if let Some(widget) = self.widgets.get_mut(widget_idx) {
                        widget.size = new_size;
                    }
                }
                _ => {} // Other widgets don't have direct interactions yet
            }
        }
    }

    fn handle_knob_interaction(&mut self, widget_idx: usize, delta_y: f32) {
        if let Some(widget) = self.widgets.get_mut(widget_idx) {
            if let WidgetType::Knob { value, min, max, .. } = &mut widget.widget_type {
                let sensitivity = 0.5; // Adjust for desired sensitivity
                let range = *max - *min;
                let delta_value = (delta_y * sensitivity / 100.0) * range;
                *value = (*value + delta_value).clamp(*min, *max);
            }
        }
    }

    fn calculate_alignment_guides(&mut self, dragging_idx: usize, position: Pos2, size: Vec2) {
        self.alignment_guides.clear();
        let threshold = 8.0; // Distance threshold for showing guides
        
        // Canvas center guides
        let canvas_center_x = self.canvas_rect.center().x;
        let canvas_center_y = self.canvas_rect.center().y;
        let widget_center_x = position.x + size.x / 2.0;
        let widget_center_y = position.y + size.y / 2.0;
        
        // Check horizontal center alignment with canvas
        if (widget_center_x - canvas_center_x).abs() < threshold {
            self.alignment_guides.push(AlignmentGuide {
                start: Pos2::new(canvas_center_x, self.canvas_rect.min.y),
                end: Pos2::new(canvas_center_x, self.canvas_rect.max.y),
                guide_type: AlignmentType::CenterHorizontal,
            });
        }
        
        // Check vertical center alignment with canvas
        if (widget_center_y - canvas_center_y).abs() < threshold {
            self.alignment_guides.push(AlignmentGuide {
                start: Pos2::new(self.canvas_rect.min.x, canvas_center_y),
                end: Pos2::new(self.canvas_rect.max.x, canvas_center_y),
                guide_type: AlignmentType::CenterVertical,
            });
        }
        
        // Check alignment with other widgets
        for (idx, other_widget) in self.widgets.iter().enumerate() {
            if idx == dragging_idx {
                continue;
            }
            
            let other_center_x = other_widget.position.x + other_widget.size.x / 2.0;
            let other_center_y = other_widget.position.y + other_widget.size.y / 2.0;
            
            // Horizontal alignment with other widgets
            if (widget_center_x - other_center_x).abs() < threshold {
                self.alignment_guides.push(AlignmentGuide {
                    start: Pos2::new(other_center_x, other_widget.position.y.min(position.y) - 20.0),
                    end: Pos2::new(other_center_x, (other_widget.position.y + other_widget.size.y).max(position.y + size.y) + 20.0),
                    guide_type: AlignmentType::WidgetAlignHorizontal,
                });
            }
            
            // Vertical alignment with other widgets
            if (widget_center_y - other_center_y).abs() < threshold {
                self.alignment_guides.push(AlignmentGuide {
                    start: Pos2::new(other_widget.position.x.min(position.x) - 20.0, other_center_y),
                    end: Pos2::new((other_widget.position.x + other_widget.size.x).max(position.x + size.x) + 20.0, other_center_y),
                    guide_type: AlignmentType::WidgetAlignVertical,
                });
            }
            
            // Edge alignments (left, right, top, bottom)
            // Left edge alignment
            if (position.x - other_widget.position.x).abs() < threshold {
                self.alignment_guides.push(AlignmentGuide {
                    start: Pos2::new(other_widget.position.x, other_widget.position.y.min(position.y) - 20.0),
                    end: Pos2::new(other_widget.position.x, (other_widget.position.y + other_widget.size.y).max(position.y + size.y) + 20.0),
                    guide_type: AlignmentType::WidgetAlignHorizontal,
                });
            }
            
            // Right edge alignment
            if ((position.x + size.x) - (other_widget.position.x + other_widget.size.x)).abs() < threshold {
                let x = other_widget.position.x + other_widget.size.x;
                self.alignment_guides.push(AlignmentGuide {
                    start: Pos2::new(x, other_widget.position.y.min(position.y) - 20.0),
                    end: Pos2::new(x, (other_widget.position.y + other_widget.size.y).max(position.y + size.y) + 20.0),
                    guide_type: AlignmentType::WidgetAlignHorizontal,
                });
            }
            
            // Top edge alignment
            if (position.y - other_widget.position.y).abs() < threshold {
                self.alignment_guides.push(AlignmentGuide {
                    start: Pos2::new(other_widget.position.x.min(position.x) - 20.0, other_widget.position.y),
                    end: Pos2::new((other_widget.position.x + other_widget.size.x).max(position.x + size.x) + 20.0, other_widget.position.y),
                    guide_type: AlignmentType::WidgetAlignVertical,
                });
            }
            
            // Bottom edge alignment
            if ((position.y + size.y) - (other_widget.position.y + other_widget.size.y)).abs() < threshold {
                let y = other_widget.position.y + other_widget.size.y;
                self.alignment_guides.push(AlignmentGuide {
                    start: Pos2::new(other_widget.position.x.min(position.x) - 20.0, y),
                    end: Pos2::new((other_widget.position.x + other_widget.size.x).max(position.x + size.x) + 20.0, y),
                    guide_type: AlignmentType::WidgetAlignVertical,
                });
            }
        }
    }

    fn apply_snapping(&self, dragging_idx: usize, position: Pos2, size: Vec2) -> Pos2 {
        let mut final_pos = position;
        let snap_threshold = 8.0;
        
        // Snap to canvas center
        if (position.x + size.x / 2.0 - self.canvas_rect.center().x).abs() < snap_threshold {
            final_pos.x = self.canvas_rect.center().x - size.x / 2.0;
        }
        if (position.y + size.y / 2.0 - self.canvas_rect.center().y).abs() < snap_threshold {
            final_pos.y = self.canvas_rect.center().y - size.y / 2.0;
        }
        
        // Snap to other widgets
        for (idx, other_widget) in self.widgets.iter().enumerate() {
            if idx == dragging_idx {
                continue;
            }
            
            let other_center_x = other_widget.position.x + other_widget.size.x / 2.0;
            let other_center_y = other_widget.position.y + other_widget.size.y / 2.0;
            
            // Center alignments
            if (position.x + size.x / 2.0 - other_center_x).abs() < snap_threshold {
                final_pos.x = other_center_x - size.x / 2.0;
            }
            if (position.y + size.y / 2.0 - other_center_y).abs() < snap_threshold {
                final_pos.y = other_center_y - size.y / 2.0;
            }
            
            // Edge alignments
            if (position.x - other_widget.position.x).abs() < snap_threshold {
                final_pos.x = other_widget.position.x;
            }
            if (position.x + size.x - (other_widget.position.x + other_widget.size.x)).abs() < snap_threshold {
                final_pos.x = other_widget.position.x + other_widget.size.x - size.x;
            }
            if (position.y - other_widget.position.y).abs() < snap_threshold {
                final_pos.y = other_widget.position.y;
            }
            if (position.y + size.y - (other_widget.position.y + other_widget.size.y)).abs() < snap_threshold {
                final_pos.y = other_widget.position.y + other_widget.size.y - size.y;
            }
        }
        
        final_pos
    }

    // Removed unused positioning methods for cleaner architecture
    
    // Canvas positioning logic moved to reposition_canvas_widgets for better organization
    
    fn reposition_canvas_widgets(&mut self) {
        // Use the new helper method to get canvas widgets
        let canvas_widget_indices = self.get_canvas_widgets();
        
        // Reposition each canvas widget using proper grid layout
        for (grid_position, &widget_idx) in canvas_widget_indices.iter().enumerate() {
            if let Some(widget) = self.widgets.get(widget_idx) {
                let widget_type = widget.widget_type.clone();
                let new_position = self.calculate_grid_position(grid_position, &widget_type);
                
                // Now update the position
                if let Some(widget_mut) = self.widgets.get_mut(widget_idx) {
                    widget_mut.position = new_position;
                }
            }
        }
    }
    
    fn calculate_grid_position(&self, grid_index: usize, widget_type: &WidgetType) -> Pos2 {
        let widget_size = DraggableWidget::calculate_size(widget_type);
        
        // Use canvas_rect or a safe fallback
        let canvas_rect = if self.canvas_rect == Rect::NOTHING {
            Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0))
        } else {
            self.canvas_rect
        };
        
        // Define the actual usable canvas area with simple margins
        let usable_start_x = canvas_rect.min.x + CANVAS_MARGIN;
        let usable_start_y = canvas_rect.min.y + CANVAS_MARGIN;
        let usable_end_x = (canvas_rect.max.x - CANVAS_MARGIN).max(usable_start_x + 100.0);
        let usable_end_y = (canvas_rect.max.y - CANVAS_MARGIN).max(usable_start_y + 100.0);
        
        // Grid layout with minimum spacing
        let available_width = usable_end_x - usable_start_x;
        let widgets_per_row = (available_width / GRID_SPACING).max(1.0) as usize;
        
        let row = grid_index / widgets_per_row;
        let col = grid_index % widgets_per_row;
        
        let x = usable_start_x + col as f32 * GRID_SPACING;
        let y = usable_start_y + row as f32 * GRID_SPACING;
        
        // Ensure position keeps widget fully within usable bounds
        let max_x = (usable_end_x - widget_size.x).max(usable_start_x);
        let max_y = (usable_end_y - widget_size.y).max(usable_start_y);
        
        Pos2::new(x.min(max_x), y.min(max_y))
    }


    fn is_widget_in_minimized_panel(&self, widget_id: usize) -> bool {
        self.is_widget_in_minimized_panel_recursive(widget_id, &mut std::collections::HashSet::new())
    }
    
    fn is_widget_in_minimized_panel_recursive(&self, widget_id: usize, visited: &mut std::collections::HashSet<usize>) -> bool {
        // Prevent infinite recursion
        if visited.contains(&widget_id) {
            return false;
        }
        visited.insert(widget_id);
        
        // Check if widget is directly in a minimized/collapsed panel
        for widget in &self.widgets {
            match &widget.widget_type {
                WidgetType::Panel { collapsed, contained_widgets, .. } => {
                    if *collapsed && contained_widgets.contains(&widget_id) {
                        return true;
                    }
                }
                WidgetType::Settings { minimized, contained_widgets, .. } => {
                    if *minimized && contained_widgets.contains(&widget_id) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        
        // Check if widget is in a panel that is itself in a minimized panel (nested case)
        if let Some(container_panel_idx) = PanelManager::find_widget_container_panel_id(&self.widgets, widget_id) {
            if let Some(container_panel) = self.widgets.iter().find(|w| w.id == container_panel_idx) {
                return self.is_widget_in_minimized_panel_recursive(container_panel.id, visited);
            }
        }
        
        false
    }

    pub fn show_widget_palette(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.set_min_width(200.0);
            ui.label(RichText::new("Widget Palette").size(16.0).color(WHITE));
            ui.label(RichText::new(format!("Ev2 v{}", APP_VERSION)).size(10.0).color(GRAY_400));
            ui.separator();

            ui.vertical(|ui| {
                // Instructions
                if let Some(_panel_id) = self.selected_panel {
                    ui.colored_label(CYAN, "→ Placing widgets in selected panel");
                } else {
                    ui.label("Click widgets to spawn on canvas");
                    ui.label("Select a panel first to spawn inside it");
                }
                ui.separator();
                
                // Knobs
                let knob_btn = ui.button("🎛️ Knob");
                if knob_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::Knob {
                        value: 50.0,
                        min: 0.0,
                        max: 100.0,
                        label: "KNOB".to_string(),
                        color: WidgetColor::Cyan,
                    });
                }
                
                // Check for drag start on knob button
                if knob_btn.drag_started() || (knob_btn.hovered() && ui.input(|i| i.pointer.primary_pressed())) {
                    self.palette_dragging = Some(WidgetType::Knob {
                        value: 50.0,
                        min: 0.0,
                        max: 100.0,
                        label: "KNOB".to_string(),
                        color: WidgetColor::Cyan,
                    });
                }

                // Toggle Switch
                let toggle_btn = ui.button("🔘 Toggle");
                if toggle_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::ToggleSwitch {
                        on: false,
                        label: "TOGGLE".to_string(),
                        color: WidgetColor::Cyan,
                        glow: true,
                    });
                }
                
                // Check for drag start on toggle button
                if toggle_btn.drag_started() || (toggle_btn.hovered() && ui.input(|i| i.pointer.primary_pressed())) {
                    self.palette_dragging = Some(WidgetType::ToggleSwitch {
                        on: false,
                        label: "TOGGLE".to_string(),
                        color: WidgetColor::Cyan,
                        glow: true,
                    });
                }

                // Push Button
                let button_btn = ui.button("🔳 Button");
                if button_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::PushButton {
                        active: false,
                        icon: "▶".to_string(),
                        label: "PLAY".to_string(),
                        color: WidgetColor::Green,
                        size: 48.0,
                    });
                }
                
                // Check for drag start on button
                if button_btn.drag_started() || (button_btn.hovered() && ui.input(|i| i.pointer.primary_pressed())) {
                    self.palette_dragging = Some(WidgetType::PushButton {
                        active: false,
                        icon: "▶".to_string(),
                        label: "PLAY".to_string(),
                        color: WidgetColor::Green,
                        size: 48.0,
                    });
                }

                // VU Meter
                let vu_btn = ui.button("📊 VU Meter");
                if vu_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::VuMeter {
                        level: 75.0,
                        peak_level: 80.0,
                        label: "VU".to_string(),
                        color: WidgetColor::Green,
                    });
                }
                
                // Check for drag start on VU meter button
                if vu_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::VuMeter {
                        level: 75.0,
                        peak_level: 80.0,
                        label: "VU".to_string(),
                        color: WidgetColor::Green,
                    });
                }

                // Horizontal Slider
                let h_slider_btn = ui.button("━ H.Slider");
                if h_slider_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::HorizontalSlider {
                        value: 60.0,
                        min: 0.0,
                        max: 100.0,
                        label: "LEVEL".to_string(),
                        color: WidgetColor::Yellow,
                    });
                }
                
                // Check for drag start on horizontal slider button
                if h_slider_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::HorizontalSlider {
                        value: 60.0,
                        min: 0.0,
                        max: 100.0,
                        label: "LEVEL".to_string(),
                        color: WidgetColor::Yellow,
                    });
                }

                // Vertical Slider
                let v_slider_btn = ui.button("┃ V.Slider");
                if v_slider_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::VerticalSlider {
                        value: 75.0,
                        min: 0.0,
                        max: 100.0,
                        label: "CH1".to_string(),
                        color: WidgetColor::Pink,
                    });
                }
                
                // Check for drag start on vertical slider button
                if v_slider_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::VerticalSlider {
                        value: 75.0,
                        min: 0.0,
                        max: 100.0,
                        label: "CH1".to_string(),
                        color: WidgetColor::Pink,
                    });
                }

                // Level Indicator
                let level_btn = ui.button("▭▭▭ Level");
                if level_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::LevelIndicator {
                        level: 62.5,
                        segments: 8,
                        label: "INPUT".to_string(),
                    });
                }
                
                // Check for drag start on level indicator button
                if level_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::LevelIndicator {
                        level: 62.5,
                        segments: 8,
                        label: "INPUT".to_string(),
                    });
                }

                // Text Label
                let label_btn = ui.button("🏷️ Label");
                if label_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::TextLabel {
                        text: "LABEL".to_string(),
                        size: 16.0,
                        color: WidgetColor::Cyan,
                    });
                }
                
                // Check for drag start on text label button
                if label_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::TextLabel {
                        text: "LABEL".to_string(),
                        size: 16.0,
                        color: WidgetColor::Cyan,
                    });
                }

                // Panel
                let panel_btn = ui.button("📦 Panel");
                if panel_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::Panel {
                        title: "CONTROL PANEL".to_string(),
                        color: WidgetColor::Cyan,
                        width: 200.0,
                        height: 150.0,
                        collapsed: false,
                        contained_widgets: Vec::new(),
                        minimize_to_settings_icon: true,
                    });
                }
                
                // Check for drag start on panel button
                if panel_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::Panel {
                        title: "CONTROL PANEL".to_string(),
                        color: WidgetColor::Cyan,
                        width: 200.0,
                        height: 150.0,
                        collapsed: false,
                        contained_widgets: Vec::new(),
                        minimize_to_settings_icon: true,
                    });
                }
                
                // Status Bar
                let status_btn = ui.button("📊 Status Bar");
                if status_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::StatusBar {
                        cpu: 23.0,
                        ram: 1.2,
                        latency: 2.3,
                        online: true,
                    });
                }
                
                // Check for drag start on status bar button
                if status_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::StatusBar {
                        cpu: 23.0,
                        ram: 1.2,
                        latency: 2.3,
                        online: true,
                    });
                }
                
                // Settings Widget
                let settings_btn = ui.button("⚙ Settings");
                if settings_btn.clicked() {
                    self.spawn_widget_directly(WidgetType::Settings {
                        label: "SETTINGS".to_string(),
                        color: WidgetColor::Cyan,
                        minimized: false,
                        contained_widgets: Vec::new(),
                    });
                }
                
                // Check for drag start on settings button
                if settings_btn.drag_started() {
                    self.palette_dragging = Some(WidgetType::Settings {
                        label: "SETTINGS".to_string(),
                        color: WidgetColor::Cyan,
                        minimized: false,
                        contained_widgets: Vec::new(),
                    });
                }
                
                ui.separator();
                ui.label("Icon Buttons:");
                
                // Icon buttons
                ui.horizontal_wrapped(|ui| {
                    let power_btn = ui.button("⏻ Power");
                    if power_btn.clicked() {
                        self.spawn_widget_directly(WidgetType::IconButton {
                            icon: IconType::Power,
                            label: "POWER".to_string(),
                            active: false,
                            color: WidgetColor::Green,
                            size: 48.0,
                        });
                    }
                    
                    // Check for drag start on power button
                    if power_btn.drag_started() {
                        self.palette_dragging = Some(WidgetType::IconButton {
                            icon: IconType::Power,
                            label: "POWER".to_string(),
                            active: false,
                            color: WidgetColor::Green,
                            size: 48.0,
                        });
                    }
                    
                    let play_btn = ui.button("▶ Play");
                    if play_btn.clicked() {
                        self.spawn_widget_directly(WidgetType::IconButton {
                            icon: IconType::Play,
                            label: "PLAY".to_string(),
                            active: false,
                            color: WidgetColor::Cyan,
                            size: 48.0,
                        });
                    }
                    
                    // Check for drag start on play button
                    if play_btn.drag_started() {
                        self.palette_dragging = Some(WidgetType::IconButton {
                            icon: IconType::Play,
                            label: "PLAY".to_string(),
                            active: false,
                            color: WidgetColor::Cyan,
                            size: 48.0,
                        });
                    }
                    
                    let pause_btn = ui.button("⏸ Pause");
                    if pause_btn.clicked() {
                        self.spawn_widget_directly(WidgetType::IconButton {
                            icon: IconType::Pause,
                            label: "PAUSE".to_string(),
                            active: false,
                            color: WidgetColor::Cyan,
                            size: 48.0,
                        });
                    }
                    
                    // Check for drag start on pause button
                    if pause_btn.drag_started() {
                        self.palette_dragging = Some(WidgetType::IconButton {
                            icon: IconType::Pause,
                            label: "PAUSE".to_string(),
                            active: false,
                            color: WidgetColor::Cyan,
                            size: 48.0,
                        });
                    }
                    
                    let settings_btn = ui.button("⚙ Settings");
                    if settings_btn.clicked() {
                        self.spawn_widget_directly(WidgetType::IconButton {
                            icon: IconType::Settings,
                            label: "CONFIG".to_string(),
                            active: false,
                            color: WidgetColor::Yellow,
                            size: 48.0,
                        });
                    }
                    
                    // Check for drag start on settings button
                    if settings_btn.drag_started() {
                        self.palette_dragging = Some(WidgetType::IconButton {
                            icon: IconType::Settings,
                            label: "CONFIG".to_string(),
                            active: false,
                            color: WidgetColor::Yellow,
                            size: 48.0,
                        });
                    }
                    
                    let mic_btn = ui.button("🎤 Mic");
                    if mic_btn.clicked() {
                        self.spawn_widget_directly(WidgetType::IconButton {
                            icon: IconType::Mic,
                            label: "MIC".to_string(),
                            active: false,
                            color: WidgetColor::Pink,
                            size: 40.0,
                        });
                    }
                    
                    // Check for drag start on mic button
                    if mic_btn.drag_started() {
                        self.palette_dragging = Some(WidgetType::IconButton {
                            icon: IconType::Mic,
                            label: "MIC".to_string(),
                            active: false,
                            color: WidgetColor::Pink,
                            size: 40.0,
                        });
                    }
                    
                    let mute_btn = ui.button("🔇 Mute");
                    if mute_btn.clicked() {
                        self.spawn_widget_directly(WidgetType::IconButton {
                            icon: IconType::Mute,
                            label: "MUTE".to_string(),
                            active: false,
                            color: WidgetColor::Red,
                            size: 40.0,
                        });
                    }
                    
                    // Check for drag start on mute button
                    if mute_btn.drag_started() {
                        self.palette_dragging = Some(WidgetType::IconButton {
                            icon: IconType::Mute,
                            label: "MUTE".to_string(),
                            active: false,
                            color: WidgetColor::Red,
                            size: 40.0,
                        });
                    }
                });
            });

            ui.separator();
            
            // Canvas Management
            ui.label(RichText::new("Canvas Management").size(14.0).color(YELLOW));
            
            ui.horizontal(|ui| {
                if ui.button("💾 Save Layout").clicked() {
                    self.save_layout();
                }
                if ui.button("🗑️ Clear Canvas").clicked() {
                    self.clear_canvas();
                }
            });
            
            ui.separator();
            
            ui.separator();
            
            // Show drag hint
            ui.label("Drag widgets to panels to organize them");
            
            ui.separator();
            ui.label("Click to add widgets");
            ui.label("Right-click to edit");
            
        });
    }

    fn show_edit_window(&mut self, ui: &mut Ui) {
        if let Some(idx) = self.editing_widget {
            let mut open = self.show_edit_window;
            let mut delete_widget = false;
            
            if let Some(widget) = self.widgets.get_mut(idx) {
                egui::Window::new("Edit Widget")
                    .open(&mut open)
                    .show(ui.ctx(), |ui| {
                        match &mut widget.widget_type {
                            WidgetType::Knob { value, min, max, label, color } => {
                                ui.label("Knob Properties:");
                                ui.add(egui::Slider::new(value, *min..=*max).text("Value"));
                                ui.add(egui::Slider::new(min, 0.0..=100.0).text("Min"));
                                ui.add(egui::Slider::new(max, 0.0..=200.0).text("Max"));
                                ui.text_edit_singleline(label);
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::ToggleSwitch { on, label, color, glow } => {
                                ui.label("Toggle Switch Properties:");
                                ui.checkbox(on, "Current State");
                                ui.checkbox(glow, "Glow Effect");
                                ui.text_edit_singleline(label);
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::PushButton { active, icon, label, color, size } => {
                                ui.label("Push Button Properties:");
                                ui.checkbox(active, "Active State");
                                ui.text_edit_singleline(icon);
                                ui.text_edit_singleline(label);
                                ui.add(egui::Slider::new(size, 20.0..=100.0).text("Size"));
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::VuMeter { level, peak_level, label, color } => {
                                ui.label("VU Meter Properties:");
                                ui.add(egui::Slider::new(level, 0.0..=100.0).text("Level"));
                                ui.add(egui::Slider::new(peak_level, 0.0..=100.0).text("Peak Level"));
                                ui.text_edit_singleline(label);
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::HorizontalSlider { value, min, max, label, color } => {
                                ui.label("Horizontal Slider Properties:");
                                ui.add(egui::Slider::new(value, *min..=*max).text("Value"));
                                ui.add(egui::Slider::new(min, 0.0..=100.0).text("Min"));
                                ui.add(egui::Slider::new(max, 0.0..=200.0).text("Max"));
                                ui.text_edit_singleline(label);
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::VerticalSlider { value, min, max, label, color } => {
                                ui.label("Vertical Slider Properties:");
                                ui.add(egui::Slider::new(value, *min..=*max).text("Value"));
                                ui.add(egui::Slider::new(min, 0.0..=100.0).text("Min"));
                                ui.add(egui::Slider::new(max, 0.0..=200.0).text("Max"));
                                ui.text_edit_singleline(label);
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::LevelIndicator { level, segments, label } => {
                                ui.label("Level Indicator Properties:");
                                ui.add(egui::Slider::new(level, 0.0..=100.0).text("Level"));
                                ui.add(egui::Slider::new(segments, 4..=16).text("Segments"));
                                ui.text_edit_singleline(label);
                            }
                            WidgetType::TextLabel { text, size, color } => {
                                ui.label("Text Label Properties:");
                                ui.text_edit_singleline(text);
                                ui.add(egui::Slider::new(size, 8.0..=32.0).text("Font Size"));
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::Panel { title, color, width, height, contained_widgets, minimize_to_settings_icon, .. } => {
                                ui.label("Panel Properties:");
                                ui.text_edit_singleline(title);
                                ui.add(egui::Slider::new(width, 100.0..=400.0).text("Width"));
                                ui.add(egui::Slider::new(height, 100.0..=300.0).text("Height"));
                                ui.checkbox(minimize_to_settings_icon, "Minimize to ⚙");
                                ui.label(format!("Contains {} widgets", contained_widgets.len()));
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::StatusBar { cpu, ram, latency, online } => {
                                ui.label("Status Bar Properties:");
                                ui.add(egui::Slider::new(cpu, 0.0..=100.0).text("CPU %"));
                                ui.add(egui::Slider::new(ram, 0.0..=8.0).text("RAM (GB)"));
                                ui.add(egui::Slider::new(latency, 0.0..=100.0).text("Latency (ms)"));
                                ui.checkbox(online, "System Online");
                            }
                            WidgetType::IconButton { icon, label, active, color, size } => {
                                ui.label("Icon Button Properties:");
                                ui.checkbox(active, "Active State");
                                ui.text_edit_singleline(label);
                                ui.add(egui::Slider::new(size, 20.0..=80.0).text("Size"));
                                
                                ui.horizontal(|ui| {
                                    ui.label("Icon:");
                                    ui.selectable_value(icon, IconType::Power, "Power");
                                    ui.selectable_value(icon, IconType::Play, "Play");
                                    ui.selectable_value(icon, IconType::Pause, "Pause");
                                });
                                ui.horizontal(|ui| {
                                    ui.selectable_value(icon, IconType::Settings, "Settings");
                                    ui.selectable_value(icon, IconType::Mic, "Mic");
                                    ui.selectable_value(icon, IconType::Mute, "Mute");
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                            WidgetType::Settings { label, color, minimized, .. } => {
                                ui.label("Settings Properties:");
                                ui.text_edit_singleline(label);
                                ui.checkbox(minimized, "Minimized");
                                ui.horizontal(|ui| {
                                    ui.label("Color:");
                                    if ui.radio_value(color, WidgetColor::Cyan, "Cyan").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Pink, "Pink").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Green, "Green").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Yellow, "Yellow").clicked() {}
                                    if ui.radio_value(color, WidgetColor::Red, "Red").clicked() {}
                                });
                            }
                        }
                        
                        ui.separator();
                        if ui.button("Delete Widget").clicked() {
                            delete_widget = true;
                        }
                    });
            }
            
            self.show_edit_window = open;
            
            if delete_widget {
                self.widgets.remove(idx);
                self.editing_widget = None;
                self.show_edit_window = false;
            }
        } else {
            self.show_edit_window = false;
        }
        
        if !self.show_edit_window {
            self.editing_widget = None;
        }
    }
    
    fn render_settings_icon(&self, ui: &mut Ui) {
        let _icon_size = 30.0;
        let padding = 15.0;
        let icon_pos = Pos2::new(padding, padding);
        
        let painter = ui.painter();
        
        // Simple static settings icon
        painter.text(
            icon_pos,
            Align2::LEFT_TOP,
            "⚙",
            FontId::monospace(20.0),
            Color32::from_rgba_unmultiplied(156, 163, 175, 200), // Semi-transparent gray
        );
    }
    
    
    pub fn save_layout(&self) {
        // For now, just print to console - could be extended to save to file
        println!("💾 Layout saved! {} widgets on canvas", self.widgets.len());
        
        // In a real implementation, you would serialize self.widgets and self.config_panel.items
        // and save them to a file or local storage
        
        // Example of what could be saved:
        for (i, widget) in self.widgets.iter().enumerate() {
            println!("  Widget {}: {:?} at {:?}", i, widget.widget_type, widget.position);
        }
    }
    
    pub fn clear_canvas(&mut self) {
        self.widgets.clear();
        println!("🗑️ Canvas cleared!");
    }
    
    
    // Legacy drop logic removed
    
}