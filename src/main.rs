use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl = sdl2::init().expect("SDL init failed");
    let video = sdl.video().expect("video subsystem");

    let window = video
        .window("SDL2 Window", 1280, 720)
        .resizable()
        .build()
        .expect("window");

    let mut event_pump = sdl.event_pump().expect("event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        // Normally you would render here (OpenGL/egui/etc.)
        // For now the window just exists.

        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}