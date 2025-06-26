//! # Drag and Drop Canvas
//! 
//! A comprehensive drag-and-drop audio control widget system built with egui.

use egui::{Color32, Pos2, Rect, Ui, Vec2, FontId, Align2, Stroke};
use std::f32::consts::PI;
use serde::{Deserialize, Serialize};

// Application version
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Positioning constants for better maintainability
const PANEL_MARGIN: f32 = 10.0;
const PANEL_TITLE_HEIGHT: f32 = 40.0;
const CANVAS_MARGIN: f32 = 20.0;
const GRID_SPACING: f32 = 120.0;
const WIDGET_SPACING_IN_PANEL: f32 = 70.0;

// Exact React color palette
const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
const CYAN: Color32 = Color32::from_rgb(6, 182, 212);
const PINK: Color32 = Color32::from_rgb(236, 72, 153);
const GREEN: Color32 = Color32::from_rgb(16, 185, 129);
const YELLOW: Color32 = Color32::from_rgb(245, 158, 11);
const RED: Color32 = Color32::from_rgb(239, 68, 68);
const GRAY_900: Color32 = Color32::from_rgb(17, 24, 39);
const GRAY_800: Color32 = Color32::from_rgb(31, 41, 55);
const GRAY_700: Color32 = Color32::from_rgb(55, 65, 81);
const GRAY_600: Color32 = Color32::from_rgb(75, 85, 99);
const GRAY_400: Color32 = Color32::from_rgb(156, 163, 175);
const WHITE: Color32 = Color32::WHITE;

/// Color themes for widgets matching the React app palette
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WidgetColor {
    Cyan,
    Pink,
    Green,
    Yellow,
    Red,
}

impl WidgetColor {
    pub fn to_color32(&self) -> Color32 {
        match self {
            WidgetColor::Cyan => CYAN,
            WidgetColor::Pink => PINK,
            WidgetColor::Green => GREEN,
            WidgetColor::Yellow => YELLOW,
            WidgetColor::Red => RED,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IconType {
    Power,
    Play,
    Pause,
    SkipBack,
    SkipForward,
    Volume,
    Mic,
    Settings,
    Mute,
    Zap,
}

/// All supported widget types with their configuration parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    Knob { value: f32, min: f32, max: f32, label: String, color: WidgetColor },
    ToggleSwitch { on: bool, label: String, color: WidgetColor, glow: bool },
    PushButton { active: bool, icon: String, label: String, color: WidgetColor, size: f32 },
    VuMeter { level: f32, peak_level: f32, label: String, color: WidgetColor },
    HorizontalSlider { value: f32, min: f32, max: f32, label: String, color: WidgetColor },
    VerticalSlider { value: f32, min: f32, max: f32, label: String, color: WidgetColor },
    LevelIndicator { level: f32, segments: usize, label: String },
    TextLabel { text: String, size: f32, color: WidgetColor },
    Panel { title: String, color: WidgetColor, width: f32, height: f32, collapsed: bool, contained_widgets: Vec<usize>, minimize_to_settings_icon: bool },
    StatusBar { cpu: f32, ram: f32, latency: f32, online: bool },
    IconButton { icon: IconType, label: String, active: bool, color: WidgetColor, size: f32 },
    Settings { label: String, color: WidgetColor },
}

/// A widget instance with position, size, and type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraggableWidget {
    pub id: usize,
    pub widget_type: WidgetType,
    pub position: Pos2,
    pub size: Vec2,
}

impl DraggableWidget {
    pub fn new(id: usize, widget_type: WidgetType, position: Pos2) -> Self {
        let size = Self::calculate_size(&widget_type);
        Self {
            id,
            widget_type,
            position,
            size,
        }
    }

    pub fn calculate_size(widget_type: &WidgetType) -> Vec2 {
        match widget_type {
            WidgetType::Knob { .. } => Vec2::new(104.0, 124.0),
            WidgetType::ToggleSwitch { .. } => Vec2::new(68.0, 49.0),
            WidgetType::PushButton { size, .. } => Vec2::new(size + 10.0, size + 30.0),
            WidgetType::VuMeter { .. } => Vec2::new(26.0, 158.0),
            WidgetType::HorizontalSlider { .. } => Vec2::new(176.0, 28.0),
            WidgetType::VerticalSlider { .. } => Vec2::new(28.0, 146.0),
            WidgetType::LevelIndicator { .. } => Vec2::new(120.0, 40.0),
            WidgetType::TextLabel { size, .. } => Vec2::new(size * 8.0, size * 1.5),
            WidgetType::Panel { width, height, collapsed, minimize_to_settings_icon, .. } => {
                if *collapsed {
                    if *minimize_to_settings_icon {
                        Vec2::new(40.0, 40.0) // Settings icon size when minimized with special setting
                    } else {
                        Vec2::new(*width, 40.0) // Just title bar height when collapsed normally
                    }
                } else {
                    Vec2::new(*width, *height)
                }
            },
            WidgetType::StatusBar { .. } => Vec2::new(400.0, 60.0),
            WidgetType::IconButton { size, .. } => Vec2::new(size + 10.0, size + 30.0),
            WidgetType::Settings { .. } => Vec2::new(40.0, 40.0),
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::from_min_size(self.position, self.size)
    }
}

/// Main canvas structure managing all widgets and interactions
pub struct DragDropCanvas {
    pub widgets: Vec<DraggableWidget>,
    pub next_widget_id: usize,
    pub selected_panel: Option<usize>,
    pub pending_widget: Option<WidgetType>,
    pub dragging_widget: Option<usize>,
    pub drag_offset: Vec2,
}

impl DragDropCanvas {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            next_widget_id: 0,
            selected_panel: None,
            pending_widget: None,
            dragging_widget: None,
            drag_offset: Vec2::ZERO,
        }
    }

    pub fn add_widget(&mut self, widget_type: WidgetType, position: Pos2) {
        let widget = DraggableWidget::new(self.next_widget_id, widget_type, position);
        self.widgets.push(widget);
        self.next_widget_id += 1;
    }

    pub fn show_widget_palette(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.heading("Widget Palette");
            ui.separator();
            
            if let Some(ref widget_type) = self.pending_widget {
                ui.colored_label(CYAN, format!("→ {} selected - click to place", self.get_widget_name(widget_type)));
                ui.separator();
            }

            if ui.button("Knob").clicked() {
                self.pending_widget = Some(WidgetType::Knob {
                    value: 50.0,
                    min: 0.0,
                    max: 100.0,
                    label: "KNOB".to_string(),
                    color: WidgetColor::Cyan,
                });
            }

            if ui.button("Panel").clicked() {
                self.pending_widget = Some(WidgetType::Panel {
                    title: "Panel".to_string(),
                    color: WidgetColor::Cyan,
                    width: 200.0,
                    height: 150.0,
                    collapsed: false,
                    contained_widgets: Vec::new(),
                    minimize_to_settings_icon: false,
                });
            }

            if ui.button("Settings").clicked() {
                self.pending_widget = Some(WidgetType::Settings {
                    label: "Settings".to_string(),
                    color: WidgetColor::Cyan,
                });
            }
        });
    }

    fn get_widget_name(&self, widget_type: &WidgetType) -> &'static str {
        match widget_type {
            WidgetType::Knob { .. } => "Knob",
            WidgetType::Panel { .. } => "Panel", 
            WidgetType::Settings { .. } => "Settings",
            _ => "Widget",
        }
    }

    pub fn render(&mut self, ui: &mut Ui) {
        let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
        let _canvas_rect = response.rect;

        // Handle clicks for widget placement
        if response.clicked() {
            if let Some(click_pos) = response.interact_pointer_pos() {
                if let Some(widget_type) = self.pending_widget.take() {
                    // Place widget where clicked, respecting panel selection
                    self.place_widget_at_position(widget_type, click_pos);
                } else {
                    // Handle panel selection
                    self.handle_panel_selection(click_pos);
                }
            }
        }

        // Draw canvas background with selection highlighting
        let painter = ui.painter();
        
        // TODO: Add panel highlighting once we figure out the egui API
        // For now, just render without highlighting

        // Render all widgets
        for widget in &self.widgets {
            Self::render_widget_static(widget, &painter);
        }

        // Handle dragging
        if response.dragged() {
            let drag_delta = response.drag_delta();
            if drag_delta != Vec2::ZERO {
                self.handle_dragging(drag_delta);
            }
        }

        if response.drag_stopped() {
            self.dragging_widget = None;
        }
    }

    fn place_widget_at_position(&mut self, widget_type: WidgetType, position: Pos2) {
        // Adjust position if it would cause overlap
        let final_position = self.find_non_overlapping_position(position, &widget_type);
        self.add_widget(widget_type, final_position);
    }

    fn find_non_overlapping_position(&self, desired_pos: Pos2, widget_type: &WidgetType) -> Pos2 {
        let size = DraggableWidget::calculate_size(widget_type);
        let desired_rect = Rect::from_min_size(desired_pos, size);
        let padding = 1.0; // 1-pixel padding as requested

        // Check for overlaps and adjust position if needed
        for widget in &self.widgets {
            let widget_rect = widget.get_rect().expand(padding);
            if widget_rect.intersects(desired_rect) {
                // Simple offset strategy - move right and down
                return Pos2::new(
                    widget_rect.max.x + padding,
                    widget.position.y
                );
            }
        }

        desired_pos
    }

    fn handle_panel_selection(&mut self, click_pos: Pos2) {
        // Find if clicking on a panel
        for widget in &self.widgets {
            if widget.get_rect().contains(click_pos) {
                if let WidgetType::Panel { .. } = widget.widget_type {
                    self.selected_panel = Some(widget.id);
                    return;
                }
            }
        }
        
        // Clicked on empty space - deselect panel
        self.selected_panel = None;
    }

    fn handle_dragging(&mut self, drag_delta: Vec2) {
        if self.dragging_widget.is_none() {
            // Start dragging - find widget under cursor
            // This is simplified - in real implementation you'd check mouse position
        }
        
        if let Some(widget_idx) = self.dragging_widget {
            if let Some(widget) = self.widgets.get_mut(widget_idx) {
                widget.position += drag_delta;
            }
        }
    }

    fn render_widget_static(widget: &DraggableWidget, painter: &egui::Painter) {
        let rect = widget.get_rect();
        
        match &widget.widget_type {
            WidgetType::Knob { value, label, color, .. } => {
                Self::render_knob(painter, rect, *value, label, *color);
            }
            WidgetType::Panel { title, color, collapsed, .. } => {
                Self::render_panel(painter, rect, title, *color, *collapsed);
            }
            WidgetType::Settings { label, color } => {
                Self::render_settings_icon(painter, rect, label, *color);
            }
            _ => {
                // Placeholder for other widget types
                painter.rect_filled(rect, 5.0, color32_with_alpha(GRAY_700, 200));
                painter.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "Widget",
                    FontId::default(),
                    WHITE,
                );
            }
        }
    }

    fn render_knob(painter: &egui::Painter, rect: Rect, value: f32, label: &str, color: WidgetColor) {
        let center = rect.center();
        let radius = 30.0;
        
        // Background circle
        painter.circle_filled(center, radius, GRAY_800);
        painter.circle_stroke(center, radius, Stroke::new(2.0, color.to_color32()));
        
        // Value indicator
        let angle = -PI * 0.75 + (value / 100.0) * PI * 1.5;
        let indicator_end = center + Vec2::new(angle.cos(), angle.sin()) * (radius - 5.0);
        painter.line_segment([center, indicator_end], Stroke::new(3.0, WHITE));
        
        // Label
        painter.text(
            Pos2::new(center.x, rect.max.y - 15.0),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(12.0),
            WHITE,
        );
    }

    fn render_panel(painter: &egui::Painter, rect: Rect, title: &str, color: WidgetColor, _collapsed: bool) {
        // Panel background  
        painter.rect_filled(rect, 5.0, GRAY_800);
        // TODO: Add stroke once we figure out the egui API
        
        // Title bar
        let title_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), PANEL_TITLE_HEIGHT));
        painter.rect_filled(title_rect, 5.0, color32_with_alpha(color.to_color32(), 100));
        
        // Title text
        painter.text(
            title_rect.center(),
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(14.0),
            WHITE,
        );
    }

    fn render_settings_icon(painter: &egui::Painter, rect: Rect, _label: &str, color: WidgetColor) {
        // Simple gear icon representation
        painter.circle_filled(rect.center(), 18.0, color.to_color32());
        painter.circle_filled(rect.center(), 8.0, GRAY_800);
        
        // Gear teeth (simplified)
        for i in 0..8 {
            let angle = (i as f32 / 8.0) * 2.0 * PI;
            let tooth_start = rect.center() + Vec2::new(angle.cos(), angle.sin()) * 15.0;
            let tooth_end = rect.center() + Vec2::new(angle.cos(), angle.sin()) * 20.0;
            painter.line_segment([tooth_start, tooth_end], Stroke::new(2.0, color.to_color32()));
        }
    }
}

fn color32_with_alpha(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}