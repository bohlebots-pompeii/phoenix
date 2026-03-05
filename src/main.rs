mod app;
mod widgets;
mod windows;

use app::SoccerToolApp;
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RoboCup Visualization Tool",
        options,
        Box::new(|_cc| Box::new(SoccerToolApp::new())),
    );
}