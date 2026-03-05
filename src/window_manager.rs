use eframe::egui;

pub trait AppWindow {
    fn title(&self) -> &str;
    fn open(&mut self) -> &mut bool;
    fn show(&mut self, ctx: &egui::Context);
}

pub struct WindowManager {
    pub windows: Vec<Box<dyn AppWindow>>,
}

impl WindowManager {

    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn add(&mut self, window: Box<dyn AppWindow>) {
        self.windows.push(window);
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        for w in &mut self.windows {
            if *w.open() {
                w.show(ctx);
            }
        }
    }
}