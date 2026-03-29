pub mod field;
pub mod console;
pub mod graph;
pub mod field_playback;
pub mod raw_serial;

pub mod raw_playback;
mod window_layouts;
pub mod serial_settings;
pub mod window_config;
pub mod layout_utils;
pub mod panel_id;

pub use field::FieldWindow;
pub use console::ConsoleWindow;
pub use graph::GraphWindow;
pub use field_playback::PlaybackWindow;
pub use raw_serial::RawSerialWindow;
pub use raw_playback::RawPlaybackWindow;
pub use window_layouts::LayoutWindow;
pub use window_config::WindowConfig;
pub use serial_settings::SerialSettingsWindow;

use egui::Context;

use std::any::Any;

pub trait Window {
    fn draw(&mut self, ctx: &Context, config: &mut WindowConfig, app_width: f32, app_height: f32);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}