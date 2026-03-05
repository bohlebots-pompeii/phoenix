use eframe::egui;
use crate::widgets::window_manager::WindowManager;
use crate::windows::{FieldWindow, ConsoleWindow, GraphWindow};
use crate::widgets::window_manager;


pub struct SoccerToolApp {
    manager: WindowManager,
}

impl SoccerToolApp {
    pub fn new() -> Self {
        let mut manager = WindowManager::new();

        manager.add_window(Box::new(FieldWindow::new()));
        manager.add_window(Box::new(ConsoleWindow::new()));
        manager.add_window(Box::new(GraphWindow::new()));

        Self { manager }
    }
}

impl eframe::App for SoccerToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.manager.draw(ctx);
    }
}