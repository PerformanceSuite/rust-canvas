//! Widget rendering functions for the drag-and-drop canvas system
//! 
//! This module contains all the rendering implementations for the various widget types
//! supported by the canvas. Each widget has its own specialized rendering function
//! that handles its visual representation.

use egui::{Color32, Pos2, Rect, Vec2, FontId, Align2, Stroke};
use std::f32::consts::PI;

use crate::canvas::constants::*;
use super::types::{WidgetColor, IconType, CanvasEdge};

pub fn render_knob(painter: &egui::Painter, rect: Rect, value: &mut f32, min: f32, max: f32, label: &str, color: WidgetColor) {
    let knob_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.top() + 37.0),
        Vec2::splat(64.0),
    );
    let center = knob_rect.center();
    let radius = 32.0;
    let normalized = (*value - min) / (max - min);
    let angle = normalized * 270.0 * PI / 180.0 - 135.0 * PI / 180.0;

    // Draw outer ring
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
        
        painter.line_segment([inner_pos, outer_pos], Stroke::new(2.0, color.to_color32()));
    }

    // Draw inner circle
    painter.circle_filled(center, radius - 12.0, GRAY_900);

    // Draw indicator line
    let indicator_length = radius - 16.0;
    let indicator_pos = center + Vec2::new(
        angle.cos() * indicator_length,
        angle.sin() * indicator_length,
    );
    painter.line_segment([center, indicator_pos], Stroke::new(4.0, color.to_color32()));

    // Draw center dot
    painter.circle_filled(center, 4.0, color.to_color32());

    // Draw label
    painter.text(
        Pos2::new(center.x, rect.bottom() - 30.0),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(10.0),
        GRAY_400,
    );

    // Draw value
    painter.text(
        Pos2::new(center.x, rect.bottom() - 15.0),
        Align2::CENTER_CENTER,
        format!("{:.1}", value),
        FontId::monospace(10.0),
        color.to_color32(),
    );
}

pub fn render_toggle_switch(painter: &egui::Painter, rect: Rect, on: &mut bool, label: &str, color: WidgetColor, glow: bool) {
    let switch_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.top() + 17.0),
        Vec2::new(48.0, 24.0),
    );
    let radius = 12.0;
    
    let bg_color = if *on { color.to_color32() } else { GRAY_700 };

    // Draw glow effect if on
    if *on && glow {
        let glow_color = Color32::from_rgba_unmultiplied(
            color.to_color32().r(),
            color.to_color32().g(),
            color.to_color32().b(),
            30
        );
        for i in 1..=3 {
            let glow_rect = switch_rect.expand(i as f32 * 2.0);
            painter.rect_filled(glow_rect, radius + i as f32 * 2.0, glow_color);
        }
    }

    // Draw switch background
    painter.rect_filled(switch_rect, radius, bg_color);

    // Draw switch handle
    let handle_radius = 10.0;
    let handle_x = if *on {
        switch_rect.right() - radius
    } else {
        switch_rect.left() + radius
    };

    painter.circle_filled(
        Pos2::new(handle_x, switch_rect.center().y),
        handle_radius,
        WHITE,
    );

    // Draw shadow for active state
    if *on {
        // Glow for handle
        if glow {
            let handle_glow = Color32::from_rgba_unmultiplied(255, 255, 255, 20);
            painter.circle_filled(
                Pos2::new(handle_x, switch_rect.center().y),
                handle_radius + 3.0,
                handle_glow,
            );
        }
    }

    // Draw label
    if !label.is_empty() {
        painter.text(
            Pos2::new(rect.center().x, rect.bottom() - 10.0),
            Align2::CENTER_CENTER,
            label,
            FontId::monospace(10.0),
            GRAY_400,
        );
    }
}

pub fn render_push_button(painter: &egui::Painter, rect: Rect, active: &mut bool, icon: &str, label: &str, color: WidgetColor, size: f32) {
    let button_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.top() + size / 2.0 + 5.0),
        Vec2::splat(size),
    );

    let (fill_color, _stroke_color) = if *active {
        (Color32::from_rgba_unmultiplied(color.to_color32().r(), color.to_color32().g(), color.to_color32().b(), 60), color.to_color32())
    } else {
        (GRAY_800, GRAY_600)
    };

    // Draw button background
    painter.rect_filled(button_rect, 12.0, fill_color);
    
    // No borders for push buttons

    // Draw icon
    let icon_color = if *active { color.to_color32() } else { GRAY_400 };
    painter.text(
        button_rect.center(),
        Align2::CENTER_CENTER,
        icon,
        FontId::monospace(20.0),
        icon_color,
    );

    // Draw label
    painter.text(
        Pos2::new(rect.center().x, rect.bottom() - 10.0),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(8.0),
        GRAY_400,
    );
}

pub fn render_vu_meter(painter: &egui::Painter, rect: Rect, level: f32, peak_level: &mut f32, label: &str, color: WidgetColor) {
    let meter_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.top() + 69.0),
        Vec2::new(16.0, 128.0),
    );

    // Update peak level
    if level > *peak_level {
        *peak_level = level;
    } else {
        *peak_level = (*peak_level - 0.5).max(0.0);
    }

    // Draw background
    painter.rect_filled(meter_rect, 4.0, GRAY_800);
    
    // No borders for VU meters

    // Draw level segments
    let segments = 20;
    let segment_height = 128.0 / segments as f32;
    let current_segments = ((level / 100.0) * segments as f32) as usize;

    for i in 0..segments {
        let segment_rect = Rect::from_min_size(
            Pos2::new(
                meter_rect.left() + 2.0,
                meter_rect.bottom() - (i + 1) as f32 * segment_height,
            ),
            Vec2::new(12.0, segment_height - 1.0),
        );

        if i < current_segments {
            let segment_color = if i >= 18 {
                RED
            } else if i >= 14 {
                YELLOW
            } else {
                color.to_color32()
            };
            painter.rect_filled(segment_rect, 1.0, segment_color);
        }
    }

    // Draw peak indicator
    if *peak_level > 0.0 {
        let peak_y = meter_rect.bottom() - (*peak_level / 100.0) * 128.0;
        painter.line_segment(
            [
                Pos2::new(meter_rect.left() + 2.0, peak_y),
                Pos2::new(meter_rect.right() - 2.0, peak_y),
            ],
            Stroke::new(2.0, WHITE),
        );
    }

    // Draw label
    painter.text(
        Pos2::new(rect.center().x, rect.bottom() - 10.0),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(10.0),
        GRAY_400,
    );
}

pub fn render_horizontal_slider(painter: &egui::Painter, rect: Rect, value: &mut f32, min: f32, max: f32, label: &str, color: WidgetColor) {
    let normalized = (*value - min) / (max - min);

    // Draw label
    painter.text(
        Pos2::new(rect.left() + 25.0, rect.center().y),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(10.0),
        GRAY_400,
    );

    let slider_rect = Rect::from_center_size(
        Pos2::new(rect.center().x + 10.0, rect.center().y),
        Vec2::new(96.0, 8.0),
    );

    // Draw background
    painter.rect_filled(slider_rect, 4.0, GRAY_700);

    // Draw filled portion
    let fill_width = slider_rect.width() * normalized;
    let fill_rect = Rect::from_min_size(
        slider_rect.min,
        Vec2::new(fill_width, slider_rect.height()),
    );
    painter.rect_filled(fill_rect, 4.0, color.to_color32());

    // Draw value
    painter.text(
        Pos2::new(rect.right() - 15.0, rect.center().y),
        Align2::CENTER_CENTER,
        format!("{:.0}", value),
        FontId::monospace(10.0),
        color.to_color32(),
    );
}

pub fn render_vertical_slider(painter: &egui::Painter, rect: Rect, value: &mut f32, min: f32, max: f32, _label: &str, color: WidgetColor) {
    let normalized = (*value - min) / (max - min);

    let slider_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.center().y - 10.0),
        Vec2::new(8.0, 96.0),
    );

    // Draw background
    painter.rect_filled(slider_rect, 4.0, GRAY_700);

    // Draw filled portion
    let fill_height = slider_rect.height() * normalized;
    let fill_rect = Rect::from_min_size(
        Pos2::new(slider_rect.left(), slider_rect.bottom() - fill_height),
        Vec2::new(slider_rect.width(), fill_height),
    );
    painter.rect_filled(fill_rect, 4.0, color.to_color32());

    // Draw value
    painter.text(
        Pos2::new(rect.center().x, rect.bottom() - 15.0),
        Align2::CENTER_CENTER,
        format!("{:.0}", value),
        FontId::monospace(8.0),
        color.to_color32(),
    );
}

pub fn render_level_indicator(painter: &egui::Painter, rect: Rect, level: f32, segments: usize, label: &str) {
    let colors = vec![GREEN, GREEN, GREEN, GREEN, GREEN, YELLOW, YELLOW, RED];
    let indicator_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.center().y - 5.0),
        Vec2::new(rect.width() - 20.0, 20.0)
    );
    let segment_width = (indicator_rect.width() - (segments - 1) as f32) / segments as f32;
    let active_segments = ((level / 100.0) * segments as f32) as usize;

    for i in 0..segments {
        let x = indicator_rect.left() + i as f32 * (segment_width + 1.0);
        let segment_rect = Rect::from_min_size(
            Pos2::new(x, indicator_rect.top()),
            Vec2::new(segment_width, indicator_rect.height()),
        );

        let color = if i < active_segments {
            colors.get(i).copied().unwrap_or(GREEN)
        } else {
            GRAY_600
        };

        painter.rect_filled(segment_rect, 1.0, color);
        
        // Add glow effect for active segments
        if i < active_segments {
            let glow_color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 20);
            painter.rect_filled(segment_rect.expand(2.0), 1.0, glow_color);
        }
    }
    
    // Draw label
    if !label.is_empty() {
        painter.text(
            Pos2::new(rect.left() + 10.0, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            FontId::monospace(10.0),
            GRAY_400,
        );
    }
}

pub fn render_text_label(painter: &egui::Painter, rect: Rect, text: &str, size: f32, color: WidgetColor) {
    painter.text(
        rect.center(),
        Align2::CENTER_CENTER,
        text,
        FontId::monospace(size),
        color.to_color32(),
    );
}

pub fn render_panel(painter: &egui::Painter, rect: Rect, title: &str, color: WidgetColor, collapsed: bool, contained_widgets: &Vec<usize>, minimize_to_settings_icon: bool) {
    if collapsed && minimize_to_settings_icon {
        // Show only settings icon when collapsed AND minimize_to_settings_icon is enabled
        // No background, just the icon at top-left corner
        painter.text(
            Pos2::new(rect.left() + 20.0, rect.top() + 20.0),
            Align2::CENTER_CENTER,
            "⚙",
            FontId::monospace(20.0),
            color.to_color32(),
        );
    } else {
        // Normal panel rendering
        // Draw panel background (matching React's gray-900)
        painter.rect_filled(rect, 16.0, GRAY_900);
        
        // Draw gradient background
        let gradient_color = Color32::from_rgba_unmultiplied(
            color.to_color32().r(),
            color.to_color32().g(),
            color.to_color32().b(),
            10
        );
        painter.rect_filled(rect.shrink(1.0), 16.0, gradient_color);
        
        // Draw title with collapse indicator
        let title_text = if collapsed {
            format!("▶ {}", title)
        } else {
            format!("▼ {}", title)
        };
        
        painter.text(
            Pos2::new(rect.left() + 10.0, rect.top() + 20.0),
            Align2::LEFT_CENTER,
            &title_text,
            FontId::monospace(14.0),
            color.to_color32(),
        );
        
        // Show widget count for panels
        if !contained_widgets.is_empty() {
            painter.text(
                Pos2::new(rect.right() - 60.0, rect.top() + 20.0),
                Align2::CENTER_CENTER,
                &format!("({})", contained_widgets.len()),
                FontId::monospace(10.0),
                GRAY_400,
            );
        }
        
        // Only draw resize handle if not collapsed
        if !collapsed {
            let handle_size = 12.0;
            let handle_rect = Rect::from_min_size(
                Pos2::new(rect.max.x - handle_size, rect.max.y - handle_size),
                Vec2::splat(handle_size),
            );
            
            // Draw resize handle lines
            for i in 0..3 {
                let offset = i as f32 * 3.0;
                painter.line_segment(
                    [
                        Pos2::new(handle_rect.min.x + offset, handle_rect.max.y - 2.0),
                        Pos2::new(handle_rect.max.x - 2.0, handle_rect.min.y + offset),
                    ],
                    Stroke::new(1.0, GRAY_600),
                );
            }
        }
    }
}

pub fn render_status_bar(painter: &egui::Painter, rect: Rect, cpu: f32, ram: f32, latency: f32, online: bool) {
    // Background
    painter.rect_filled(rect, 8.0, GRAY_900);
    
    // No borders for status bar
    
    // Online indicator
    let indicator_pos = Pos2::new(rect.left() + 15.0, rect.center().y);
    let indicator_color = if online { GREEN } else { RED };
    painter.circle_filled(indicator_pos, 4.0, indicator_color);
    
    // Pulsing effect for online
    if online {
        let pulse_color = Color32::from_rgba_unmultiplied(
            indicator_color.r(),
            indicator_color.g(),
            indicator_color.b(),
            50
        );
        painter.circle_filled(indicator_pos, 6.0, pulse_color);
    }
    
    // Status text
    painter.text(
        Pos2::new(rect.left() + 30.0, rect.center().y),
        Align2::LEFT_CENTER,
        if online { "SYSTEM ONLINE" } else { "SYSTEM OFFLINE" },
        FontId::monospace(10.0),
        indicator_color,
    );
    
    // System stats
    painter.text(
        Pos2::new(rect.center().x - 50.0, rect.center().y),
        Align2::CENTER_CENTER,
        "48kHz / 24-bit",
        FontId::monospace(10.0),
        CYAN,
    );
    
    painter.text(
        Pos2::new(rect.center().x + 50.0, rect.center().y),
        Align2::CENTER_CENTER,
        format!("LATENCY: {:.1}ms", latency),
        FontId::monospace(10.0),
        PINK,
    );
    
    painter.text(
        Pos2::new(rect.right() - 120.0, rect.center().y),
        Align2::CENTER_CENTER,
        format!("CPU: {:.0}%", cpu),
        FontId::monospace(10.0),
        YELLOW,
    );
    
    painter.text(
        Pos2::new(rect.right() - 50.0, rect.center().y),
        Align2::CENTER_CENTER,
        format!("RAM: {:.1}GB", ram),
        FontId::monospace(10.0),
        GREEN,
    );
    
    // Draw resize handle in bottom-right corner
    let handle_size = 12.0;
    let handle_rect = Rect::from_min_size(
        Pos2::new(rect.max.x - handle_size, rect.max.y - handle_size),
        Vec2::splat(handle_size),
    );
    
    // Draw resize handle lines
    for i in 0..3 {
        let offset = i as f32 * 3.0;
        painter.line_segment(
            [
                Pos2::new(handle_rect.min.x + offset, handle_rect.max.y - 2.0),
                Pos2::new(handle_rect.max.x - 2.0, handle_rect.min.y + offset),
            ],
            Stroke::new(1.0, GRAY_600),
        );
    }
}

pub fn render_icon_button(painter: &egui::Painter, rect: Rect, icon: IconType, label: &str, active: &mut bool, color: WidgetColor, size: f32) {
    let button_rect = Rect::from_center_size(
        Pos2::new(rect.center().x, rect.top() + size / 2.0 + 5.0),
        Vec2::splat(size),
    );

    // All icon buttons have transparent background
    let icon_color = if *active {
        color.to_color32()
    } else {
        GRAY_400
    };
    
    // No background or border for any icon buttons
    let fill_color = Color32::TRANSPARENT;
    let stroke_color = Color32::TRANSPARENT;

    // Draw button background (only if not transparent)
    if fill_color != Color32::TRANSPARENT {
        painter.rect_filled(button_rect, size / 2.0, fill_color);
    }
    
    // Draw border (only if not transparent)
    if stroke_color != Color32::TRANSPARENT {
        let border_width = 2.0;
        for i in 0..4 {
            let border_edge = match i {
                0 => Rect::from_min_size(button_rect.min, Vec2::new(border_width, button_rect.height())),
                1 => Rect::from_min_size(Pos2::new(button_rect.max.x - border_width, button_rect.min.y), Vec2::new(border_width, button_rect.height())),
                2 => Rect::from_min_size(button_rect.min, Vec2::new(button_rect.width(), border_width)),
                _ => Rect::from_min_size(Pos2::new(button_rect.min.x, button_rect.max.y - border_width), Vec2::new(button_rect.width(), border_width)),
            };
            painter.rect_filled(border_edge, 0.0, stroke_color);
        }
    }

    // Draw icon based on type
    let icon_text = match icon {
        IconType::Power => "⏻",
        IconType::Play => "▶",
        IconType::Pause => "⏸",
        IconType::SkipBack => "⏮",
        IconType::SkipForward => "⏭",
        IconType::Volume => "🔊",
        IconType::Mic => "🎤",
        IconType::Settings => "⚙",
        IconType::Mute => "🔇",
        IconType::Zap => "⚡",
    };
    
    painter.text(
        button_rect.center(),
        Align2::CENTER_CENTER,
        icon_text,
        FontId::monospace(size / 3.0),
        icon_color,
    );

    // Draw label
    painter.text(
        Pos2::new(rect.center().x, rect.bottom() - 10.0),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(8.0),
        GRAY_400,
    );
}

pub fn render_settings_panel(painter: &egui::Painter, rect: Rect, title: &str, color: WidgetColor, minimized: bool, edge: CanvasEdge, _contained_widgets: &Vec<usize>) {
    if minimized {
        // Render minimized state - just a settings icon
        let icon_color = color.to_color32();
        
        // Draw semi-transparent background for the icon
        painter.rect_filled(rect, 8.0, Color32::from_rgba_unmultiplied(0, 0, 0, 120));
        
        // Draw settings icon
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            "⚙",
            FontId::monospace(24.0),
            icon_color,
        );
    } else {
        // Render expanded state - full panel
        // Draw panel background (solid black)
        painter.rect_filled(rect, 16.0, BLACK);
        
        // Draw border around the panel
        let border_stroke = Stroke::new(2.0, color.to_color32());
        // Top border
        painter.line_segment([rect.left_top(), rect.right_top()], border_stroke);
        // Right border  
        painter.line_segment([rect.right_top(), rect.right_bottom()], border_stroke);
        // Bottom border
        painter.line_segment([rect.right_bottom(), rect.left_bottom()], border_stroke);
        // Left border
        painter.line_segment([rect.left_bottom(), rect.left_top()], border_stroke);

        // Draw title bar with minimize button
        let title_height = 30.0;
        let _title_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), title_height));
        
        // Draw title
        painter.text(
            Pos2::new(rect.left() + 10.0, rect.top() + 15.0),
            Align2::LEFT_CENTER,
            title,
            FontId::monospace(12.0),
            color.to_color32(),
        );
        
        // Draw minimize button (X) in top-right
        let close_button_size = 20.0;
        let close_button_pos = Pos2::new(
            rect.right() - close_button_size - 5.0,
            rect.top() + 5.0
        );
        let close_rect = Rect::from_min_size(close_button_pos, Vec2::splat(close_button_size));
        
        painter.rect_filled(close_rect, 4.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30));
        painter.text(
            close_rect.center(),
            Align2::CENTER_CENTER,
            "−",
            FontId::monospace(12.0),
            WHITE,
        );
        
        // Draw edge indicator based on snapped edge
        let indicator_color = match edge {
            CanvasEdge::Left => CYAN,
            CanvasEdge::Right => PINK,
            CanvasEdge::Top => GREEN,
            CanvasEdge::Bottom => YELLOW,
            CanvasEdge::None => GRAY_600,
        };
        
        // Draw edge indicator line
        match edge {
            CanvasEdge::Left => {
                painter.line_segment(
                    [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.left(), rect.bottom())],
                    Stroke::new(3.0, indicator_color),
                );
            }
            CanvasEdge::Right => {
                painter.line_segment(
                    [Pos2::new(rect.right(), rect.top()), Pos2::new(rect.right(), rect.bottom())],
                    Stroke::new(3.0, indicator_color),
                );
            }
            CanvasEdge::Top => {
                painter.line_segment(
                    [Pos2::new(rect.left(), rect.top()), Pos2::new(rect.right(), rect.top())],
                    Stroke::new(3.0, indicator_color),
                );
            }
            CanvasEdge::Bottom => {
                painter.line_segment(
                    [Pos2::new(rect.left(), rect.bottom()), Pos2::new(rect.right(), rect.bottom())],
                    Stroke::new(3.0, indicator_color),
                );
            }
            CanvasEdge::None => {} // No indicator for unsnapped panels
        }
        
        // Draw resize handle based on edge
        match edge {
            CanvasEdge::Left => {
                // Right edge resize handle for width
                let handle_size = 8.0;
                let handle_rect = Rect::from_center_size(
                    Pos2::new(rect.right(), rect.center().y),
                    Vec2::new(handle_size, 60.0),
                );
                painter.rect_filled(handle_rect, 2.0, GRAY_600);
                
                // Draw resize indicator lines
                for i in 0..3 {
                    let y_offset = (i as f32 - 1.0) * 8.0;
                    painter.line_segment(
                        [
                            Pos2::new(handle_rect.center().x - 2.0, handle_rect.center().y + y_offset),
                            Pos2::new(handle_rect.center().x + 2.0, handle_rect.center().y + y_offset),
                        ],
                        Stroke::new(1.0, WHITE),
                    );
                }
            }
            CanvasEdge::Right => {
                // Left edge resize handle for width
                let handle_size = 8.0;
                let handle_rect = Rect::from_center_size(
                    Pos2::new(rect.left(), rect.center().y),
                    Vec2::new(handle_size, 60.0),
                );
                painter.rect_filled(handle_rect, 2.0, GRAY_600);
                
                // Draw resize indicator lines
                for i in 0..3 {
                    let y_offset = (i as f32 - 1.0) * 8.0;
                    painter.line_segment(
                        [
                            Pos2::new(handle_rect.center().x - 2.0, handle_rect.center().y + y_offset),
                            Pos2::new(handle_rect.center().x + 2.0, handle_rect.center().y + y_offset),
                        ],
                        Stroke::new(1.0, WHITE),
                    );
                }
            }
            CanvasEdge::Top => {
                // Bottom edge resize handle for height
                let handle_size = 8.0;
                let handle_rect = Rect::from_center_size(
                    Pos2::new(rect.center().x, rect.bottom()),
                    Vec2::new(60.0, handle_size),
                );
                painter.rect_filled(handle_rect, 2.0, GRAY_600);
                
                // Draw resize indicator lines
                for i in 0..3 {
                    let x_offset = (i as f32 - 1.0) * 8.0;
                    painter.line_segment(
                        [
                            Pos2::new(handle_rect.center().x + x_offset, handle_rect.center().y - 2.0),
                            Pos2::new(handle_rect.center().x + x_offset, handle_rect.center().y + 2.0),
                        ],
                        Stroke::new(1.0, WHITE),
                    );
                }
            }
            CanvasEdge::Bottom => {
                // Top edge resize handle for height
                let handle_size = 8.0;
                let handle_rect = Rect::from_center_size(
                    Pos2::new(rect.center().x, rect.top()),
                    Vec2::new(60.0, handle_size),
                );
                painter.rect_filled(handle_rect, 2.0, GRAY_600);
                
                // Draw resize indicator lines
                for i in 0..3 {
                    let x_offset = (i as f32 - 1.0) * 8.0;
                    painter.line_segment(
                        [
                            Pos2::new(handle_rect.center().x + x_offset, handle_rect.center().y - 2.0),
                            Pos2::new(handle_rect.center().x + x_offset, handle_rect.center().y + 2.0),
                        ],
                        Stroke::new(1.0, WHITE),
                    );
                }
            }
            CanvasEdge::None => {} // No resize handle for unsnapped panels
        }
        
        // No content text - clean canvas area
    }
}