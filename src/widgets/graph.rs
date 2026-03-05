use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

use crate::window_manager::AppWindow;

pub struct GraphWindow {
    open: bool,
    values: Vec<f64>,
    t: f64,
}

impl GraphWindow {

    pub fn new() -> Self {
        Self {
            open: true,
            values: Vec::new(),
            t: 0.0,
        }
    }

    pub fn push(&mut self, v: f64) {
        self.values.push(v);
    }
}

impl AppWindow for GraphWindow {

    fn title(&self) -> &str {
        "Graph"
    }

    fn open(&mut self) -> &mut bool {
        &mut self.open
    }

    fn show(&mut self, ctx: &egui::Context) {

        // demo data
        self.t += 0.05;
        self.values.push(self.t.sin());

        if self.values.len() > 200 {
            self.values.remove(0);
        }

        egui::Window::new(self.title())
            .open(&mut self.open)
            .show(ctx, |ui| {

                let points: PlotPoints = self
                    .values
                    .iter()
                    .enumerate()
                    .map(|(i, v)| [i as f64, *v])
                    .collect();

                let line = Line::new(points);

                Plot::new("demo_plot")
                    .height(200.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                    });
            });
    }
}