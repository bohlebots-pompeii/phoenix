use eframe::egui;
use crate::widgets::window_manager::WindowManager;
use crate::windows::{FieldWindow, ConsoleWindow, GraphWindow};
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
                    let state = RobotState {
    vision: VisionData {
        heading: 45.0,
        global_x: 30.0,
        global_y: -50.0,
        ball_rot: 30.0,
        ball_dist: 60.0,
        ball_exists: true,
        target_goal_rot: -10.0,
        target_goal_dist: 170.0,
        own_goal_rot: 170.0,
        own_goal_dist: 140.0,
        away_from_own_goal_angle: 85.0,
        target_goal_label: 1,
        own_goal_label: 2,
        num_detections: 2,
        object_label: [1, 2, 0, 0, 0, 0],
        object_rot_deg: [15.0, -90.0, 0.0, 0.0, 0.0, 0.0],
        object_dist_cm: [80.0, 180.0, 0.0, 0.0, 0.0, 0.0],
        cm5_running: true,
    },
    sensors: SensorData {
        line_rot: -70,
        progress: 12,
        line_seen: true,
        has_ball: true,
        ball_light_gate: 100,
        ena: true,
    },
    motion: MotionData {
        velocity_x: 10.0,
        velocity_y: -12.5,
        velocity_magnitude: 16.0,
        rotation_delta: 3.5,
        middle_point_vec_x: 5.0,
        middle_point_vec_y: 10.0,
        middle_point_vec_angle: 63.0,
        middle_point_vec_magnitude: 11.2,
    },
    game: GameState {
        flags: (1 << 0) | (1 << 2) | (2 << 4), // running, target_is_yellow, state=2
    },
    peer: PeerRobot {
        global_x: -40.0,
        global_y: 55.0,
        heading: 135.0,
        ball_rot: -45.0,
        ball_dist: 80.0,
        flags: (1 << 0) | (1 << 3) | (1 << 5), // running, ball_exists, peer_alive
    },
    esp_now_bot_id: 7,
    console_print: PrintVector { print_vector: vec![1.1, 2.2, 3.3] },
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
        manager.add_window(Box::new(FieldWindow::new(demo_robot_state)));
        manager.add_window(Box::new(ConsoleWindow::new()));
        manager.add_window(Box::new(GraphWindow::new()));

        Self {
            manager,
            rx,
            serial_enabled: false, // For extension: update if you use real serial
        }
    }
}

impl eframe::App for SoccerToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

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
                }
            }
        }

        self.manager.draw(ctx);
    }
}