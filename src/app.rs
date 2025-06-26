use egui_demo_lib::DemoWindows;
use crate::audio_controls::{AudioControlState, show_audio_controls};
use crate::drag_drop_canvas::{DragDropCanvas, WidgetType, WidgetColor};
use egui::{Color32, Pos2};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // The demo windows from egui
    demo_windows: DemoWindows,
    
    // Audio controls state
    #[serde(skip)]
    audio_state: AudioControlState,
    
    // Drag and drop canvas
    #[serde(skip)]
    canvas: DragDropCanvas,
    
    // UI mode selection
    show_demo: bool,
    show_audio_controls: bool,
    show_drag_drop: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            demo_windows: DemoWindows::default(),
            audio_state: AudioControlState::new(),
            canvas: DragDropCanvas::new(),
            show_demo: false,
            show_audio_controls: false,
            show_drag_drop: true,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up dark theme to match React app
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.extreme_bg_color = Color32::BLACK;
        style.visuals.panel_fill = Color32::from_rgb(17, 24, 39); // gray-900
        style.visuals.window_fill = Color32::from_rgb(17, 24, 39);
        cc.egui_ctx.set_style(style);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // Initialize with some example widgets
        let mut app = Self::default();
        app.setup_example_widgets();
        app
    }
    
    fn setup_example_widgets(&mut self) {
        // Add some example widgets to showcase the system
        self.canvas.add_widget(
            WidgetType::Panel {
                title: "MASTER CONTROL".to_string(),
                color: WidgetColor::Cyan,
                width: 220.0,
                height: 180.0,
                collapsed: false,
                contained_widgets: Vec::new(),
                minimize_to_settings_icon: false,
            },
            Pos2::new(50.0, 50.0),
        );
        
        self.canvas.add_widget(
            WidgetType::Knob {
                value: 75.0,
                min: 0.0,
                max: 100.0,
                label: "VOLUME".to_string(),
                color: WidgetColor::Cyan,
            },
            Pos2::new(80.0, 100.0),
        );
        
        self.canvas.add_widget(
            WidgetType::Knob {
                value: 30.0,
                min: 0.0,
                max: 100.0,
                label: "GAIN".to_string(),
                color: WidgetColor::Pink,
            },
            Pos2::new(180.0, 100.0),
        );
        
        self.canvas.add_widget(
            WidgetType::Panel {
                title: "EQ & EFFECTS".to_string(),
                color: WidgetColor::Pink,
                width: 220.0,
                height: 200.0,
                collapsed: false,
                contained_widgets: Vec::new(),
                minimize_to_settings_icon: false,
            },
            Pos2::new(300.0, 50.0),
        );
        
        self.canvas.add_widget(
            WidgetType::HorizontalSlider {
                value: 60.0,
                min: 0.0,
                max: 100.0,
                label: "LOW".to_string(),
                color: WidgetColor::Green,
            },
            Pos2::new(320.0, 120.0),
        );
        
        self.canvas.add_widget(
            WidgetType::HorizontalSlider {
                value: 45.0,
                min: 0.0,
                max: 100.0,
                label: "MID".to_string(),
                color: WidgetColor::Yellow,
            },
            Pos2::new(320.0, 150.0),
        );
        
        self.canvas.add_widget(
            WidgetType::HorizontalSlider {
                value: 70.0,
                min: 0.0,
                max: 100.0,
                label: "HIGH".to_string(),
                color: WidgetColor::Pink,
            },
            Pos2::new(320.0, 180.0),
        );
        
        self.canvas.add_widget(
            WidgetType::Panel {
                title: "MONITORING".to_string(),
                color: WidgetColor::Green,
                width: 180.0,
                height: 200.0,
                collapsed: false,
                contained_widgets: Vec::new(),
                minimize_to_settings_icon: false,
            },
            Pos2::new(550.0, 50.0),
        );
        
        self.canvas.add_widget(
            WidgetType::VuMeter {
                level: 75.0,
                peak_level: 80.0,
                label: "L".to_string(),
                color: WidgetColor::Green,
            },
            Pos2::new(580.0, 100.0),
        );
        
        self.canvas.add_widget(
            WidgetType::VuMeter {
                level: 60.0,
                peak_level: 65.0,
                label: "R".to_string(),
                color: WidgetColor::Yellow,
            },
            Pos2::new(620.0, 100.0),
        );
        
        self.canvas.add_widget(
            WidgetType::VuMeter {
                level: 85.0,
                peak_level: 90.0,
                label: "C".to_string(),
                color: WidgetColor::Pink,
            },
            Pos2::new(660.0, 100.0),
        );
        
        // Add some toggle switches
        self.canvas.add_widget(
            WidgetType::ToggleSwitch {
                on: false,
                label: "REVERB".to_string(),
                color: WidgetColor::Cyan,
                glow: true,
            },
            Pos2::new(320.0, 220.0),
        );
        
        self.canvas.add_widget(
            WidgetType::ToggleSwitch {
                on: true,
                label: "ECHO".to_string(),
                color: WidgetColor::Pink,
                glow: true,
            },
            Pos2::new(400.0, 220.0),
        );
        
        self.canvas.add_widget(
            WidgetType::ToggleSwitch {
                on: false,
                label: "EQ".to_string(),
                color: WidgetColor::Green,
                glow: true,
            },
            Pos2::new(480.0, 220.0),
        );
        
        // Add some push buttons
        self.canvas.add_widget(
            WidgetType::PushButton {
                active: true,
                icon: "⚡".to_string(),
                label: "POWER".to_string(),
                color: WidgetColor::Green,
                size: 48.0,
            },
            Pos2::new(80.0, 250.0),
        );
        
        self.canvas.add_widget(
            WidgetType::PushButton {
                active: false,
                icon: "▶".to_string(),
                label: "PLAY".to_string(),
                color: WidgetColor::Cyan,
                size: 56.0,
            },
            Pos2::new(150.0, 250.0),
        );
        
        // Add vertical sliders for mixer channels
        for i in 0..8 {
            let values = [75.0, 60.0, 85.0, 45.0, 90.0, 30.0, 65.0, 50.0];
            let colors = [WidgetColor::Cyan, WidgetColor::Pink, WidgetColor::Green, WidgetColor::Yellow];
            
            self.canvas.add_widget(
                WidgetType::VerticalSlider {
                    value: values[i],
                    min: 0.0,
                    max: 100.0,
                    label: format!("CH{}", i + 1),
                    color: colors[i % 4],
                },
                Pos2::new(50.0 + i as f32 * 50.0, 350.0),
            );
        }
        
        // Add level indicators
        self.canvas.add_widget(
            WidgetType::LevelIndicator {
                level: 62.5,
                segments: 8,
                label: "INPUT".to_string(),
            },
            Pos2::new(580.0, 280.0),
        );
        
        // Add title
        self.canvas.add_widget(
            WidgetType::TextLabel {
                text: "AUDIO CONTROL MATRIX".to_string(),
                size: 24.0,
                color: WidgetColor::Cyan,
            },
            Pos2::new(300.0, 20.0),
        );
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update audio state
        self.audio_state.update_levels(ctx.input(|i| i.unstable_dt));

        // Top menu bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_demo, "Demo Windows");
                    ui.checkbox(&mut self.show_audio_controls, "Audio Controls");
                    ui.checkbox(&mut self.show_drag_drop, "Drag & Drop Canvas");
                });
                
                ui.separator();
                
                ui.label("Audio Control Matrix - Drag & Drop Interface");
            });
        });

        // Show widget palette on the left
        if self.show_drag_drop {
            egui::SidePanel::left("widget_palette")
                .default_width(220.0)
                .show(ctx, |ui| {
                    self.canvas.show_widget_palette(ui);
                });
        }

        // Main canvas area
        if self.show_drag_drop {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Drag & Drop Audio Control Canvas");
                ui.separator();
                
                // Instructions
                ui.horizontal(|ui| {
                    ui.label("🎛️ Drag widgets from the palette or move existing widgets around the canvas");
                    ui.separator();
                    ui.label("🎨 Each widget matches the exact styling from the React app");
                });
                
                ui.add_space(10.0);
                
                // Render the canvas
                self.canvas.render(ui);
            });
        }

        // Show audio controls window if enabled
        if self.show_audio_controls {
            egui::Window::new("Audio Controls")
                .default_size([800.0, 600.0])
                .show(ctx, |ui| {
                    show_audio_controls(ui, &mut self.audio_state);
                });
        }

        // Show demo windows if enabled
        if self.show_demo {
            self.demo_windows.ui(ctx);
        }
    }
}