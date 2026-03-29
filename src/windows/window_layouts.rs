use std::any::Any;
use egui::Context;

use crate::windows::panel_id::PanelId;
use crate::windows::window_config::WindowConfig;
use crate::windows::layout_utils::{compute_scale, scale_rect};
use crate::windows::Window as WindowTrait;

// ---------------------------------------------------------------------------
// Fallback position for the layout window before any layout is applied.
// Exported so WindowConfig::default() can insert it without a circular dep.
// ---------------------------------------------------------------------------
pub const LAYOUT_WINDOW_LAYOUTS_FALLBACK: [f32; 4] = [80.0, 80.0, 320.0, 360.0];

// ---------------------------------------------------------------------------
// Layout definitions (reference resolution: 2560 × 1440)
// To add a new panel: add a variant to PanelId, then add it to each layout.
// ---------------------------------------------------------------------------

pub struct WindowLayout {
    pub name: &'static str,
    /// Slice of (PanelId, [x, y, w, h]) pairs at reference resolution.
    pub panels: &'static [(PanelId, [f32; 4])],
}

pub const LAYOUT_DEFAULT: WindowLayout = WindowLayout {
    name: "Default",
    panels: &[
        (PanelId::Console,       [2095.0,  0.0,   900.0,  95.0 ]),
        (PanelId::Field,         [0.0,     664.5, 1200.0, 900.0 ]),
        (PanelId::FieldPlayback, [100.0,   100.0, 900.0,  900.0 ]),
        (PanelId::Graph,         [1600.0,  0.0,   700.0,  600.0 ]),
        (PanelId::RawPlayback,   [0.0,     900.0, 1300.0, 250.0 ]),
        (PanelId::RawSerial,     [0.0,     1150.0,1300.0, 250.0 ]),
        (PanelId::SerialSettings,[100.0,   100.0, 900.0,  900.0 ]),
        (PanelId::WindowLayouts, [80.0,    80.0,  320.0,  360.0 ]),
    ],
};

pub const LAYOUT_NORMAL: WindowLayout = WindowLayout {
    name: "Normal",
    panels: &[
        (PanelId::Console,       [2150.0, 0.0,   600.0, 1560.0]),
        (PanelId::Field,         [0.0,    0.0,   550.0,  780.0]),
        (PanelId::FieldPlayback, [570.0,  0.0,   550.0,  780.0]),
        (PanelId::Graph,         [1200.0, 900.0, 500.0,  400.0]),
        (PanelId::RawPlayback,   [570.0,  827.0, 550.0,  733.0]),
        (PanelId::RawSerial,     [0.0,    827.0, 550.0,  733.0]),
        (PanelId::SerialSettings,[1140.0, 0.0,   900.0,  900.0]),
        (PanelId::WindowLayouts, [1140.0, 60.0,  300.0,  340.0]),
    ],
};

pub const ALL_LAYOUTS: &[&WindowLayout] = &[
    &LAYOUT_DEFAULT,
    &LAYOUT_NORMAL,
];

// ---------------------------------------------------------------------------
// LayoutWindow — only responsible for drawing the selector UI.
// All state lives in WindowConfig; this struct holds no data of its own.
// ---------------------------------------------------------------------------

pub struct LayoutWindow;

impl LayoutWindow {
    pub fn new() -> Self {
        Self
    }

    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        config: &mut WindowConfig,
        app_width: f32,
        app_height: f32,
    ) {
        // Read position BEFORE any mutation this frame.
        // Using current_pos() means egui applies it every frame, so
        // the window always tracks the config — no reset_areas() needed.
        let pos  = config.pos(PanelId::WindowLayouts);
        let size = config.size(PanelId::WindowLayouts);

        egui::Window::new("Window Layout")
            .current_pos(pos)   // applied every frame — fixes the same-frame ordering bug
            .default_size(size)
            .show(ctx, |ui| {
                // --- Layout selector buttons ---
                ui.horizontal(|ui| {
                    for (i, layout) in ALL_LAYOUTS.iter().enumerate() {
                        let selected = config.selected_layout_idx == i;
                        if ui.selectable_label(selected, layout.name).clicked() {
                            // apply_layout writes ALL panels including WindowLayouts,
                            // so this window will move next frame automatically.
                            config.apply_layout(i, app_width, app_height);
                            // No reset_areas() needed — current_pos() handles it.
                        }
                    }
                });

                ui.separator();

                // --- Optional debug readout (remove or gate behind a flag) ---
                let scale = compute_scale(app_width, app_height);
                let layout = ALL_LAYOUTS[config.selected_layout_idx];
                ui.label(format!("Scale: {:.3}", scale));
                for (id, rect) in layout.panels {
                    let scaled = scale_rect(*rect, scale);
                    ui.label(format!("{:?}: [{:.0}, {:.0}, {:.0}, {:.0}]",
                        id, scaled[0], scaled[1], scaled[2], scaled[3]));
                }
            });
    }
}

// ---------------------------------------------------------------------------
// WindowTrait impl
// ---------------------------------------------------------------------------

impl WindowTrait for LayoutWindow {
    fn draw(
        &mut self,
        ctx: &Context,
        config: &mut WindowConfig,
        app_width: f32,
        app_height: f32,
    ) {
        self.draw(ctx, config, app_width, app_height);
    }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
