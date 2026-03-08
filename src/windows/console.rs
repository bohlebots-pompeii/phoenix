use std::any::Any;
use egui::{Context, Window};
use super::Window as WindowTrait;
use crate::data;
use crate::windows::WindowConfig;

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
    fn draw(&mut self, ctx: &Context, config: &mut WindowConfig, app_width: f32, app_height: f32) {
        let print_vec = data::robot_state::PrintVector::default();
        let line = format!("{:?}\n", print_vec.print_vector);
        const MAX_LINES: usize = 500;
        let mut lines: Vec<_> = self.log.lines().collect();
        lines.push(&line);
        if lines.len() > MAX_LINES {
            lines = lines[lines.len() - MAX_LINES..].to_vec();
        }
        self.log = lines.join("\n");
        let rect = config.console_rect(app_width, app_height);
        Window::new(format!("Console [{}]", config.selected_layout_idx()))
            .default_width(rect.width())
            .default_height(rect.height())
            .default_pos([rect.left(), rect.top()])
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

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}