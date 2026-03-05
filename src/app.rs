use eframe::egui;

use crate::window_manager::WindowManager;
use crate::widgets::{console::ConsoleWindow, field::FieldWindow, graph::GraphWindow};

pub struct RoboVizApp {
    pub window_manager: WindowManager,
}

impl RoboVizApp {
    pub fn new() -> Self {
        let mut manager = WindowManager::new();

        manager.add(Box::new(FieldWindow::new()));
        manager.add(Box::new(ConsoleWindow::new()));
        manager.add(Box::new(GraphWindow::new()));

        Self {
            window_manager: manager,
        }
    }
}

impl eframe::App for RoboVizApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Windows:");

                for w in &mut self.window_manager.windows {
                    let title = w.title().to_string();
                    let open = w.open();
                    ui.checkbox(open, title);
                }
            });
        });

        self.window_manager.show(ctx);
    }
}