use egui::Context;
use crate::windows::Window;

pub struct WindowManager {
    pub windows: Vec<Box<dyn Window>>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self { windows: Vec::new() }
    }

    pub fn add_window(&mut self, window: Box<dyn Window>) {
        self.windows.push(window);
    }

    pub fn draw(&mut self, ctx: &Context) {
        use crate::windows::{RawPlaybackWindow, PlaybackWindow};
        // Find PlaybackWindow and prepare snapshot for playback
        let playback_snapshot = self.windows.iter()
            .find_map(|w| w.as_any().downcast_ref::<PlaybackWindow>())
            .map(|pbw| {
                use crate::windows::raw_playback::PlaybackSnapshot;
                PlaybackSnapshot {
                    playback_states: pbw.playback_states.clone(),
                    playhead: pbw.playhead,
                }
            });

        for window in self.windows.iter_mut() {
            if let Some(raw_playback) = window.as_any_mut().downcast_mut::<RawPlaybackWindow>() {
                raw_playback.draw_with_playback(ctx, playback_snapshot.clone());
            } else {
                window.draw(ctx);
            }
        }
    }
}