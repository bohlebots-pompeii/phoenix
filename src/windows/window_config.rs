use app::SoccerToolApp;
use crate::app;

pub struct WindowConfig {
    pub console: [f32; 4],        // first two are x, y positions last two are dimensions
    pub field: [f32; 4],
    pub field_playback: [f32; 4],
    pub graph: [f32; 4],
    pub raw_playback: [f32; 4],
    pub raw_serial: [f32; 4],
    pub window_layouts: [f32; 4],
    pub selected_layout_idx: usize, // NEW: currently selected layout index
}

impl WindowConfig {
    pub fn selected_layout_idx(&self) -> usize {
        self.selected_layout_idx
    }

    pub fn window_layouts_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.window_layouts[0], self.window_layouts[1]),
            egui::vec2(self.window_layouts[2], self.window_layouts[3])
        )
    }
    pub fn console_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.console[0], self.console[1]),
            egui::vec2(self.console[2], self.console[3])
        )
    }
    pub fn field_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.field[0], self.field[1]),
            egui::vec2(self.field[2], self.field[3])
        )
    }
    pub fn graph_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.graph[0], self.graph[1]),
            egui::vec2(self.graph[2], self.graph[3])
        )
    }
    pub fn raw_serial_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.raw_serial[0], self.raw_serial[1]),
            egui::vec2(self.raw_serial[2], self.raw_serial[3])
        )
    }
    pub fn raw_playback_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.raw_playback[0], self.raw_playback[1]),
            egui::vec2(self.raw_playback[2], self.raw_playback[3])
        )
    }
    pub fn field_playback_rect(&self, _app_width: f32, _app_height: f32) -> egui::Rect {
        egui::Rect::from_min_size(
            egui::pos2(self.field_playback[0], self.field_playback[1]),
            egui::vec2(self.field_playback[2], self.field_playback[3])
        )
    }
    pub fn scale_rect(rect: [f32; 4], scale: f32) -> [f32; 4] {
        [rect[0] * scale, rect[1] * scale, rect[2] * scale, rect[3] * scale]
    }
    pub fn compute_scale(app_width: f32, app_height: f32) -> f32 {
        let base_width = 2560.0;
        let base_height = 1440.0;
        let scale_w = app_width / base_width;
        let scale_h = app_height / base_height;
        scale_w.min(scale_h)
    }
    pub fn with_scale(scale: f32) -> Self{
        Self {
            console: [2000.0 * scale, 0.0 * scale, 700.0 * scale, 1329.0 * scale],
            field: [0.0 * scale, 0.0 * scale, 900.0 * scale, 900.0 * scale],
            field_playback: [0.0, 0.0, 0.0, 0.0],
            graph: [0.0, 0.0, 0.0, 0.0],
            raw_playback: [0.0, 0.0, 0.0, 0.0],
            raw_serial: [0.0, 0.0, 0.0, 0.0],
            window_layouts: [0.0, 0.0, 0.0, 0.0],
            selected_layout_idx: 0,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            console: [0.0; 4],
            field: [0.0; 4],
            field_playback: [0.0; 4],
            graph: [0.0; 4],
            raw_playback: [0.0; 4],
            raw_serial: [0.0; 4],
            window_layouts: [0.0; 4],
            selected_layout_idx: 0,
        }
    }
}
