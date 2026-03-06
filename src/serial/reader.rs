use std::io::{BufRead, BufReader};
use serialport::SerialPort;
use std::sync::mpsc::Sender;
use std::thread;

pub fn start_serial_thread(port: Box<dyn SerialPort>, tx: Sender<String>) {
    thread::spawn(move || {
        read_lines(port, |line| {
            let _ = tx.send(line);
        });
    });
}

pub fn read_lines(mut port: Box<dyn SerialPort>, on_line: impl Fn(String)) {
    let mut reader = BufReader::new(port);
    let mut buf = String::new();

    loop {
        buf.clear();

        match reader.read_line(&mut buf) {
            Ok(0) => continue,
            Ok(_) => on_line(buf.trim_end().to_string()),
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }
    }
}