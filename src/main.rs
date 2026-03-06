mod app;
mod widgets;
mod windows;
mod data;
mod serial;


use app::SoccerToolApp;

fn main() {
    eframe::run_native(
        "RoboCup Visualizer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(SoccerToolApp::new(None))),
    ).expect("app error");
}