#![windows_subsystem = "windows"]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use core::Emulator;

use pacman::util;

const WIDTH: u32 = 224;
const HEIGHT: u32 = 256;
const SCALE: f32 = 2.0;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Pac-Man", (WIDTH as f32 * SCALE) as u32, (HEIGHT as f32 * SCALE) as u32)
        .position_centered()
        .build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_scale(SCALE, SCALE)?;
    canvas.present();

    let program = include_bytes!("../assets/pacman");
    let mut emulator = Emulator::new(program);

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Some(keycode), keymod, .. } if util::has_ctrl(keymod) => {
                    match keycode {
                         Keycode::Q => break 'main,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        emulator.step().map_err(|e| e.to_string())?;
    }

    Ok(())
}
