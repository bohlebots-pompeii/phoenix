pub mod field;
pub mod console;
pub mod graph;
pub mod field_playback;

pub use field::FieldWindow;
pub use console::ConsoleWindow;
pub use graph::GraphWindow;
pub use field_playback::PlaybackWindow;

use egui::Context;

use std::any::Any;

pub trait Window {
    fn draw(&mut self, ctx: &Context);
    fn as_any(&mut self) -> &mut dyn Any;
}