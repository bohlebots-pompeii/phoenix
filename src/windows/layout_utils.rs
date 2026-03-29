/// Scale a [x, y, w, h] rect by a uniform factor.
pub fn scale_rect(rect: [f32; 4], scale: f32) -> [f32; 4] {
    [
        rect[0] * scale,
        rect[1] * scale,
        rect[2] * scale,
        rect[3] * scale,
    ]
}

/// Compute a uniform scale factor relative to the 2560×1440 reference resolution.
/// Uses the smaller of the two axis scales so nothing is clipped.
pub fn compute_scale(app_width: f32, app_height: f32) -> f32 {
    let base_width = 2560.0;
    let base_height = 1440.0;
    (app_width / base_width).min(app_height / base_height)
}

/// Convert a [x, y, w, h] array to an egui::Rect.
pub fn rect_from_array(r: [f32; 4]) -> egui::Rect {
    egui::Rect::from_min_size(egui::pos2(r[0], r[1]), egui::vec2(r[2], r[3]))
}
