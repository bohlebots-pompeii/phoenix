use eframe::egui;
use crate::widgets::window_manager::WindowManager;
use crate::windows::{FieldWindow, ConsoleWindow, GraphWindow};
use std::sync::mpsc::{Receiver, channel};

pub struct SoccerToolApp {
    manager: WindowManager,

    rx: Option<Receiver<String>>,
    serial_enabled: bool,
}

impl SoccerToolApp {
    pub fn new(rx: Option<Receiver<String>>) -> Self {
        let serial_enabled = rx.is_some();

        let mut manager = WindowManager::new();

        manager.add_window(Box::new(FieldWindow::new()));
        manager.add_window(Box::new(ConsoleWindow::new()));
        manager.add_window(Box::new(GraphWindow::new()));

        Self {
            manager,
            rx,
            serial_enabled,
        }
    }
}

impl eframe::App for SoccerToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if let Some(rx) = &self.rx {
            while let Ok(line) = rx.try_recv() {
                println!("Received: {}", line);
                // later: parse line -> RobotState
            }
        }

        self.manager.draw(ctx);
    }
}