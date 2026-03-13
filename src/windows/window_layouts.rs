use crate::windows::window_config::WindowConfig;

// --- Layout definitions (reference size) ---

pub struct WindowLayout {
    pub name: &'static str,
    pub console: [f32; 4],
    pub field: [f32; 4],
    pub field_playback: [f32; 4],
    pub graph: [f32; 4],
    pub raw_playback: [f32; 4],
    pub raw_serial: [f32; 4],
    pub window_layouts: [f32; 4],
    pub serial_layouts: [f32; 4],
    // Add more window rects here if needed
}

pub const LAYOUT_DEFAULT: WindowLayout = WindowLayout {
    name: "Default",
    console: [2095.0, 0.0, 900.0, 95.0],
    field: [0.0, 664.5, 12000.0, 900.0],
    field_playback: [100.0, 100.0, 9000.0, 900.0],
    graph: [1600.0, 0.0, 700.0, 600.0],
    raw_playback: [0.0, 900.0, 1300.0, 250.0],
    raw_serial: [0.0, 1150.0, 1300.0, 250.0],
    window_layouts: [80.0, 80.0, 320.0, 360.0],
    serial_layouts: [100.0, 100.0, 900.0, 900.0],
};

pub const LAYOUT_NORMAL: WindowLayout = WindowLayout {
    name: "Normal",
    console: [2150.0, 0.0, 600.0, 1560.0],
    field: [0.0, 0.0, 550.0, 780.0],
    field_playback: [570.0, 0.0, 550.0, 780.0],
    graph: [1200.0, 900.0, 500.0, 400.0],
    raw_playback: [570.0, 827.0, 550.0, 733.0],
    raw_serial: [0.0, 827.0, 550.0, 733.0],
    window_layouts: [600.0, 60.0, 300.0, 340.0],
    serial_layouts: [1140.0, 0.0, 900.0, 900.0],
};

pub const ALL_LAYOUTS: &[&WindowLayout] = &[
    &LAYOUT_DEFAULT,
    &LAYOUT_NORMAL,
];

// --- LayoutWindow for interacting/selecting layouts ---

pub struct LayoutWindow {
    // No state needed; selection is in WindowConfig
}

impl LayoutWindow {
    pub fn new() -> Self {
        Self {}
    }
    // This takes `app_width` and `app_height` at draw time
    pub fn draw(&mut self, ctx: &egui::Context, window_config: &mut WindowConfig, app_width: f32, app_height: f32) {
        let scale = WindowConfig::compute_scale(app_width, app_height);
        // Set a sensible default for layout window if not set
if window_config.window_layouts[2] < 40.0 || window_config.window_layouts[3] < 40.0 {
    window_config.window_layouts = [80.0, 80.0, 320.0, 360.0];
}
let layout_rect = window_config.window_layouts_rect(app_width, app_height);
        egui::Window::new("Window Layout")
            .default_width(layout_rect.width())
            .default_height(layout_rect.height())
            .default_pos([layout_rect.left(), layout_rect.top()])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for (i, layout) in ALL_LAYOUTS.iter().enumerate() {
                        if ui.button(layout.name).clicked() {
                            window_config.selected_layout_idx = i;
                            let scale = WindowConfig::compute_scale(app_width, app_height);
                            window_config.console = WindowConfig::scale_rect(layout.console, scale);
                            window_config.field = WindowConfig::scale_rect(layout.field, scale);
                            window_config.field_playback = WindowConfig::scale_rect(layout.field_playback, scale);
                            window_config.graph = WindowConfig::scale_rect(layout.graph, scale);
                            window_config.raw_playback = WindowConfig::scale_rect(layout.raw_playback, scale);
                             window_config.raw_serial = WindowConfig::scale_rect(layout.raw_serial, scale);
                             window_config.serial_settings = WindowConfig::scale_rect(layout.serial_layouts, scale);
                             window_config.window_layouts = WindowConfig::scale_rect(layout.window_layouts, scale);
                             // --- Force egui to close windows so default_* is respected next frame ---
                             ui.ctx().memory_mut(|mem| mem.reset_areas());
                        }
                    }
                });
                let layout = ALL_LAYOUTS[window_config.selected_layout_idx];
                let scaled_console = WindowConfig::scale_rect(layout.console, scale);
                let scaled_field = WindowConfig::scale_rect(layout.field, scale);
                let scaled_field_playback = WindowConfig::scale_rect(layout.field_playback, scale);
                let scaled_graph = WindowConfig::scale_rect(layout.graph, scale);
                let scaled_raw_playback = WindowConfig::scale_rect(layout.raw_playback, scale);
                let scaled_raw_serial = WindowConfig::scale_rect(layout.raw_serial, scale);
                let scaled_window_layouts = WindowConfig::scale_rect(layout.window_layouts, scale);
                ui.label(format!("Console (scaled): {:?}", scaled_console));
                ui.label(format!("Field (scaled): {:?}", scaled_field));
                ui.label(format!("Field Playback (scaled): {:?}", scaled_field_playback));
                ui.label(format!("Graph (scaled): {:?}", scaled_graph));
                ui.label(format!("Raw Playback (scaled): {:?}", scaled_raw_playback));
                ui.label(format!("Raw Serial (scaled): {:?}", scaled_raw_serial));
                ui.label(format!("Window Layout (scaled): {:?}", scaled_window_layouts));
                // Actual UI rendering logic using scaled rects for all windows
            });
    }
}

// trait impl (adapts to your system)
use std::any::Any;
use egui::Context;
use crate::windows::Window as WindowTrait;

impl WindowTrait for LayoutWindow {
    fn draw(&mut self, ctx: &Context, config: &mut WindowConfig, app_width: f32, app_height: f32) {
        self.draw(ctx, config, app_width, app_height);
    }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
