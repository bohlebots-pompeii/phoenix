use std::any::Any;
use egui::{Context, Window};
use super::Window as WindowTrait;
use crate::data;

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
        let print_vec = data::robot_state::PrintVector::default();
        let line = format!("{:?}\n", print_vec.print_vector);
        const MAX_LINES: usize = 500;
        let mut lines: Vec<_> = self.log.lines().collect();
        lines.push(&line);
        if lines.len() > MAX_LINES {
            lines = lines[lines.len() - MAX_LINES..].to_vec();
        }
        self.log = lines.join("\n");
        Window::new("Console")
            .default_width(9000.0)
            .default_height(600.0)
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.log)
                            .desired_rows(20)
                            .font(egui::TextStyle::Monospace)
                            .lock_focus(true)
                            .cursor_at_end(true)
                            .desired_width(f32::INFINITY)
                    );
                });
            });
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}