use egui::Context;
use crate::windows::Window;

pub struct WindowManager {
    pub windows: Vec<Box<dyn Window>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self { windows: Vec::new() }
    }

    pub fn add_window(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }

    pub fn draw(&mut self, ctx: &Context) {
        for window in self.windows.iter_mut() {
            window.draw(ctx);
        }
    }
}