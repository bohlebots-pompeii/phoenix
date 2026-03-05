use egui::{Context, Window};
use super::Window as WindowTrait;

pub struct ConsoleWindow {
    log: String,
}

impl ConsoleWindow {
    pub fn new() -> Self {
        Self {
            log: String::from("Console ready.\n"),
        }
    }
}

impl WindowTrait for ConsoleWindow {
    fn draw(&mut self, ctx: &Context) {
        Window::new("Console")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(&self.log);
            });
    }
}