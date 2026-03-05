use eframe::egui;

use crate::window_manager::AppWindow;

pub struct ConsoleWindow {
    open: bool,
    logs: Vec<String>,
}

impl ConsoleWindow {

    pub fn new() -> Self {
        Self {
            open: true,
            logs: vec![
                "Console started".into(),
                "Robot connected".into(),
                "Ball detected".into(),
            ],
        }
    }

    pub fn log(&mut self, msg: String) {
        self.logs.push(msg);
    }
}

impl AppWindow for ConsoleWindow {

    fn title(&self) -> &str {
        "Console"
    }

    fn open(&mut self) -> &mut bool {
        &mut self.open
    }

    fn show(&mut self, ctx: &egui::Context) {

        egui::Window::new(self.title())
            .open(&mut self.open)
            .show(ctx, |ui| {

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for line in &self.logs {
                        ui.label(line);
                    }
                });
            });
    }
}