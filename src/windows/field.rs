use egui::{Color32, Context, Painter, Pos2, Stroke, Window, Shape};
use super::Window as WindowTrait;

pub struct FieldWindow;

impl FieldWindow {
    pub fn new() -> Self {
        Self
    }
}

impl WindowTrait for FieldWindow {
    fn draw(&mut self, ctx: &Context) {
        Window::new("Soccer Field")
            .resizable(true)
            .show(ctx, |ui| {
                let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
                let painter = ui.painter();

                // Draw field background (optional)
                painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 20));

                // Convert field coordinates to screen coordinates
                draw_field(painter, rect);
            });
    }
}

fn draw_arc(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    stroke: Stroke,
) {
    let segments = 32; // More segments for smoother arc
    let mut points = Vec::with_capacity(segments + 1);
    let sweep = end_angle - start_angle;
    for i in 0..=segments {
        let angle = start_angle + sweep * (i as f32) / (segments as f32);
        points.push(Pos2::new(
            center.x + radius * angle.cos(),
            center.y + radius * angle.sin(),
        ));
    }
    painter.add(egui::Shape::line(points, stroke));
}

fn draw_field(painter: &Painter, rect: egui::Rect) {
    // Field dimensions: 105m x 68m (used as ratio only)
    let field_m_width = 105.0;
    let field_m_height = 68.0;

    // Fit with margin inside UI rect
    let margin = 12.0; // px
    let avail_width = rect.width() - 2.0 * margin;
    let avail_height = rect.height() - 2.0 * margin;
    let scale = f32::min(avail_width / field_m_width, avail_height / field_m_height);

    let field_px_width = field_m_width * scale;
    let field_px_height = field_m_height * scale;

    let field_left = rect.center().x - 0.5 * field_px_width;
    let field_top = rect.center().y - 0.5 * field_px_height;

    // Helper closure to map field meters to screen px
    let to_px = |mx: f32, my: f32| -> Pos2 {
        Pos2::new(field_left + mx * scale, field_top + my * scale)
    };

    // Colors and strokes
    let line = Stroke::new(2.0, Color32::WHITE);
    let goal_stroke = Stroke::new(3.0, Color32::WHITE);
    let thin = Stroke::new(1.0, Color32::GRAY);

    // Field boundary
    painter.rect_stroke(
        egui::Rect::from_min_size(
            to_px(0.0, 0.0),
            egui::vec2(field_px_width, field_px_height)
        ), 0.0, line
    );

    // Halfway line
    painter.line_segment(
        [to_px(0.0, field_m_height / 2.0), to_px(field_m_width, field_m_height / 2.0)],
        line
    );

    // Center circle and mark
    let center = to_px(field_m_width / 2.0, field_m_height / 2.0);
    let center_circle_radius = 9.15 * scale; // 9.15m
    painter.circle_stroke(center, center_circle_radius, line);
    painter.circle_filled(center, 0.2 * scale, Color32::WHITE);

    // Penalty areas and goal areas
    let pa_w = 40.32; // Penalty area width (box): 40.32m
    let pa_h = 16.5;  // Penalty area height (box): 16.5m
    let ga_w = 18.32; // Goal area width: 18.32m
    let ga_h = 5.5;   // Goal area height: 5.5m
    let penalty_spot_dist = 11.0; // meters from goal line
    let penalty_arc_r = 9.15; // 9.15m penalty arc radius
    for &side in &[0.0, field_m_width] {
        let dir = if side < 1.0 { 1.0 } else { -1.0 };
        // Penalty box rect
        let pa_left = side - dir * pa_h;
        painter.rect_stroke(
            egui::Rect::from_min_max(
                to_px(pa_left, (field_m_height - pa_w)/2.0),
                to_px(side, (field_m_height + pa_w)/2.0)
            ), 0.0, line
        );
        // Goal area rect
        let ga_left = side - dir * ga_h;
        painter.rect_stroke(
            egui::Rect::from_min_max(
                to_px(ga_left, (field_m_height - ga_w)/2.0),
                to_px(side, (field_m_height + ga_w)/2.0)
            ), 0.0, line
        );
        // Penalty spot
        let pen_spot = to_px(side - dir * penalty_spot_dist, field_m_height / 2.0);
        painter.circle_filled(pen_spot, 0.18 * scale, Color32::WHITE);
        // Penalty arc ('D') -- only the outward part
        let arc_c = to_px(side - dir * penalty_spot_dist, field_m_height / 2.0);
        let arc_ang0 = if dir > 0.0 { -0.29 * std::f32::consts::PI } else { 1.29 * std::f32::consts::PI };
        let arc_ang1 = if dir > 0.0 { 0.29 * std::f32::consts::PI } else { 0.71 * std::f32::consts::PI };
        draw_arc(painter, arc_c, penalty_arc_r * scale, arc_ang0, arc_ang1, line);

        // Simple goal rendering
        let goal_post1 = to_px(side, (field_m_height/2.0) - 3.66);
        let goal_post2 = to_px(side, (field_m_height/2.0) + 3.66);
        painter.line_segment([goal_post1, goal_post2], goal_stroke);
    }
    // Corner arcs
    for &xy in &[(0.0,0.0),(0.0,field_m_height),(field_m_width,0.0),(field_m_width,field_m_height)] {
        let center = to_px(xy.0, xy.1);
        let start_ang = if xy.0 == 0.0 && xy.1 == 0.0 {
            0.5 * std::f32::consts::PI
        } else if xy.0 == 0.0 {
            std::f32::consts::PI
        } else if xy.1 == 0.0 {
            0.0
        } else {
            1.5 * std::f32::consts::PI
        };
        draw_arc(painter, center, scale, start_ang, start_ang + 0.5 * std::f32::consts::PI, thin);
    }
}
