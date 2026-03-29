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

    // Load PNG icon for all platforms (window icon, taskbar, launcher)
    let icon_path = "assets/bohlebots_pompeii_logo.png";
    let icon_data = match image::open(icon_path) {
        Ok(img) => {
            let img = img.to_rgba8();
            let (width, height) = img.dimensions();
            println!("Icon loaded: {} ({}x{})", icon_path, width, height);
            Some(eframe::egui::IconData {
                rgba: img.into_raw(),
                width,
                height,
            })
        },
        Err(e) => {
            eprintln!("[Warning] Could not load icon '{}': {}\nUsing default window icon", icon_path, e);
            None
        },
    };

    let mut viewport = egui::ViewportBuilder::default();
    if let Some(icon) = icon_data {
        viewport = viewport.with_icon(icon);
    }

    eframe::run_native(
        "Phoenix",
        eframe::NativeOptions {
            viewport,
            ..eframe::NativeOptions::default()
        },
        Box::new(move |_cc| Box::new(SoccerToolApp::new_with_dummy(dummy_mode))),
    ).expect("app error");
}