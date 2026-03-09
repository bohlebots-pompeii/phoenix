use egui::{Context, Window};
use crate::windows::{Window as WindowTrait, WindowConfig};
use serialport::SerialPortInfo;
use std::any::Any;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum SerialStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error,
}

pub struct SerialSettingsWindow {
    pub port_candidates: Vec<SerialPortInfo>,   // List of discovered serial ports
    pub selected_port: Option<String>,          // Currently selected port (name)
    pub baud: u32,                             // Selected baud rate
    pub status: SerialStatus, 
    pub error_message: Option<String>,          // latest error shown
    pub last_status: Option<SerialStatus>, // Used to trigger polling/updates
    pub pending_connect: Option<(String, u32)>, // Some((port, baud)) when Connect is clicked
    pub pending_disconnect: bool,               // true when Disconnect is clicked
}

impl SerialSettingsWindow {
    pub fn new() -> Self {
        let port_candidates = serialport::available_ports().unwrap_or_else(|_| vec![]);
        SerialSettingsWindow {
            port_candidates,
            selected_port: None,
            baud: 115200,
            status: SerialStatus::Disconnected,
            error_message: None,
            last_status: None,
            pending_connect: None,
            pending_disconnect: false,
        }
    }

    pub fn refresh_ports(&mut self) {
        self.port_candidates = serialport::available_ports().unwrap_or_else(|_| vec![]);
    }

    pub fn draw(&mut self, ctx: &Context, _config: &mut WindowConfig, _app_width: f32, _app_height: f32) {
        let rect = _config.serial_settings_rect(_app_width, _app_height);
        let response = Window::new("Serial Settings")
            .default_width(rect.width())
            .default_height(rect.height())
            .default_pos([rect.left(), rect.top()])
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Refresh").clicked() {
                        self.refresh_ports();
                    }
                    ui.label("Serial Port:");
                    if self.port_candidates.is_empty() {
                        ui.label("No ports found");
                    } else {
                        egui::ComboBox::from_id_source("serialport-combo").selected_text(
                            self.selected_port.clone().unwrap_or_else(|| "Select...".into()),
                        ).show_ui(ui, |ui| {
                            for port in &self.port_candidates {
                                let id = &port.port_name;
                                ui.selectable_value(&mut self.selected_port, Some(id.clone()), &port.port_name);
                            }
                        });
                    }
                    ui.label("Baud:");
                    let mut baud_buf = self.baud.to_string();
                    if ui.text_edit_singleline(&mut baud_buf).changed() {
                        if let Ok(val) = baud_buf.parse::<u32>() {
                            self.baud = val;
                        }
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    let connect_enabled = self.status == SerialStatus::Disconnected && self.selected_port.is_some();
                    if ui.add_enabled(connect_enabled, egui::Button::new("Connect")).clicked() {
                        self.status = SerialStatus::Connecting;
                        if let Some(ref port) = self.selected_port {
                            self.pending_connect = Some((port.clone(), self.baud));
                        }
                    }
                    if ui.add_enabled(self.status == SerialStatus::Connected, egui::Button::new("Disconnect")).clicked() {
                        self.status = SerialStatus::Disconnected;
                        self.pending_disconnect = true;
                    }
                    ui.label(format!("Status: {:?}", self.status));
                });
                if let Some(ref err) = self.error_message {
                    ui.colored_label(egui::Color32::RED, err);
                }
            });

        // Persist latest rectangle
        if let Some(window_response) = response {
            let rect = window_response.response.rect;
            _config.serial_settings = [rect.left(), rect.top(), rect.width(), rect.height()];
        }
    }
}

impl WindowTrait for SerialSettingsWindow {
    fn draw(&mut self, ctx: &Context, config: &mut WindowConfig, app_width: f32, app_height: f32) {
        self.draw(ctx, config, app_width, app_height)
    }
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
