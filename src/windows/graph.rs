use std::any::Any;
use egui::{Context, Window};
use super::{Window as WindowTrait, WindowConfig};

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
    fn draw(&mut self, ctx: &Context, config: &mut WindowConfig, app_width: f32, app_height: f32) {
        let rect = config.graph_rect(app_width, app_height);
        Window::new(format!("Graph [{}]", config.selected_layout_idx()))
            .default_width(rect.width())
            .default_height(rect.height())
            .default_pos([rect.left(), rect.top()])
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("Graph placeholder 4 add plots here.");
            });
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}