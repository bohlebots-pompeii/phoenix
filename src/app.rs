use eframe::egui;
use crate::widgets::window_manager::WindowManager;
use crate::windows::{FieldWindow, ConsoleWindow, GraphWindow, PlaybackWindow};
use std::sync::mpsc::{Receiver, channel};

pub struct SoccerToolApp {
    manager: WindowManager,

    rx: Option<Receiver<String>>,
    serial_enabled: bool,
}

impl SoccerToolApp {
    pub fn new(rx: Option<Receiver<String>>) -> Self {
        Self::new_with_dummy(false)
    }

    pub fn new_with_dummy(dummy_mode: bool) -> Self {
        use std::thread;
        use std::sync::mpsc;
        use std::time::Duration;
        use crate::data::robot_state::*;

        // The receiver used to simulate (dummy) or implement ('real') serial channel
        let rx = if dummy_mode {
            // Dummy mode: spawn thread that sends a new dummy RobotState every 100ms
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                loop {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    let state = RobotState {
                        vision: VisionData {
                            heading: rng.gen_range(0.0..360.0),
                            global_x: rng.gen_range(-60.0..60.0),
                            global_y: rng.gen_range(-90.0..90.0),
                            ball_rot: rng.gen_range(-180.0..180.0),
                            ball_dist: rng.gen_range(0.0..180.0),
                            ball_exists: rng.gen_bool(0.8),
                            target_goal_rot: rng.gen_range(-180.0..180.0),
                            target_goal_dist: rng.gen_range(0.0..220.0),
                            own_goal_rot: rng.gen_range(-180.0..180.0),
                            own_goal_dist: rng.gen_range(0.0..220.0),
                            away_from_own_goal_angle: rng.gen_range(0.0..180.0),
                            target_goal_label: rng.gen_range(1..=2),
                            own_goal_label: rng.gen_range(1..=2),
                            num_detections: 2,
                            object_label: [1, 2, 0, 0, 0, 0],
                            object_rot_deg: [rng.gen_range(-180.0..180.0), rng.gen_range(-180.0..180.0), 0.0, 0.0, 0.0, 0.0],
                            object_dist_cm: [rng.gen_range(0.0..200.0), rng.gen_range(0.0..200.0), 0.0, 0.0, 0.0, 0.0],
                            cm5_running: rng.gen_bool(0.9),
                        },
                        sensors: SensorData {
                            line_rot: rng.gen_range(-180..180),
                            progress: rng.gen_range(0..100),
                            line_seen: rng.gen_bool(0.8),
                            has_ball: rng.gen_bool(0.5),
                            ball_light_gate: rng.gen_range(80..150),
                            ena: rng.gen_bool(0.85),
                        },
                        motion: MotionData {
                            velocity_x: rng.gen_range(-25.0..25.0),
                            velocity_y: rng.gen_range(-25.0..25.0),
                            velocity_magnitude: rng.gen_range(0.0..36.0),
                            rotation_delta: rng.gen_range(-8.0..8.0),
                            middle_point_vec_x: rng.gen_range(-10.0..10.0),
                            middle_point_vec_y: rng.gen_range(-10.0..10.0),
                            middle_point_vec_angle: rng.gen_range(0.0..360.0),
                            middle_point_vec_magnitude: rng.gen_range(0.0..12.0),
                        },
                        game: GameState {
                            flags: ((rng.gen_bool(0.9) as u8) << 0) | ((rng.gen_bool(0.5) as u8) << 2) | (rng.gen_range(0..3) << 4),
                        },
                        peer: PeerRobot {
                            global_x: rng.gen_range(-75.0..75.0),
                            global_y: rng.gen_range(-105.0..105.0),
                            heading: rng.gen_range(0.0..360.0),
                            ball_rot: rng.gen_range(-180.0..180.0),
                            ball_dist: rng.gen_range(0.0..180.0),
                            flags: ((rng.gen_bool(0.8) as u8) << 0) | ((rng.gen_bool(0.2) as u8) << 3) | ((rng.gen_bool(0.7) as u8) << 5),
                        },
                        esp_now_bot_id: rng.gen_range(0..2),
                        console_print: PrintVector { print_vector: vec![rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0), rng.gen_range(0.0..10.0)] },
                    };

                    let _ = tx.send(serde_json::to_string(&state).unwrap());
                    thread::sleep(Duration::from_millis(100));
                }
            });
            Some(rx)
        } else {
            // PRODUCTION: Replace with real serial receive code, or keep None for now
            None
        };

        let mut manager = WindowManager::new();

        // Always start with default state, doesn't affect logging/logic
        let demo_robot_state = RobotState::default();
        manager.add_window(Box::new(ConsoleWindow::new()));
        manager.add_window(Box::new(GraphWindow::new()));
        manager.add_window(Box::new(PlaybackWindow::new()));
        manager.add_window(Box::new(FieldWindow::new(demo_robot_state)));


        Self {
            manager,
            rx,
            serial_enabled: false, // For extension: update if you use real serial
        }
    }
}

impl eframe::App for SoccerToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
        let mut got_new_data = false;

        if let Some(rx) = &self.rx {
            while let Ok(line) = rx.try_recv() {
                // Parse line into RobotState:
                if let Some(state) = crate::serial::parser::parse_line(&line) {
                    // Find FieldWindow and send state
                    for window in self.manager.windows.iter_mut() {
                        if let Some(field) = window.as_any().downcast_mut::<FieldWindow>() {
                            field.on_new_state(state.clone());
                        }
                    }
                    got_new_data = true; // We got at least one message this frame
                }
            }
        }
        // ---
        // This ensures GUI repaints instantly on new data, even if user is not interacting.
        // This works for both real and simulated (dummy) data modes.
        if got_new_data {
            ctx.request_repaint();
        }
        // ---
        self.manager.draw(ctx);
    }
}