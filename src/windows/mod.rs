pub mod field;
pub mod console;
pub mod graph;
pub mod field_playback;
pub mod raw_serial;

pub mod raw_playback;

pub use field::FieldWindow;
pub use console::ConsoleWindow;
pub use graph::GraphWindow;
pub use field_playback::PlaybackWindow;
pub use raw_serial::RawSerialWindow;
pub use raw_playback::RawPlaybackWindow;

use egui::Context;

use std::any::Any;

pub trait Window {
    fn draw(&mut self, ctx: &Context);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}