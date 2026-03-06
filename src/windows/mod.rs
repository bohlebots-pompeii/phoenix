pub mod field;
pub mod console;
pub mod graph;

pub use field::FieldWindow;
pub use console::ConsoleWindow;
pub use graph::GraphWindow;

use egui::Context;

use std::any::Any;

pub trait Window {
    fn draw(&mut self, ctx: &Context);
    fn as_any(&mut self) -> &mut dyn Any;
}