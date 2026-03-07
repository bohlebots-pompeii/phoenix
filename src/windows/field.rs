use egui::{Color32, Context, Painter, Pos2, Stroke, Window, Shape};
use super::Window as WindowTrait;

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct SerializableTimedState {
    pub timestamp: f64, // unix seconds
    pub state: RobotState,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VisualizationMode {
    Bot0,
    Bot1,
    BothBots,
}

pub struct FieldWindow {
    robot_state_0: Option<RobotState>,
    robot_state_1: Option<RobotState>,
    slider_value: f32,
    logged_states: VecDeque<SerializableTimedState>,
    is_logging: bool,
    // Multi-bot visualization UI state
    mode: VisualizationMode,
    authoritative_bot: u8, // 0 or 1, valid iff mode == BothBots
}

impl FieldWindow {
    pub fn new(initial_state: RobotState) -> Self {
        // Place initial state into the correct bot slot based on bot id (if -1, default to 0)
        let mut robot_state_0 = None;
        let mut robot_state_1 = None;
        match initial_state.esp_now_bot_id {
            0 | -1 => robot_state_0 = Some(initial_state),
            1 => robot_state_1 = Some(initial_state),
            _ => {},
        }
        Self {
            robot_state_0,
            robot_state_1,
            slider_value: 5.0,
            logged_states: VecDeque::new(),
            is_logging: false,
            mode: VisualizationMode::Bot0,
            authoritative_bot: 0,
        }
    }
}

impl FieldWindow {
    /// Call this ONLY when a new serial state arrives
    pub fn on_new_state(&mut self, state: RobotState) {
        // Only log if logging enabled
        if self.is_logging {
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            let window = self.slider_value as f64;

            if window > 0.0 {
                self.logged_states.push_back(SerializableTimedState {
                    timestamp: now,
                    state: state.clone(),
                });
                // Remove anything older than window seconds
                while let Some(front) = self.logged_states.front() {
                    if front.timestamp < now - window {
                        self.logged_states.pop_front();
                    } else {
                        break;
                    }
                }
            } else {
                // Clear all if slider is set to 0
                self.logged_states.clear();
            }
            // Always overwrite log file (even if empty!)
            let _ = std::fs::write(
                "rolling_log.json",
                serde_json::to_string_pretty(&self.logged_states).unwrap()
            );
        }
        // Store in the correct bot cache field
        match state.esp_now_bot_id {
            0 | -1 => self.robot_state_0 = Some(state),
            1 => self.robot_state_1 = Some(state),
            _ => {},
        }
    }
}

impl WindowTrait for FieldWindow {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn draw(&mut self, ctx: &Context) {
        Window::new("Soccer Field")
            .resizable(true)
            .show(ctx, |ui| {
                ui.add(egui::Slider::new(&mut self.slider_value, 0.0..=600.0).text("Recording time (s)"));

                // --- Rolling Log Progress Bar ---
                let log_fill = if self.logged_states.len() > 1 && self.slider_value > 0.0 {
                    let first = self.logged_states.front().unwrap().timestamp;
                    let last = self.logged_states.back().unwrap().timestamp;
                    let span = (last - first).max(0.0);
                    (span / (self.slider_value as f64)).min(1.0)
                } else { 0.0 };
                ui.add(egui::ProgressBar::new(log_fill as f32).text(format!("Rolling log filled: {:.0}%", 100.0 * log_fill)));

                // Export button (must be first, before painter!)
                if ui.button("Export State to JSON").clicked() {
                    // Use matching bot for mode (in BothBots, use authoritative_bot)
                    let export_bot = match self.mode {
                        VisualizationMode::Bot0 => self.robot_state_0.as_ref(),
                        VisualizationMode::Bot1 => self.robot_state_1.as_ref(),
                        VisualizationMode::BothBots => match self.authoritative_bot {
                            0 => self.robot_state_0.as_ref(),
                            1 => self.robot_state_1.as_ref(),
                            _ => None,
                        }
                    };
                    if let Some(bot) = export_bot {
                        let _ = bot.export_json("robot_state_export.json");
                    }
                }

                // --- Replay Export Button ---
                if ui.button("Save Replay").clicked() {
                    use std::fs;
                    use std::path::Path;
                    use chrono::Local;
                    // Ensure replays directory exists
                    let _ = fs::create_dir_all("replays");
                    // Format timestamp for file name
                    let time_str = Local::now().format("%Y-%m-%d_%H-%M-%S");
                    let replay_path = format!("replays/{}.json", time_str);
                    // Copy rolling_log.json to new file (best-effort)
                    let _ = fs::copy("rolling_log.json", &replay_path);
                }

                ui.horizontal(|ui| {
                    if ui.button("Start Logging").clicked() {
                        self.is_logging = true;
                    }
                    if ui.button("Stop Logging").clicked() {
                        self.is_logging = false;
                    }
                    let txt = if self.is_logging { "Logging: ON" } else { "Logging: OFF" };
                    ui.label(txt);
                });
                ui.separator();

                // --- Visualization Mode Selection ---
                ui.horizontal(|ui| {
                    ui.label("Visualization mode:");
                    if ui.radio_value(&mut self.mode, VisualizationMode::Bot0, "Bot 0").clicked() {
                        // When switching, default authoritative_bot to this bot if applicable
                        self.authoritative_bot = 0;
                    }
                    if ui.radio_value(&mut self.mode, VisualizationMode::Bot1, "Bot 1").clicked() {
                        self.authoritative_bot = 1;
                    }
                    if ui.radio_value(&mut self.mode, VisualizationMode::BothBots, "Both Bots").clicked() {
                        // Default to bot 0 being authoritative,
                        // or leave previous authoritative as-is
                    }
                });

                // If mode is BothBots, let user choose authoritative bot for e.g. ball
                if self.mode == VisualizationMode::BothBots {
                    ui.horizontal(|ui| {
                        ui.label("Authoritative for shared features:");
                        ui.radio_value(&mut self.authoritative_bot, 0, "Bot 0");
                        ui.radio_value(&mut self.authoritative_bot, 1, "Bot 1");
                    });
                }

                // Now allocate after button/any ui widgets
                let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
                let painter = ui.painter();
                // Draw field background (optional)
                painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 20));
                // Convert field coordinates to screen coordinates
                match self.mode {
                    VisualizationMode::Bot0 => {
                        if let Some(bot) = &self.robot_state_0 {
                            draw_field(painter, rect, bot);
                        }
                    },
                    VisualizationMode::Bot1 => {
                        if let Some(bot) = &self.robot_state_1 {
                            draw_field(painter, rect, bot);
                        }
                    },
                    VisualizationMode::BothBots => {
                        // Draw both bots; for ambiguous features, authoritative_bot picks source (for future logic)
                        if let Some(bot0) = &self.robot_state_0 {
                            draw_field(painter, rect, bot0);
                        }
                        if let Some(bot1) = &self.robot_state_1 {
                            draw_field(painter, rect, bot1);
                        }
                        // (To be further refined: differentiate bots visually, share ball from authoritative)
                    },
                }
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

use crate::data::robot_state::RobotState;

pub(crate) fn draw_field(painter: &Painter, rect: egui::Rect, robot: &RobotState) {
    // Field dimensions in cm (example: FIFA std = 105m x 68m => 10500 x 6800 cm)
    let field_cm_width = 158.0;   // set your real value
    let field_cm_height = 219.0;  // set your real value

    // Fit with margin inside UI rect
    let margin = 30.0; // px
    let avail_width = rect.width() - 2.0 * margin;
    let avail_height = rect.height() - 2.0 * margin;
    let scale = f32::min(avail_width / field_cm_width, avail_height / field_cm_height);

    let field_px_width = field_cm_width * scale;
    let field_px_height = field_cm_height * scale;
    let field_center = rect.center();

    // New: map from center-origin field coordinates (in cm, where (0,0) = field center)
    let to_px = |mx: f32, my: f32| -> Pos2 {
        Pos2::new(
            field_center.x + mx * scale, // X+ is right as usual
            field_center.y - my * scale  // Y+ is upward on canvas
        )
    };


    // Colors and strokes
    let line = Stroke::new(2.0, Color32::WHITE);
    let center_circle_line = Stroke::new(1.0, Color32::BLACK);
    let yellow_goal_line = Stroke::new(2.0, Color32::YELLOW);
    let blue_goal_line = Stroke::new(2.0, Color32::BLUE);
    let walls_line = Stroke::new(5.0, Color32::BLACK);
    let goal_stroke = Stroke::new(3.0, Color32::WHITE);
    let thin = Stroke::new(1.0, Color32::GRAY);

    // Ground
    painter.rect_filled(
        egui::Rect::from_min_max(
            to_px(-field_cm_width/2.0 - 12.0, field_cm_height/2.0 + 12.0),
            to_px(field_cm_width/2.0 + 12.0, -field_cm_height/2.0 - 12.0)
        ),
        0.0,
        Color32::DARK_GREEN
    );

    // Walls
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-field_cm_width/2.0 - 12.0, field_cm_height/2.0 + 12.0),
            to_px(field_cm_width/2.0 + 12.0, -field_cm_height/2.0 - 12.0)
        ),
        0.0,
        walls_line
    );

    // Field boundary (corners: (-w/2, -h/2), (w/2, h/2))
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-field_cm_width/2.0, field_cm_height/2.0),
            to_px(field_cm_width/2.0, -field_cm_height/2.0)
        ), 0.0, line
    );

    // Center circle and mark
    let center = to_px(0.0, 0.0);
    let center_circle_radius = 30.0 * scale;
    painter.circle_stroke(center, center_circle_radius, center_circle_line);
    painter.circle_filled(center, 1.0 * scale, Color32::BLACK);

    // Penalty areas and goal areas (center-origin coordinates)
    let pa_w = 80.0; // penalty area width
    let pa_h = 24.0; // penalty area height
    let ga_w = 60.0;
    let ga_h = 7.4;
    let y_top = field_cm_height/2.0;

    // Penalty area 1 (north)
    let pa1_y1 = y_top;
    let pa1_y2 = y_top - pa_h;
    let (pa1y_min, pa1y_max) = if pa1_y1 < pa1_y2 { (pa1_y1, pa1_y2) } else { (pa1_y2, pa1_y1) };
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-pa_w/2.0, pa1y_max),
            to_px(pa_w/2.0, pa1y_min)
        ), 0.0, line
    );

    // Goal area 1 (north)
    let ga1_y1 = y_top;
    let ga1_y2 = y_top + ga_h;
    let (ga1y_min, ga1y_max) = if ga1_y1 < ga1_y2 { (ga1_y1, ga1_y2) } else { (ga1_y2, ga1_y1) };
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-ga_w/2.0, ga1y_max),
            to_px(ga_w/2.0, ga1y_min)
        ), 0.0, yellow_goal_line
    );


    // Penalty area 2 (fix: make it extend inwards from -y_top)
    let pa2_y1 = -y_top;
    let pa2_y2 = -y_top + pa_h;
    let (pa2y_min, pa2y_max) = if pa2_y1 < pa2_y2 { (pa2_y1, pa2_y2) } else { (pa2_y2, pa2_y1) };
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-pa_w/2.0, pa2y_max),
            to_px(pa_w/2.0, pa2y_min)
        ), 0.0, line
    );

    // Goal area 2 (fix: inside -y_top)
    let ga2_y1 = -y_top;
    let ga2_y2 = -y_top - ga_h;
    let (ga2y_min, ga2y_max) = if ga2_y1 < ga2_y2 { (ga2_y1, ga2_y2) } else { (ga2_y2, ga2_y1) };
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-ga_w/2.0, ga2y_max),
            to_px(ga_w/2.0, ga2y_min)
        ), 0.0, blue_goal_line
    );

    // Draw goals just outside the field line
    let goal_w = 24.0;
    let goal_depth = 7.0;
    // Top goal (north)
    let g_top_y = y_top + goal_depth;
    painter.rect_stroke(
        egui::Rect::from_min_max(
            to_px(-goal_w/2.0, y_top),
            to_px(goal_w/2.0, g_top_y)
        ), 0.0, goal_stroke
    );

    // Neutral point 1
    let pen_spot = to_px(40.0, y_top - 45.0);
    painter.circle_filled(pen_spot, 1.0 * scale, Color32::BLACK);

    // Neutral point 2
    let pen_spot = to_px(-40.0, y_top - 45.0);
    painter.circle_filled(pen_spot, 1.0 * scale, Color32::BLACK);

    // Neutral point 3
    let pen_spot = to_px(40.0, -y_top + 45.0);
    painter.circle_filled(pen_spot, 1.0 * scale, Color32::BLACK);

    // Neutral point 4
    let pen_spot = to_px(-40.0, -y_top + 45.0);
    painter.circle_filled(pen_spot, 1.0 * scale, Color32::BLACK);

    // ===== Draw Robot =====
    let robot_pos = to_px(robot.vision.global_x, robot.vision.global_y);
    let robot_radius = 5.0 * scale;
    painter.circle_filled(robot_pos, robot_radius, Color32::WHITE);
    // Heading arrow
    let heading_rad = robot.vision.heading.to_radians();
    let heading_len = 12.0 * scale;
    let tip = to_px(
        robot.vision.global_x + heading_len * heading_rad.cos(),
        robot.vision.global_y + heading_len * heading_rad.sin(),
    );
    painter.line_segment([robot_pos, tip], Stroke::new(2.0, Color32::LIGHT_BLUE));

    // ===== Draw Ball (if exists) =====
    if robot.vision.ball_exists {
        let theta = robot.vision.heading.to_radians() + robot.vision.ball_rot.to_radians();
        let b_dist = robot.vision.ball_dist;
        let bx = robot.vision.global_x + b_dist * theta.cos();
        let by = robot.vision.global_y + b_dist * theta.sin();
        let ball_pos = to_px(bx, by);
        painter.circle_filled(ball_pos, 3.0 * scale, Color32::RED);
    }

    // ===== Draw Peer Robot =====
    let peer_pos = to_px(robot.peer.global_x, robot.peer.global_y);
    painter.circle_stroke(peer_pos, robot_radius, Stroke::new(2.0, Color32::YELLOW));
    // Peer heading
    let peer_heading_rad = robot.peer.heading.to_radians();
    let peer_tip = to_px(
        robot.peer.global_x + heading_len * peer_heading_rad.cos(),
        robot.peer.global_y + heading_len * peer_heading_rad.sin(),
    );
    painter.line_segment([peer_pos, peer_tip], Stroke::new(2.0, Color32::YELLOW));

    // ===== Draw Detections/Objects =====
    for det in 0..robot.vision.num_detections as usize {
        let label = robot.vision.object_label[det];
        let rot_deg = robot.vision.object_rot_deg[det];
        let dist = robot.vision.object_dist_cm[det];
        let theta = robot.vision.heading.to_radians() + rot_deg.to_radians();
        let ox = robot.vision.global_x + dist * theta.cos();
        let oy = robot.vision.global_y + dist * theta.sin();
        let obj_pos = to_px(ox, oy);
        let obj_col = match label {
            1 => Color32::LIGHT_GREEN,
            2 => Color32::LIGHT_YELLOW,
            _ => Color32::GRAY,
        };
        painter.circle_filled(obj_pos, 2.0 * scale, obj_col);
    }
}

