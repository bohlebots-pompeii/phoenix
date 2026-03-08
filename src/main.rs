mod app;
mod widgets;
mod windows;
mod data;
mod serial;

use app::SoccerToolApp;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dummy_mode = args.iter().any(|a| a == "--dummy");

    eframe::run_native(
        "Phoenix",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| Box::new(SoccerToolApp::new_with_dummy(dummy_mode))),
    ).expect("app error");
}