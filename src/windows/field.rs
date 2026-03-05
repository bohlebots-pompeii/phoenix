use egui::{Context, Window};
use super::Window as WindowTrait;

pub struct FieldWindow;

impl FieldWindow {
    pub fn new() -> Self {
        Self
    }
}

impl WindowTrait for FieldWindow {
    fn draw(&mut self, ctx: &Context) {
        Window::new("Soccer Field")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Field rendering goes here.");
                ui.add(egui::Separator::default());
                ui.label("Draw vectors, ball, robots, etc.");
            });
    }
}