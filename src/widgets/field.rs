use eframe::egui::{self, Color32, Pos2, Stroke};

use crate::window_manager::AppWindow;

pub struct FieldWindow {
    open: bool,
}

impl FieldWindow {

    pub fn new() -> Self {
        Self { open: true }
    }
}

impl AppWindow for FieldWindow {

    fn title(&self) -> &str {
        "Soccer Field"
    }

    fn open(&mut self) -> &mut bool {
        &mut self.open
    }

    fn show(&mut self, ctx: &egui::Context) {

        egui::Window::new(self.title())
            .open(&mut self.open)
            .show(ctx, |ui| {

                let (rect, _) =
                    ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());

                let painter = ui.painter_at(rect);

                let field_color = Color32::DARK_GREEN;

                painter.rect_filled(rect, 0.0, field_color);

                let stroke = Stroke::new(2.0, Color32::WHITE);

                painter.rect_stroke(rect, 0.0, stroke);

                let center = rect.center();

                painter.circle_stroke(center, 50.0, stroke);

                painter.line_segment(
                    [Pos2::new(center.x, rect.top()), Pos2::new(center.x, rect.bottom())],
                    stroke,
                );

                // Example robot
                let robot = Pos2::new(center.x - 100.0, center.y);

                painter.circle_filled(robot, 15.0, Color32::BLUE);

                // Ball
                let ball = Pos2::new(center.x + 100.0, center.y);

                painter.circle_filled(ball, 10.0, Color32::BLUE);
            });
    }
}