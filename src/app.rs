use eframe::egui;
use crate::widgets::window_manager::WindowManager;
use crate::windows::{FieldWindow, ConsoleWindow, GraphWindow, PlaybackWindow, RawSerialWindow, RawPlaybackWindow, LayoutWindow, WindowConfig};
use crate::windows::serial_settings::{SerialStatus};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

pub enum SerialAction {
    Connect {
        port: String,
        baud: u32,
    },
    Disconnect,
}

pub struct SoccerToolApp {
    manager: WindowManager,
    window_config: WindowConfig,
    pub app_width: f32,
    pub app_height: f32,

    rx: Option<Receiver<String>>,
    serial_enabled: bool,
    // Serial runtime state:
    pending_serial_action: Option<SerialAction>,
    serial_status: SerialStatus,
    serial_error: Option<String>,
    serial_thread_handle: Option<JoinHandle<()>>,
    serial_stop_tx: Option<Sender<()>>,
}


impl SoccerToolApp {
    pub fn new(_rx: Option<Receiver<String>>) -> Self {
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
            // Real serial mode: don't connect at startup, wait for user to connect via GUI
            None
        };


        let mut manager = WindowManager::new();

        // Always start with default state, doesn't affect logging/logic
        let demo_robot_state = RobotState::default();
        manager.add_window(Box::new(ConsoleWindow::new()));
        manager.add_window(Box::new(GraphWindow::new()));
        manager.add_window(Box::new(PlaybackWindow::new()));
        manager.add_window(Box::new(FieldWindow::new(demo_robot_state)));
        manager.add_window(Box::new(RawSerialWindow::new()));
        manager.add_window(Box::new(RawPlaybackWindow::new()));
        manager.add_window(Box::new(LayoutWindow::new()));


        manager.add_window(Box::new(crate::windows::SerialSettingsWindow::new()));
        let serial_enabled = !dummy_mode && rx.is_some();
        Self {
            manager,
            window_config: WindowConfig::default(),
            app_width: 0.0,
            app_height: 0.0,
            rx,
            serial_enabled,
            pending_serial_action: None,
            serial_status: SerialStatus::Disconnected,
            serial_error: None,
            serial_thread_handle: None,
            serial_stop_tx: None,
        }
    }
}

impl eframe::App for SoccerToolApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
        let app_rect = ctx.screen_rect();
        self.app_width = app_rect.width();
        self.app_height = app_rect.height();

        // Ensure all panel rects are initialized with the current app size
        if self.window_config.panels.len() < 8 { // 8 = number of panels in layout
            self.window_config.apply_layout(self.window_config.selected_layout_idx, self.app_width, self.app_height);
        }

        let mut got_new_data = false;

        // Poll SerialSettingsWindow for pending connect/disconnect actions
        if !self.serial_enabled {
            let mut pending_connect: Option<(String, u32)> = None;
            let mut pending_disconnect = false;
            for window in self.manager.windows.iter_mut() {
                if let Some(sw) = window.as_any_mut().downcast_mut::<crate::windows::SerialSettingsWindow>() {
                    if let Some(action) = sw.pending_connect.take() {
                        pending_connect = Some(action);
                    }
                    if sw.pending_disconnect {
                        sw.pending_disconnect = false;
                        pending_disconnect = true;
                    }
                }
            }
            if let Some((port, baud)) = pending_connect {
                match crate::serial::connect_serial(&port, baud) {
                    Some(rx) => {
                        self.rx = Some(rx);
                        self.serial_enabled = true;
                        for window in self.manager.windows.iter_mut() {
                            if let Some(sw) = window.as_any_mut().downcast_mut::<crate::windows::SerialSettingsWindow>() {
                                sw.status = crate::windows::serial_settings::SerialStatus::Connected;
                                sw.error_message = None;
                            }
                        }
                    }
                    None => {
                        for window in self.manager.windows.iter_mut() {
                            if let Some(sw) = window.as_any_mut().downcast_mut::<crate::windows::SerialSettingsWindow>() {
                                sw.status = crate::windows::serial_settings::SerialStatus::Error;
                                sw.error_message = Some(format!("Failed to open port: {}", port));
                            }
                        }
                    }
                }
            }
            if pending_disconnect {
                self.rx = None;
                self.serial_enabled = false;
            }
        }

        if let Some(rx) = &self.rx {
            while let Ok(line) = rx.try_recv() {
                // Deliver raw line to all RawSerialWindows
                for window in self.manager.windows.iter_mut() {
                    if let Some(raw_serial) = window.as_any_mut().downcast_mut::<RawSerialWindow>() {
                        raw_serial.on_new_serial_line(&line);
                    }
                }
                // Parse line into RobotState:
                if let Some(state) = crate::serial::parser::parse_line(&line) {
                    // Find FieldWindow and send state
                    for window in self.manager.windows.iter_mut() {
                        if let Some(field) = window.as_any_mut().downcast_mut::<FieldWindow>() {
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
        self.manager.draw(ctx, &mut self.window_config, self.app_width, self.app_height);
    }
}