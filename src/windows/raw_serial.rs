use std::any::Any;
use egui::{Context, Window};
use crate::windows::Window as WindowTrait;
use crate::data::robot_state::{RobotState, VisionData, SensorData, MotionData, GameState, PeerRobot, PrintVector};

pub struct RawSerialWindow {
    latest_bot0: Option<RobotState>,      // most recent state for bot 0
    latest_bot1: Option<RobotState>,      // most recent state for bot 1
}

impl RawSerialWindow {
    pub fn new() -> Self {
        Self {
            latest_bot0: None,
            latest_bot1: None,
        }
    }

    pub fn on_new_serial_line(&mut self, line: &str) {
        if let Ok(state) = serde_json::from_str::<RobotState>(line) {
            match state.esp_now_bot_id {
                0 => self.latest_bot0 = Some(state),
                1 => self.latest_bot1 = Some(state),
                _ => {},
            }
        }
    }
}

impl WindowTrait for RawSerialWindow {
    fn draw(&mut self, ctx: &Context) {
        Window::new("Raw Serial Data")
            .default_width(300.0)
            .default_height(800.0)
            .default_pos([1040.0, 120.0])
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    egui::Grid::new("raw-serial-table").striped(true)
                        .min_col_width(110.0)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.heading("Field");
                            ui.heading("Bot 0");
                            ui.heading("Bot 1");
                            ui.end_row();
                            let fields0 = self.latest_bot0.as_ref().map(robot_state_fields);
                            let fields1 = self.latest_bot1.as_ref().map(robot_state_fields);
                            let mut labels = std::collections::BTreeSet::new();
                            if let Some(ref f0) = fields0 {
                                for (k, _) in f0 {
                                    labels.insert(k.clone());
                                }
                            }
                            if let Some(ref f1) = fields1 {
                                for (k, _) in f1 {
                                    labels.insert(k.clone());
                                }
                            }
                            let labels: Vec<_> = labels.into_iter().collect();
                            for label in labels {
                                let val0 = fields0.as_ref()
                                    .and_then(|fs| fs.iter().find(|(k,_)| k==&label))
                                    .map(|(_,v)| v.as_str())
                                    .unwrap_or("-");
                                let val1 = fields1.as_ref()
                                    .and_then(|fs| fs.iter().find(|(k,_)| k==&label))
                                    .map(|(_,v)| v.as_str())
                                    .unwrap_or("-");
                                ui.label(&label);
                                ui.monospace(val0);
                                ui.monospace(val1);
                                ui.end_row();
                            }
                            if self.latest_bot0.is_none() && self.latest_bot1.is_none() {
                                ui.label("No packet data for either bot yet.");
                                ui.end_row();
                            }
                        });
                });
            });
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub(crate) fn robot_state_fields(state: &RobotState) -> Vec<(String, String)> {
    let mut fields = Vec::new();
    // VisionData
    fields.push(("[vision]".to_string(), "".to_string()));
    let v = &state.vision;
    fields.push(("  heading".to_string(), format!("{:.2}", v.heading)));
    fields.push(("  global_x".to_string(), format!("{:.2}", v.global_x)));
    fields.push(("  global_y".to_string(), format!("{:.2}", v.global_y)));
    fields.push(("  ball_rot".to_string(), format!("{:.2}", v.ball_rot)));
    fields.push(("  ball_dist".to_string(), format!("{:.2}", v.ball_dist)));
    fields.push(("  ball_exists".to_string(), v.ball_exists.to_string()));
    fields.push(("  target_goal_rot".to_string(), format!("{:.2}", v.target_goal_rot)));
    fields.push(("  target_goal_dist".to_string(), format!("{:.2}", v.target_goal_dist)));
    fields.push(("  own_goal_rot".to_string(), format!("{:.2}", v.own_goal_rot)));
    fields.push(("  own_goal_dist".to_string(), format!("{:.2}", v.own_goal_dist)));
    fields.push(("  away_from_own_goal_angle".to_string(), format!("{:.2}", v.away_from_own_goal_angle)));
    fields.push(("  target_goal_label".to_string(), format!("{}", v.target_goal_label)));
    fields.push(("  own_goal_label".to_string(), format!("{}", v.own_goal_label)));
    fields.push(("  num_detections".to_string(), format!("{}", v.num_detections)));
    for (i, &val) in v.object_label.iter().enumerate() {
        fields.push((format!("  object_label[{}]", i), format!("{}", val)));
    }
    for (i, &val) in v.object_rot_deg.iter().enumerate() {
        fields.push((format!("  object_rot_deg[{}]", i), format!("{:.2}", val)));
    }
    for (i, &val) in v.object_dist_cm.iter().enumerate() {
        fields.push((format!("  object_dist_cm[{}]", i), format!("{:.2}", val)));
    }
    fields.push(("  cm5_running".to_string(), v.cm5_running.to_string()));
    // SensorData
    fields.push(("[sensors]".to_string(), "".to_string()));
    let s = &state.sensors;
    fields.push(("  line_rot".to_string(), format!("{}", s.line_rot)));
    fields.push(("  progress".to_string(), format!("{}", s.progress)));
    fields.push(("  line_seen".to_string(), s.line_seen.to_string()));
    fields.push(("  has_ball".to_string(), s.has_ball.to_string()));
    fields.push(("  ball_light_gate".to_string(), s.ball_light_gate.to_string()));
    fields.push(("  ena".to_string(), s.ena.to_string()));
    // MotionData
    fields.push(("[motion]".to_string(), "".to_string()));
    let m = &state.motion;
    fields.push(("  velocity_x".to_string(), format!("{:.2}", m.velocity_x)));
    fields.push(("  velocity_y".to_string(), format!("{:.2}", m.velocity_y)));
    fields.push(("  velocity_magnitude".to_string(), format!("{:.2}", m.velocity_magnitude)));
    fields.push(("  rotation_delta".to_string(), format!("{:.2}", m.rotation_delta)));
    fields.push(("  middle_point_vec_x".to_string(), format!("{:.2}", m.middle_point_vec_x)));
    fields.push(("  middle_point_vec_y".to_string(), format!("{:.2}", m.middle_point_vec_y)));
    fields.push(("  middle_point_vec_angle".to_string(), format!("{:.2}", m.middle_point_vec_angle)));
    fields.push(("  middle_point_vec_magnitude".to_string(), format!("{:.2}", m.middle_point_vec_magnitude)));
    // GameState
    fields.push(("[game]".to_string(), "".to_string()));
    let g = &state.game;
    fields.push(("  flags".to_string(), format!("{}", g.flags)));
    fields.push(("  is_running".to_string(), format!("{}", g.is_running())));
    fields.push(("  is_goalie".to_string(), format!("{}", g.is_goalie())));
    fields.push(("  target_is_yellow".to_string(), format!("{}", g.target_is_yellow())));
    fields.push(("  switch_wanted".to_string(), format!("{}", g.switch_wanted())));
    fields.push(("  fsm_state".to_string(), format!("{}", g.fsm_state())));
    // PeerRobot
    fields.push(("[peer]".to_string(), "".to_string()));
    let p = &state.peer;
    fields.push(("  global_x".to_string(), format!("{:.2}", p.global_x)));
    fields.push(("  global_y".to_string(), format!("{:.2}", p.global_y)));
    fields.push(("  heading".to_string(), format!("{:.2}", p.heading)));
    fields.push(("  ball_rot".to_string(), format!("{:.2}", p.ball_rot)));
    fields.push(("  ball_dist".to_string(), format!("{:.2}", p.ball_dist)));
    fields.push(("  flags".to_string(), format!("{}", p.flags)));
    fields.push(("  is_running".to_string(), format!("{}", p.is_running())));
    fields.push(("  is_goalie".to_string(), format!("{}", p.is_goalie())));
    fields.push(("  sees_line".to_string(), format!("{}", p.sees_line())));
    fields.push(("  ball_exists".to_string(), format!("{}", p.ball_exists())));
    fields.push(("  switch_wanted".to_string(), format!("{}", p.switch_wanted())));
    fields.push(("  peer_alive".to_string(), format!("{}", p.peer_alive())));
    // PrintVector
    fields.push(("[console_print]".to_string(), "".to_string()));
    let pv = &state.console_print;
    for (i, &val) in pv.print_vector.iter().enumerate() {
        fields.push((format!("  print_vector[{}]", i), format!("{:.2}", val)));
    }
    // Top-level:
    fields.push(("[top-level]".to_string(), "".to_string()));
    fields.push(("  esp_now_bot_id".to_string(), format!("{}", state.esp_now_bot_id)));
    fields
}

fn format_robot_state(state: &RobotState) -> Vec<String> {
    let mut lines = Vec::new();
    // VisionData
    lines.push("[vision]".to_string());
    let v = &state.vision;
    lines.push(format!("  heading: {:.2}", v.heading));
    lines.push(format!("  global_x: {:.2}", v.global_x));
    lines.push(format!("  global_y: {:.2}", v.global_y));
    lines.push(format!("  ball_rot: {:.2}", v.ball_rot));
    lines.push(format!("  ball_dist: {:.2}", v.ball_dist));
    lines.push(format!("  ball_exists: {}", v.ball_exists));
    lines.push(format!("  target_goal_rot: {:.2}", v.target_goal_rot));
    lines.push(format!("  target_goal_dist: {:.2}", v.target_goal_dist));
    lines.push(format!("  own_goal_rot: {:.2}", v.own_goal_rot));
    lines.push(format!("  own_goal_dist: {:.2}", v.own_goal_dist));
    lines.push(format!("  away_from_own_goal_angle: {:.2}", v.away_from_own_goal_angle));
    lines.push(format!("  target_goal_label: {}", v.target_goal_label));
    lines.push(format!("  own_goal_label: {}", v.own_goal_label));
    lines.push(format!("  num_detections: {}", v.num_detections));
    for (i, &val) in v.object_label.iter().enumerate() {
        lines.push(format!("  object_label[{}]: {}", i, val));
    }
    for (i, &val) in v.object_rot_deg.iter().enumerate() {
        lines.push(format!("  object_rot_deg[{}]: {:.2}", i, val));
    }
    for (i, &val) in v.object_dist_cm.iter().enumerate() {
        lines.push(format!("  object_dist_cm[{}]: {:.2}", i, val));
    }
    lines.push(format!("  cm5_running: {}", v.cm5_running));
    // SensorData
    lines.push("[sensors]".to_string());
    let s = &state.sensors;
    lines.push(format!("  line_rot: {}", s.line_rot));
    lines.push(format!("  progress: {}", s.progress));
    lines.push(format!("  line_seen: {}", s.line_seen));
    lines.push(format!("  has_ball: {}", s.has_ball));
    lines.push(format!("  ball_light_gate: {}", s.ball_light_gate));
    lines.push(format!("  ena: {}", s.ena));
    // MotionData
    lines.push("[motion]".to_string());
    let m = &state.motion;
    lines.push(format!("  velocity_x: {:.2}", m.velocity_x));
    lines.push(format!("  velocity_y: {:.2}", m.velocity_y));
    lines.push(format!("  velocity_magnitude: {:.2}", m.velocity_magnitude));
    lines.push(format!("  rotation_delta: {:.2}", m.rotation_delta));
    lines.push(format!("  middle_point_vec_x: {:.2}", m.middle_point_vec_x));
    lines.push(format!("  middle_point_vec_y: {:.2}", m.middle_point_vec_y));
    lines.push(format!("  middle_point_vec_angle: {:.2}", m.middle_point_vec_angle));
    lines.push(format!("  middle_point_vec_magnitude: {:.2}", m.middle_point_vec_magnitude));
    // GameState
    lines.push("[game]".to_string());
    let g = &state.game;
    lines.push(format!("  flags: {}", g.flags));
    lines.push(format!("  is_running: {}", g.is_running()));
    lines.push(format!("  is_goalie: {}", g.is_goalie()));
    lines.push(format!("  target_is_yellow: {}", g.target_is_yellow()));
    lines.push(format!("  switch_wanted: {}", g.switch_wanted()));
    lines.push(format!("  fsm_state: {}", g.fsm_state()));
    // PeerRobot
    lines.push("[peer]".to_string());
    let p = &state.peer;
    lines.push(format!("  global_x: {:.2}", p.global_x));
    lines.push(format!("  global_y: {:.2}", p.global_y));
    lines.push(format!("  heading: {:.2}", p.heading));
    lines.push(format!("  ball_rot: {:.2}", p.ball_rot));
    lines.push(format!("  ball_dist: {:.2}", p.ball_dist));
    lines.push(format!("  flags: {}", p.flags));
    lines.push(format!("  is_running: {}", p.is_running()));
    lines.push(format!("  is_goalie: {}", p.is_goalie()));
    lines.push(format!("  sees_line: {}", p.sees_line()));
    lines.push(format!("  ball_exists: {}", p.ball_exists()));
    lines.push(format!("  switch_wanted: {}", p.switch_wanted()));
    lines.push(format!("  peer_alive: {}", p.peer_alive()));
    // PrintVector
    lines.push("[console_print]".to_string());
    let pv = &state.console_print;
    for (i, &val) in pv.print_vector.iter().enumerate() {
        lines.push(format!("  print_vector[{}]: {:.2}", i, val));
    }
    // Top-level:
    lines.push("[top-level]".to_string());
    lines.push(format!("  esp_now_bot_id: {}", state.esp_now_bot_id));
    lines
}
