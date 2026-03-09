pub mod reader;
pub mod parser;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn connect_serial(port_name: &str, baud: u32) -> Option<Receiver<String>> {
    let (tx, rx) = channel();

    match serialport::new(port_name, baud)
        .timeout(Duration::from_millis(100))
        .open() {
        Ok(port) => {
            reader::start_serial_thread(port, tx);
            Some(rx)
        },
        Err(e) => {
            eprintln!("Failed to open serial port {}: {}", port_name, e);
            None
        }
    }
}