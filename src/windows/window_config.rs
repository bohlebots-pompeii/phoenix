use std::collections::HashMap;
use crate::windows::panel_id::PanelId;
use crate::windows::layout_utils::{compute_scale, rect_from_array, scale_rect};
use crate::windows::window_layouts::{ALL_LAYOUTS, LAYOUT_WINDOW_LAYOUTS_FALLBACK};

/// Pure state: current panel positions/sizes and which layout is active.
/// No utility methods, no scaling logic — those live in layout_utils.rs.
pub struct WindowConfig {
    /// Current [x, y, w, h] for every managed panel.
    pub panels: HashMap<PanelId, [f32; 4]>,

    /// Index into ALL_LAYOUTS that is currently active.
    pub selected_layout_idx: usize,

    /// When true, panels are pinned to layout positions each frame via
    /// current_pos(). Set to false to let the user drag freely.
    pub layout_locked: bool,
}

impl WindowConfig {
    /// Build a config scaled to the given viewport at startup.
    pub fn with_scale(app_width: f32, app_height: f32) -> Self {
        let mut cfg = Self::default();
        cfg.apply_layout(0, app_width, app_height);
        cfg
    }

    /// Apply layout at `index` from ALL_LAYOUTS, scaling to the viewport.
    /// This is the single place that writes panel positions.
    pub fn apply_layout(&mut self, index: usize, app_width: f32, app_height: f32) {
        let layout = ALL_LAYOUTS[index];
        let scale = compute_scale(app_width, app_height);

        for (id, rect) in layout.panels {
            self.panels.insert(*id, scale_rect(*rect, scale));
        }

        self.selected_layout_idx = index;
    }

    /// Returns the current rect for a panel, or a zero rect if unknown.
    pub fn rect(&self, id: PanelId) -> [f32; 4] {
        self.panels.get(&id).copied().unwrap_or([0.0; 4])
    }

    /// Returns the current rect as an egui::Rect.
    pub fn egui_rect(&self, id: PanelId) -> egui::Rect {
        rect_from_array(self.rect(id))
    }

    /// Returns just the top-left pos as egui::Pos2.
    pub fn pos(&self, id: PanelId) -> egui::Pos2 {
        let r = self.rect(id);
        egui::pos2(r[0], r[1])
    }

    /// Returns just the size as egui::Vec2.
    pub fn size(&self, id: PanelId) -> egui::Vec2 {
        let r = self.rect(id);
        egui::vec2(r[2], r[3])
    }

    /// Call this when the user drags a panel manually so it doesn't snap back.
    pub fn update_panel_rect(&mut self, id: PanelId, rect: egui::Rect) {
        self.panels.insert(id, [
            rect.left(),
            rect.top(),
            rect.width(),
            rect.height(),
        ]);
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        let mut panels = HashMap::new();

        // Give the layout window a sensible position before the first
        // layout is applied, so it is visible on startup.
        panels.insert(PanelId::WindowLayouts, LAYOUT_WINDOW_LAYOUTS_FALLBACK);

        Self {
            panels,
            selected_layout_idx: 0,
            layout_locked: true,
        }
    }
}
