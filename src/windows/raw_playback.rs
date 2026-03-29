use std::any::Any;
use egui::{Context, Window};
use crate::windows::{Window as WindowTrait, WindowConfig};
use crate::data::robot_state::RobotState;
use crate::windows::panel_id::PanelId;

// Helper reused from raw_serial.rs
use crate::windows::raw_serial::robot_state_fields;

pub struct RawPlaybackWindow;

impl RawPlaybackWindow {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub struct PlaybackSnapshot {
    pub playback_states: Vec<crate::windows::field_playback::SerializableTimedState>,
    pub playhead: usize,
}

impl RawPlaybackWindow {
    pub fn draw_with_playback(
        &mut self,
        ctx: &Context,
        config: &WindowConfig,
        app_width: f32,
        app_height: f32,
        playback: Option<PlaybackSnapshot>,
    ) {
        let rect = config.panels.get(&PanelId::RawPlayback).unwrap();
        Window::new(format!("Raw Playback [{}]", config.selected_layout_idx))
            .default_width(rect[2])
            .default_height(rect[3])
            .default_pos([rect[0], rect[1]])
            .resizable(true)
            .show(ctx, |ui| {
                let mut latest_bot0: Option<RobotState> = None;
                let mut latest_bot1: Option<RobotState> = None;
                if let Some(playback) = playback {
                    for frameidx in 0..=playback.playhead {
                        if let Some(fr) = playback.playback_states.get(frameidx) {
                            match fr.state.esp_now_bot_id {
                                0 => latest_bot0 = Some(fr.state.clone()),
                                1 => latest_bot1 = Some(fr.state.clone()),
                                _ => {},
                            }
                        }
                    }
                }
                egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                    let mut labels = std::collections::BTreeSet::<String>::new();
                    let fields0: Option<Vec<(String, String)>> = latest_bot0.as_ref().map(|state| robot_state_fields(state));
                    let fields1: Option<Vec<(String, String)>> = latest_bot1.as_ref().map(|state| robot_state_fields(state));
                    if let Some(ref f0) = fields0 {
                        for (k, _) in f0.iter() {
                            labels.insert(k.clone());
                        }
                    }
                    if let Some(ref f1) = fields1 {
                        for (k, _) in f1.iter() {
                            labels.insert(k.clone());
                        }
                    }
                    let labels: Vec<String> = labels.into_iter().collect();
                    egui::Grid::new("raw-playback-table").striped(true)
                        .min_col_width(110.0)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                        ui.heading("Field");
                        ui.heading("Bot 0");
                        ui.heading("Bot 1");
                        ui.end_row();
                        for label in labels.iter() {
                            let val0 = fields0.as_ref().and_then(|fs| fs.iter().find(|(k,_)| k==label)).map(|(_,v)| v.as_str()).unwrap_or("-");
                            let val1 = fields1.as_ref().and_then(|fs| fs.iter().find(|(k,_)| k==label)).map(|(_,v)| v.as_str()).unwrap_or("-");
                            ui.label(label);
                            ui.monospace(val0);
                            ui.monospace(val1);
                            ui.end_row();
                        }
                        if latest_bot0.is_none() && latest_bot1.is_none() {
                            ui.label("No packet data for either bot yet.");
                            ui.end_row();
                        }
                    });
                });
            });
    }
}

impl WindowTrait for RawPlaybackWindow {
    fn draw(&mut self, _ctx: &Context, _config: &mut WindowConfig, _app_width: f32, _app_height: f32) {
        // Fallback for legacy uses; do nothing
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
