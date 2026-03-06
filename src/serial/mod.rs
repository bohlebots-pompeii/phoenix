pub mod reader;
pub mod parser;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn connect_serial(port_name: &str, baud: u32) -> Receiver<String> {
    let (tx, rx) = channel();

    let port = serialport::new(port_name, baud)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open serial port");

    reader::start_serial_thread(port, tx);

    rx
}