use std::any::Any;
use egui::{Context, Window};
use super::Window as WindowTrait;

pub struct GraphWindow {
    values: Vec<f32>,
}

impl GraphWindow {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push_value(&mut self, v: f32) {
        self.values.push(v);
        if self.values.len() > 1000 {
            self.values.remove(0);
        }
    }
}

impl WindowTrait for GraphWindow {
    fn draw(&mut self, ctx: &Context) {
        Window::new("Graph")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Graph placeholder — add plots here.");
            });
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}