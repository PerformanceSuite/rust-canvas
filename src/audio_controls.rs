use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget, FontId, Align2, RichText};
use std::f32::consts::PI;

// Color constants matching React version
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
#[allow(dead_code)]
const GRAY_200: Color32 = Color32::from_rgb(229, 231, 235);

pub struct Knob<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
    label: &'a str,
    color: Color32,
    size: f32,
}

impl<'a> Knob<'a> {
    pub fn new(value: &'a mut f32, label: &'a str) -> Self {
        Self {
            value,
            min: 0.0,
            max: 100.0,
            label,
            color: CYAN,
            size: 64.0,
        }
    }

    #[allow(dead_code)]
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Widget for Knob<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(self.size + 40.0, self.size + 60.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::drag());

        if response.dragged() {
            let delta = response.drag_delta();
            let delta_value = -delta.y * (self.max - self.min) / 100.0;
            *self.value = (*self.value + delta_value).clamp(self.min, self.max);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let knob_rect = Rect::from_center_size(
                Pos2::new(rect.center().x, rect.top() + self.size / 2.0 + 5.0),
                Vec2::splat(self.size),
            );
            let center = knob_rect.center();
            let radius = knob_rect.width() / 2.0;
            let normalized = (*self.value - self.min) / (self.max - self.min);
            let angle = normalized * 270.0 * PI / 180.0 - 135.0 * PI / 180.0;

            let painter = ui.painter();

            // Draw outer ring with gradient effect
            painter.circle_filled(center, radius, GRAY_900);
            painter.circle_stroke(center, radius, Stroke::new(4.0, GRAY_700));

            // Draw progress arc
            let arc_points = 32;
            let start_angle = -135.0 * PI / 180.0;
            let end_angle = start_angle + normalized * 270.0 * PI / 180.0;
            
            for i in 0..arc_points {
                let t = i as f32 / (arc_points - 1) as f32;
                let a = start_angle + t * (end_angle - start_angle);
                let inner_radius = radius - 8.0;
                let outer_radius = radius - 4.0;
                
                let inner_pos = center + Vec2::new(a.cos() * inner_radius, a.sin() * inner_radius);
                let outer_pos = center + Vec2::new(a.cos() * outer_radius, a.sin() * outer_radius);
                
                painter.line_segment([inner_pos, outer_pos], Stroke::new(2.0, self.color));
            }

            // Draw inner circle
            painter.circle_filled(center, radius - 12.0, GRAY_900);

            // Draw indicator line
            let indicator_length = radius - 16.0;
            let indicator_pos = center + Vec2::new(
                angle.cos() * indicator_length,
                angle.sin() * indicator_length,
            );
            painter.line_segment(
                [center, indicator_pos],
                Stroke::new(4.0, self.color),
            );

            // Draw center dot
            painter.circle_filled(center, 4.0, self.color);

            // Draw label
            painter.text(
                Pos2::new(center.x, rect.bottom() - 30.0),
                Align2::CENTER_CENTER,
                self.label,
                FontId::monospace(10.0),
                GRAY_400,
            );

            // Draw value
            painter.text(
                Pos2::new(center.x, rect.bottom() - 15.0),
                Align2::CENTER_CENTER,
                format!("{:.1}", self.value),
                FontId::monospace(10.0),
                self.color,
            );
        }

        response
    }
}

pub struct ToggleSwitch<'a> {
    on: &'a mut bool,
    label: &'a str,
    color: Color32,
    size: Vec2,
}

impl<'a> ToggleSwitch<'a> {
    pub fn new(on: &'a mut bool, label: &'a str) -> Self {
        Self {
            on,
            label,
            color: CYAN,
            size: Vec2::new(48.0, 24.0),
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Widget for ToggleSwitch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(self.size.x, self.size.y + 25.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let switch_rect = Rect::from_center_size(
                Pos2::new(rect.center().x, rect.top() + self.size.y / 2.0 + 5.0),
                self.size,
            );
            let painter = ui.painter();
            let radius = switch_rect.height() / 2.0;
            
            let bg_color = if *self.on {
                self.color
            } else {
                GRAY_700
            };

            // Draw switch background
            painter.rect_filled(switch_rect, radius, bg_color);

            // Draw switch handle
            let handle_radius = radius - 2.0;
            let handle_x = if *self.on {
                switch_rect.right() - radius
            } else {
                switch_rect.left() + radius
            };

            painter.circle_filled(
                Pos2::new(handle_x, switch_rect.center().y),
                handle_radius,
                Color32::WHITE,
            );

            // Add shadow effect for active state
            if *self.on {
                painter.circle_filled(
                    Pos2::new(handle_x, switch_rect.center().y + 1.0),
                    handle_radius,
                    Color32::from_rgba_unmultiplied(0, 0, 0, 50),
                );
            }

            // Draw label
            if !self.label.is_empty() {
                painter.text(
                    Pos2::new(rect.center().x, rect.bottom() - 10.0),
                    Align2::CENTER_CENTER,
                    self.label,
                    FontId::monospace(10.0),
                    GRAY_400,
                );
            }
        }

        response
    }
}

pub struct PushButton<'a> {
    active: &'a mut bool,
    icon: &'a str,
    label: &'a str,
    color: Color32,
    size: f32,
}

impl<'a> PushButton<'a> {
    pub fn new(active: &'a mut bool, icon: &'a str, label: &'a str) -> Self {
        Self {
            active,
            icon,
            label,
            color: CYAN,
            size: 48.0,
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Widget for PushButton<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(self.size + 10.0, self.size + 30.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

        if response.clicked() {
            *self.active = !*self.active;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let button_rect = Rect::from_center_size(
                Pos2::new(rect.center().x, rect.top() + self.size / 2.0 + 5.0),
                Vec2::splat(self.size),
            );
            let painter = ui.painter();

            let (fill_color, stroke_color) = if *self.active {
                (Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), 60), self.color)
            } else {
                (GRAY_800, GRAY_600)
            };

            // Draw button background
            painter.rect_filled(button_rect, 12.0, fill_color);
            
            // Draw border
            let border_rect = button_rect.expand(1.0);
            painter.rect_filled(border_rect, 13.0, Color32::TRANSPARENT);
            let stroke_width = 2.0;
            for i in 0..4 {
                let corner_rect = match i {
                    0 => Rect::from_min_size(border_rect.min, Vec2::new(stroke_width, border_rect.height())),
                    1 => Rect::from_min_size(Pos2::new(border_rect.max.x - stroke_width, border_rect.min.y), Vec2::new(stroke_width, border_rect.height())),
                    2 => Rect::from_min_size(border_rect.min, Vec2::new(border_rect.width(), stroke_width)),
                    _ => Rect::from_min_size(Pos2::new(border_rect.min.x, border_rect.max.y - stroke_width), Vec2::new(border_rect.width(), stroke_width)),
                };
                painter.rect_filled(corner_rect, 0.0, stroke_color);
            }

            // Add glow effect for active state
            if *self.active {
                let glow_rect = button_rect.expand(3.0);
                let glow_color = Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), 50);
                painter.rect_filled(glow_rect, 15.0, Color32::TRANSPARENT);
                for i in 0..4 {
                    let glow_edge = match i {
                        0 => Rect::from_min_size(glow_rect.min, Vec2::new(1.0, glow_rect.height())),
                        1 => Rect::from_min_size(Pos2::new(glow_rect.max.x - 1.0, glow_rect.min.y), Vec2::new(1.0, glow_rect.height())),
                        2 => Rect::from_min_size(glow_rect.min, Vec2::new(glow_rect.width(), 1.0)),
                        _ => Rect::from_min_size(Pos2::new(glow_rect.min.x, glow_rect.max.y - 1.0), Vec2::new(glow_rect.width(), 1.0)),
                    };
                    painter.rect_filled(glow_edge, 0.0, glow_color);
                }
            }

            // Draw icon
            let icon_color = if *self.active { self.color } else { GRAY_400 };
            painter.text(
                button_rect.center(),
                Align2::CENTER_CENTER,
                self.icon,
                FontId::monospace(20.0),
                icon_color,
            );

            // Draw label
            painter.text(
                Pos2::new(rect.center().x, rect.bottom() - 10.0),
                Align2::CENTER_CENTER,
                self.label,
                FontId::monospace(8.0),
                GRAY_400,
            );
        }

        response
    }
}

pub struct VuMeter<'a> {
    level: &'a f32,
    peak_level: &'a mut f32,
    label: &'a str,
    color: Color32,
    width: f32,
    height: f32,
}

impl<'a> VuMeter<'a> {
    pub fn new(level: &'a f32, peak_level: &'a mut f32, label: &'a str) -> Self {
        Self {
            level,
            peak_level,
            label,
            color: GREEN,
            width: 16.0,
            height: 128.0,
        }
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

impl<'a> Widget for VuMeter<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::new(self.width + 10.0, self.height + 30.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());

        // Update peak level
        if *self.level > *self.peak_level {
            *self.peak_level = *self.level;
        } else {
            *self.peak_level = (*self.peak_level - 0.5).max(0.0);
        }

        if ui.is_rect_visible(rect) {
            let meter_rect = Rect::from_center_size(
                Pos2::new(rect.center().x, rect.top() + self.height / 2.0 + 5.0),
                Vec2::new(self.width, self.height),
            );
            let painter = ui.painter();

            // Draw background
            painter.rect_filled(meter_rect, 4.0, GRAY_800);
            
            // Draw border manually
            let border_width = 1.0;
            let border_color = GRAY_600;
            for i in 0..4 {
                let border_edge = match i {
                    0 => Rect::from_min_size(meter_rect.min, Vec2::new(border_width, meter_rect.height())),
                    1 => Rect::from_min_size(Pos2::new(meter_rect.max.x - border_width, meter_rect.min.y), Vec2::new(border_width, meter_rect.height())),
                    2 => Rect::from_min_size(meter_rect.min, Vec2::new(meter_rect.width(), border_width)),
                    _ => Rect::from_min_size(Pos2::new(meter_rect.min.x, meter_rect.max.y - border_width), Vec2::new(meter_rect.width(), border_width)),
                };
                painter.rect_filled(border_edge, 0.0, border_color);
            }

            // Draw level segments
            let segments = 20;
            let segment_height = self.height / segments as f32;
            let current_segments = ((*self.level / 100.0) * segments as f32) as usize;

            for i in 0..segments {
                let segment_rect = Rect::from_min_size(
                    Pos2::new(
                        meter_rect.left() + 2.0,
                        meter_rect.bottom() - (i + 1) as f32 * segment_height,
                    ),
                    Vec2::new(self.width - 4.0, segment_height - 1.0),
                );

                if i < current_segments {
                    let color = if i >= 18 {
                        RED
                    } else if i >= 14 {
                        YELLOW
                    } else {
                        self.color
                    };
                    painter.rect_filled(segment_rect, 1.0, color);
                }
            }

            // Draw peak indicator
            if *self.peak_level > 0.0 {
                let peak_y = meter_rect.bottom() - (*self.peak_level / 100.0) * self.height;
                painter.line_segment(
                    [
                        Pos2::new(meter_rect.left() + 2.0, peak_y),
                        Pos2::new(meter_rect.right() - 2.0, peak_y),
                    ],
                    Stroke::new(2.0, Color32::WHITE),
                );
            }

            // Draw label
            painter.text(
                Pos2::new(rect.center().x, rect.bottom() - 10.0),
                Align2::CENTER_CENTER,
                self.label,
                FontId::monospace(10.0),
                GRAY_400,
            );
        }

        response
    }
}

pub struct Slider<'a> {
    value: &'a mut f32,
    min: f32,
    max: f32,
    label: &'a str,
    color: Color32,
    vertical: bool,
    size: Vec2,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32, label: &'a str) -> Self {
        Self {
            value,
            min: 0.0,
            max: 100.0,
            label,
            color: CYAN,
            vertical: false,
            size: Vec2::new(96.0, 8.0),
        }
    }

    #[allow(dead_code)]
    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        if vertical {
            self.size = Vec2::new(8.0, 96.0);
        }
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}

impl<'a> Widget for Slider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = if self.vertical {
            Vec2::new(self.size.x + 20.0, self.size.y + 50.0)
        } else {
            Vec2::new(self.size.x + 80.0, self.size.y + 20.0)
        };
        let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::drag());

        if response.dragged() {
            let delta = response.drag_delta();
            let delta_value = if self.vertical {
                -delta.y * (self.max - self.min) / self.size.y
            } else {
                delta.x * (self.max - self.min) / self.size.x
            };
            *self.value = (*self.value + delta_value).clamp(self.min, self.max);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let normalized = (*self.value - self.min) / (self.max - self.min);

            if self.vertical {
                let slider_rect = Rect::from_center_size(
                    Pos2::new(rect.center().x, rect.center().y - 10.0),
                    self.size,
                );

                // Draw background
                painter.rect_filled(slider_rect, 4.0, GRAY_700);

                // Draw filled portion
                let fill_height = slider_rect.height() * normalized;
                let fill_rect = Rect::from_min_size(
                    Pos2::new(slider_rect.left(), slider_rect.bottom() - fill_height),
                    Vec2::new(slider_rect.width(), fill_height),
                );
                painter.rect_filled(fill_rect, 4.0, self.color);

                // Draw value
                painter.text(
                    Pos2::new(rect.center().x, rect.bottom() - 15.0),
                    Align2::CENTER_CENTER,
                    format!("{:.0}", self.value),
                    FontId::monospace(8.0),
                    self.color,
                );
            } else {
                // Draw label
                painter.text(
                    Pos2::new(rect.left() + 25.0, rect.center().y),
                    Align2::CENTER_CENTER,
                    self.label,
                    FontId::monospace(10.0),
                    GRAY_400,
                );

                let slider_rect = Rect::from_center_size(
                    Pos2::new(rect.center().x + 10.0, rect.center().y),
                    self.size,
                );

                // Draw background
                painter.rect_filled(slider_rect, 4.0, GRAY_700);

                // Draw filled portion
                let fill_width = slider_rect.width() * normalized;
                let fill_rect = Rect::from_min_size(
                    slider_rect.min,
                    Vec2::new(fill_width, slider_rect.height()),
                );
                painter.rect_filled(fill_rect, 4.0, self.color);

                // Draw value
                painter.text(
                    Pos2::new(rect.right() - 15.0, rect.center().y),
                    Align2::CENTER_CENTER,
                    format!("{:.0}", self.value),
                    FontId::monospace(10.0),
                    self.color,
                );
            }
        }

        response
    }
}

pub struct LevelIndicator {
    level: f32,
    segments: usize,
    colors: Vec<Color32>,
    size: Vec2,
}

impl LevelIndicator {
    pub fn new(level: f32) -> Self {
        Self {
            level,
            segments: 8,
            colors: vec![GREEN, GREEN, GREEN, GREEN, GREEN, YELLOW, YELLOW, RED],
            size: Vec2::new(16.0, 64.0),
        }
    }

    #[allow(dead_code)]
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    #[allow(dead_code)]
    pub fn colors(mut self, colors: Vec<Color32>) -> Self {
        self.colors = colors;
        self
    }

    #[allow(dead_code)]
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }
}

impl Widget for LevelIndicator {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let segment_width = (self.size.x - (self.segments - 1) as f32) / self.segments as f32;
            let active_segments = ((self.level / 100.0) * self.segments as f32) as usize;

            for i in 0..self.segments {
                let x = rect.left() + i as f32 * (segment_width + 1.0);
                let segment_rect = Rect::from_min_size(
                    Pos2::new(x, rect.top()),
                    Vec2::new(segment_width, self.size.y),
                );

                let color = if i < active_segments {
                    self.colors.get(i).copied().unwrap_or(GREEN)
                } else {
                    GRAY_600
                };

                painter.rect_filled(segment_rect, 1.0, color);
            }
        }

        response
    }
}

pub fn show_audio_controls(ui: &mut Ui, state: &mut AudioControlState) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Header
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(RichText::new("AUDIO CONTROL MATRIX")
                .size(24.0)
                .color(Color32::WHITE)
                .font(FontId::monospace(24.0)));
            ui.add_space(5.0);
            
            // Gradient line
            let line_rect = Rect::from_min_size(
                Pos2::new(ui.available_rect_before_wrap().center().x - 64.0, ui.cursor().top()),
                Vec2::new(128.0, 4.0),
            );
            ui.painter().rect_filled(line_rect, 2.0, CYAN);
            ui.add_space(15.0);
        });

        // Main control panels
        ui.horizontal(|ui| {
            // Left Panel - Master Controls
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(Vec2::new(200.0, 400.0));
                    ui.label(RichText::new("MASTER CONTROL")
                        .size(16.0)
                        .color(CYAN)
                        .font(FontId::monospace(16.0)));
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(Knob::new(&mut state.master_volume, "VOLUME").color(CYAN));
                        ui.add(Knob::new(&mut state.master_gain, "GAIN").color(PINK));
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(PushButton::new(&mut state.power, "⚡", "POWER").color(GREEN));
                        let play_icon = if state.playing { "⏸" } else { "▶" };
                        let play_label = if state.playing { "PAUSE" } else { "PLAY" };
                        ui.add(PushButton::new(&mut state.playing, play_icon, play_label).color(CYAN).size(56.0));
                        ui.add(PushButton::new(&mut state.config, "⚙", "CONFIG").color(YELLOW));
                    });

                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.add(PushButton::new(&mut state.prev, "⏮", "PREV").color(CYAN).size(32.0));
                        ui.add(PushButton::new(&mut state.mute, "🔊", "MUTE").color(CYAN).size(32.0));
                        ui.add(PushButton::new(&mut state.mic, "🎤", "MIC").color(CYAN).size(32.0));
                        ui.add(PushButton::new(&mut state.next, "⏭", "NEXT").color(CYAN).size(32.0));
                    });
                });
            });

            ui.add_space(10.0);

            // Center Panel - EQ & Effects
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(Vec2::new(200.0, 400.0));
                    ui.label(RichText::new("EQ & EFFECTS")
                        .size(16.0)
                        .color(PINK)
                        .font(FontId::monospace(16.0)));
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(Knob::new(&mut state.bass, "BASS").range(-20.0, 20.0).color(GREEN));
                        ui.add(Knob::new(&mut state.treble, "TREBLE").range(-20.0, 20.0).color(YELLOW));
                    });

                    ui.add_space(10.0);

                    ui.add(Slider::new(&mut state.low_eq, "LOW").color(GREEN));
                    ui.add(Slider::new(&mut state.mid_eq, "MID").color(YELLOW));
                    ui.add(Slider::new(&mut state.high_eq, "HIGH").color(PINK));

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(ToggleSwitch::new(&mut state.reverb, "REVERB").color(CYAN));
                        ui.add(ToggleSwitch::new(&mut state.echo, "ECHO").color(PINK));
                        ui.add(ToggleSwitch::new(&mut state.eq, "EQ").color(GREEN));
                    });
                });
            });

            ui.add_space(10.0);

            // Right Panel - Monitoring
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.set_min_size(Vec2::new(200.0, 400.0));
                    ui.label(RichText::new("MONITORING")
                        .size(16.0)
                        .color(GREEN)
                        .font(FontId::monospace(16.0)));
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(VuMeter::new(&state.left_level, &mut state.left_peak, "L").color(GREEN));
                        ui.add(VuMeter::new(&state.right_level, &mut state.right_peak, "R").color(YELLOW));
                        ui.add(VuMeter::new(&state.center_level, &mut state.center_peak, "C").color(PINK));
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("INPUT").size(12.0).color(GRAY_400).font(FontId::monospace(12.0)));
                        ui.add(LevelIndicator::new(state.input_level).size(Vec2::new(64.0, 16.0)));
                    });

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("OUTPUT").size(12.0).color(GRAY_400).font(FontId::monospace(12.0)));
                        ui.add(LevelIndicator::new(state.output_level).size(Vec2::new(64.0, 16.0)));
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.add(ToggleSwitch::new(&mut state.compressor, "COMP").color(YELLOW));
                        ui.add(ToggleSwitch::new(&mut state.limiter, "LIMIT").color(RED));
                    });
                });
            });
        });

        ui.add_space(15.0);

        // Bottom Panel - Advanced Matrix
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label("⚡");
                ui.label(RichText::new("ADVANCED MATRIX")
                    .size(16.0)
                    .color(YELLOW)
                    .font(FontId::monospace(16.0)));
            });
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                for (i, channel) in state.channels.iter_mut().enumerate() {
                    ui.vertical(|ui| {
                        ui.label(RichText::new(format!("CH {}", i + 1))
                            .size(10.0)
                            .color(GRAY_400)
                            .font(FontId::monospace(10.0)));
                        ui.add(Slider::new(&mut channel.value, "")
                            .vertical(true)
                            .color(match i % 4 {
                                0 => CYAN,
                                1 => PINK,
                                2 => GREEN,
                                _ => YELLOW,
                            }));
                        ui.add(ToggleSwitch::new(&mut channel.is_on, "")
                            .color(match i % 4 {
                                0 => CYAN,
                                1 => PINK,
                                2 => GREEN,
                                _ => YELLOW,
                            }));
                    });
                }
            });
        });

        ui.add_space(15.0);

        // Status Bar
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    ui.label("●");
                    ui.label(RichText::new("SYSTEM ONLINE")
                        .size(12.0)
                        .color(GREEN)
                        .font(FontId::monospace(12.0)));
                    ui.label("|");
                    ui.label(RichText::new("48kHz / 24-bit")
                        .size(12.0)
                        .color(CYAN)
                        .font(FontId::monospace(12.0)));
                    ui.label("|");
                    ui.label(RichText::new("LATENCY: 2.3ms")
                        .size(12.0)
                        .color(PINK)
                        .font(FontId::monospace(12.0)));
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("RAM: 1.2GB")
                        .size(12.0)
                        .color(GREEN)
                        .font(FontId::monospace(12.0)));
                    ui.label("|");
                    ui.label(RichText::new("CPU: 23%")
                        .size(12.0)
                        .color(YELLOW)
                        .font(FontId::monospace(12.0)));
                });
            });
        });
    });
}

#[derive(Default)]
pub struct ChannelState {
    pub value: f32,
    pub is_on: bool,
}

#[derive(Default)]
pub struct AudioControlState {
    pub master_volume: f32,
    pub master_gain: f32,
    pub bass: f32,
    pub treble: f32,
    pub low_eq: f32,
    pub mid_eq: f32,
    pub high_eq: f32,
    pub reverb: bool,
    pub echo: bool,
    pub eq: bool,
    pub compressor: bool,
    pub limiter: bool,
    pub power: bool,
    pub playing: bool,
    pub config: bool,
    pub prev: bool,
    pub mute: bool,
    pub mic: bool,
    pub next: bool,
    pub left_level: f32,
    pub right_level: f32,
    pub center_level: f32,
    pub left_peak: f32,
    pub right_peak: f32,
    pub center_peak: f32,
    pub input_level: f32,
    pub output_level: f32,
    pub channels: Vec<ChannelState>,
}

impl AudioControlState {
    pub fn new() -> Self {
        let mut state = Self {
            master_volume: 75.0,
            master_gain: 30.0,
            bass: 0.0,
            treble: 0.0,
            low_eq: 60.0,
            mid_eq: 45.0,
            high_eq: 70.0,
            power: true,
            reverb: false,
            echo: true,
            compressor: false,
            limiter: true,
            eq: false,
            left_level: 0.0,
            right_level: 0.0,
            center_level: 0.0,
            left_peak: 0.0,
            right_peak: 0.0,
            center_peak: 0.0,
            input_level: 62.5,
            output_level: 75.0,
            channels: Vec::new(),
            ..Default::default()
        };

        // Initialize 8 channels with fixed values matching React version
        state.channels = vec![
            ChannelState { value: 75.0, is_on: true },
            ChannelState { value: 60.0, is_on: false },
            ChannelState { value: 85.0, is_on: true },
            ChannelState { value: 45.0, is_on: false },
            ChannelState { value: 90.0, is_on: true },
            ChannelState { value: 30.0, is_on: true },
            ChannelState { value: 65.0, is_on: false },
            ChannelState { value: 50.0, is_on: true },
        ];

        state
    }

    pub fn update_levels(&mut self, _dt: f32) {
        // Simulate VU meter levels
        self.left_level = (self.left_level + (rand::random::<f32>() - 0.5) * 20.0)
            .clamp(0.0, 100.0);
        self.right_level = (self.right_level + (rand::random::<f32>() - 0.5) * 20.0)
            .clamp(0.0, 100.0);
        self.center_level = (self.center_level + (rand::random::<f32>() - 0.5) * 20.0)
            .clamp(0.0, 100.0);
        
        // Simulate input/output levels
        self.input_level = 62.5 + (rand::random::<f32>() - 0.5) * 25.0;
        self.output_level = 75.0 + (rand::random::<f32>() - 0.5) * 25.0;
    }
}