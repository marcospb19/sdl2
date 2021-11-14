use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window, Sdl};

const WINDOW_NAME: &str = "rust-sdl2 demo";
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;

fn should_exit(event: Event) -> bool {
    matches!(event, Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. })
}

fn start_window() -> (Sdl, Canvas<Window>) {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();

    let window = video_subsystem.window(WINDOW_NAME, WIDTH, HEIGHT).position_centered().build().unwrap();

    let canvas = window.into_canvas().build().unwrap();
    (context, canvas)
}

pub fn main() {
    let (context, mut canvas) = start_window();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            if should_exit(event) {
                break 'running;
            }
        }

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
