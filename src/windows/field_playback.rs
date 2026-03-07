use std::path::PathBuf;
use egui::{Color32, Context, Window};
//use crate::windows::field::SerializableTimedState;
use serde::{Serialize, Deserialize};
use crate::data::robot_state::RobotState;

/// Holds a single robot state with its replay timestamp.
#[derive(Serialize, Deserialize, Clone)]
pub struct SerializableTimedState {
    /// UNIX timestamp (seconds, fractional OK)
    pub timestamp: f64,
    /// Snapshot of the robot state at this moment
    pub state: RobotState,
}

/// UI and replay logic for selecting, playing, and visualizing log files.
/// 
/// Maintains file list, playhead, playback state, error display.
pub struct PlaybackWindow {
    /// List of available .json log files in /replays/
    pub replay_files: Vec<String>,
    /// Index of file currently selected in the dropdown
    pub selected_replay_idx: usize,
    /// States loaded from the chosen replay file
    pub playback_states: Vec<SerializableTimedState>,
    /// Current frame/playhead index into playback_states
    pub playhead: usize,
    /// Should playback auto-advance?
    pub playing: bool,
    /// Monotonic wall time (seconds) at last playhead update (for smooth playback)
    pub last_update_time: f64,
    /// If present, describes current user-facing file/parsing error
    pub playback_error: Option<String>,
    /// UNIX time of last directory scan (auto-refresh support)
    pub last_scan_time: f64, // for directory rescan every N seconds
}

impl super::Window for PlaybackWindow {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn draw(&mut self, ctx: &Context) {
        use std::time::{SystemTime, UNIX_EPOCH};
        // --- Auto refresh replay_files every second ---
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        if now - self.last_scan_time > 1.0 {
            self.last_scan_time = now;
            match std::fs::read_dir("replays") {
                Ok(entries) => {
                    let mut names: Vec<String> = entries.filter_map(|e| e.ok())
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .filter(|n| n.ends_with(".json"))
                        .collect();
                    names.sort();
                    if names != self.replay_files {
                        self.replay_files = names;
                        self.selected_replay_idx = 0;
                        self.load_selected_replay();
                    }
                },
                Err(_) => {
                    self.replay_files.clear();
                    self.selected_replay_idx = 0;
                    self.playback_states.clear();
                    self.playback_error = Some("Could not read /replays directory".to_owned());
                },
            }
        }
        Window::new("Replay Window")
            .resizable(true)
            .show(ctx, |ui| {
                // --- Replay file selector ---
                if !self.replay_files.is_empty() {
                    egui::ComboBox::from_label("Select Replay File")
                        .selected_text(self.replay_files[self.selected_replay_idx].clone())
                        .show_ui(ui, |cb_ui| {
                            for (i, fname) in self.replay_files.iter().enumerate() {
                                cb_ui.selectable_value(&mut self.selected_replay_idx, i, fname);
                            }
                        });
                    if ui.add(egui::Button::new("Load Replay")).on_hover_text("Reloads the selected replay file").clicked() {
                        self.load_selected_replay();
                    }
                } else {
                    ui.label("No replay files found in /replays");
                }
                // --- Error display ---
                if let Some(ref err) = self.playback_error {
                    egui::Frame::none().fill(Color32::DARK_RED).show(ui, |ui| {
                     ui.colored_label(Color32::WHITE, format!("⛔ Error: {err}"));
                });
                }
                // === Playback Controls ===
                if !self.playback_states.is_empty() {
                    // --- Step/Speed controls row ---
                    ui.horizontal(|ui| {
                        // Playback controls group
                        if self.playing {
                            if ui.add(egui::Button::new("Pause")).on_hover_text("Pause playback").clicked() {
                                self.playing = false;
                            }
                        } else {
                            if ui.add(egui::Button::new("Play")).on_hover_text("Start playback (spacebar)").clicked() {
                                self.playing = true;
                                self.last_update_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                            }
                        }
                        // Step Backward
                        if ui.add(egui::Button::new("<")).on_hover_text("Previous frame").clicked() {
                            if self.playhead > 0 {
                                self.playhead -= 1;
                                self.playing = false;
                                self.last_update_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                            }
                        }
                        // Step Forward
                        if ui.add(egui::Button::new("> ")).on_hover_text("Next frame").clicked() {
                            if self.playhead + 1 < self.playback_states.len() {
                                self.playhead += 1;
                                self.playing = false;
                                self.last_update_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                            }
                        }
                        // --- Playback speed selector ---
                        static SPEEDS: &[(&str, f64)] = &[ ("0.5x", 0.5), ("1x", 1.0), ("2x", 2.0) ];
                        let mut speed = 1.0f64;
                        ui.ctx().memory_mut(|mem| {
                        if mem.data.get_persisted::<f64>("REPLAY_SPEED".into()).is_none() {
                                mem.data.insert_persisted("REPLAY_SPEED".into(), 1.0f64);
                            }
                            if let Some(val) = mem.data.get_persisted::<f64>("REPLAY_SPEED".into()) {
                                speed = val;
                            }
                        });
                        egui::ComboBox::from_id_source("replay_speed").selected_text(format!("Speed: {}x", speed))
                            .show_ui(ui, |cb_ui| {
                                for (label, val) in SPEEDS {
                                    if cb_ui.selectable_label((speed-*val).abs() < 0.01, *label)
                                        .on_hover_text(format!("Set playback speed to {}", *label)).clicked() {
                                        speed = *val;
                                    }
                                }
                            });
                        ui.ctx().memory_mut(|mem| mem.data.insert_persisted("REPLAY_SPEED".into(), speed));
                        // Frame and total
                        ui.label(format!("Frame {}/{}", self.playhead + 1, self.playback_states.len()));
                    });
                    // Time slider (seek bar)
                    let mut slider = self.playhead as usize;
                    let total = self.playback_states.len();
                    let slider_range = 0..=if total > 0 { total - 1 } else { 0 };
                    let slider_response = ui.add(egui::Slider::new(&mut slider, slider_range)
                            .text("Timeline")
                            .show_value(true));
let changed = slider_response.on_hover_text("Drag to seek to a specific frame").changed();
                    if changed {
                        self.playhead = slider;
                        self.last_update_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                        self.playing = false;
                    }
                }
                // === Advance playhead if playing ===
                // --- Defensive safety for playhead bounds ---
                let num_frames = self.playback_states.len();
                if self.playhead >= num_frames && num_frames > 0 {
                    self.playhead = num_frames - 1;
                }
                if self.playback_states.is_empty() {
                    self.playhead = 0;
                    self.playing = false;
                }
                // --- Advance playhead if playing ---
                if self.playing && num_frames > 1 && self.playhead < num_frames - 1 {
                    let real_now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
                    if self.last_update_time == 0.0 { self.last_update_time = real_now; }
                    let advance = {
                        let ts_now = self.playback_states.get(self.playhead).map(|s| s.timestamp).unwrap_or(0.0);
                        let ts_next = self.playback_states.get(self.playhead + 1).map(|s| s.timestamp).unwrap_or(ts_now+1.0);
                        let dt_real = real_now - self.last_update_time;
                        let dt_replay = ts_next - ts_now;
                        dt_real >= dt_replay.max(0.01)
                    };
                    if advance {
                        self.playhead += 1;
                        if self.playhead >= num_frames {
                            self.playhead = num_frames - 1;
                        }
                        self.last_update_time = real_now;
                    }
                    if self.playhead >= num_frames - 1 {
                        self.playing = false;
                    }
                }
                // --- Field Visualization of replayed RobotState ---
                ui.separator();
                let (rect, _response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
                let painter = ui.painter();
                use crate::windows::field::draw_field;
                if let Some(state) = self.playback_states.get(self.playhead) {
                    draw_field(painter, rect, &state.state);
                } else {
                    painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 0, 0));
                    painter.text(rect.center(), egui::Align2::CENTER_CENTER, "No state loaded", egui::FontId::default(), Color32::WHITE);
                }
            });
    }
}

impl PlaybackWindow {
    /// Create a new, empty playback window (usually called on app start).
    pub fn new() -> Self {
        Self { 
            replay_files: vec![], 
            selected_replay_idx: 0, 
            playback_states: vec![], 
            playhead: 0, 
            playing: false, 
            last_update_time: 0.0, 
            playback_error: None,
            last_scan_time: 0.0, // initialize
        }
    }

    /// Loads the replay file currently selected in the dropdown.
    /// Populates `playback_states` and resets playhead to 0.
    /// On parse errors or FS errors, sets `playback_error` for the UI to display.
    pub fn load_selected_replay(&mut self) {
        if self.replay_files.is_empty() {
            self.playback_states.clear();
            self.playback_error = Some("No replay files found".to_string());
            return;
        }
        let frame = &self.replay_files[self.selected_replay_idx];

        let path = format!("replays/{}", frame);
        match std::fs::File::open(&path) {
            Ok(file) => {
                match serde_json::from_reader::<_, Vec<SerializableTimedState>>(file) {
                    Ok(states) => {
                        self.playhead = 0;
                        self.playback_states = states;
                        self.playing = false;
                        self.playback_error = None;
                    },
                    Err(e) => {
                        self.playback_states.clear();
                        self.playback_error = Some(format!("Failed to parse replay: {}", e));
                    }
                }
            }
            Err(e) => {
                self.playback_states.clear();
                self.playback_error = Some(format!("Failed to open replay: {}", e));
            }
        }
    }
}
