use sdl2::{
    event::Event,
    gfx::{framerate::FPSManager, primitives::DrawRenderer},
    keyboard::{KeyboardState, Keycode, Scancode},
    pixels::Color,
    render::Canvas,
    video::Window,
    EventPump, Sdl,
};

const WINDOW_NAME: &str = "rust-sdl2 demo";
const WIDTH: u32 = 1000;
const HEIGHT: u32 = 800;
const FRAMES_PER_SECOND: u32 = 75;
const SPEED: i16 = 7;

fn event_is_termination(event: &Event) -> bool {
    matches!(
        event,
        Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape | Keycode::Q), .. }
    )
}

#[derive(Clone, Debug, Default)]
struct PlayerInput {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn register_event(&mut self, event: &Event) {
    //     let key = match event {
    //         Event::KeyDown { keycode: Some(key), .. } => key,
    //         _ => return,
    //     };

    //     match key {
    //         Keycode::W => self.up = true,
    //         Keycode::A => self.left = true,
    //         Keycode::S => self.down = true,
    //         Keycode::D => self.right = true,
    //         _ => {}
    //     }
    // }

    pub fn from_keyboard_state(keyboard_state: KeyboardState) -> Self {
        let mut new = Self::new();
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            new.up = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            new.left = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            new.down = true;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            new.right = true;
        }
        new
    }
}

fn process_events(event_pump: &mut EventPump) -> (bool, PlayerInput) {
    let mut should_exit = false;

    for event in event_pump.poll_iter() {
        should_exit |= event_is_termination(&event);
    }
    let player_input = PlayerInput::from_keyboard_state(event_pump.keyboard_state());

    (should_exit, player_input)
}

fn start_rendering() -> (Sdl, Canvas<Window>, FPSManager) {
    let context = sdl2::init().unwrap();
    let video_subsystem = context.video().unwrap();
    let window =
        video_subsystem.window(WINDOW_NAME, WIDTH, HEIGHT).position_centered().build().unwrap();
    let canvas = window.into_canvas().build().unwrap();

    let mut fps_manager = FPSManager::new();
    fps_manager.set_framerate(FRAMES_PER_SECOND).unwrap();
    (context, canvas, fps_manager)
}

pub fn main() {
    let (context, mut canvas, mut fps_manager) = start_rendering();

    let mut event_pump = context.event_pump().unwrap();

    let (x, y) = (WIDTH / 2, HEIGHT / 2);
    let (mut x, mut y) = (x as i16, y as i16);

    let mut old_pos = vec![(x, y); 150];

    'running: loop {
        // Setting the color of the whole canvas
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        canvas.filled_circle(x, y, 34, Color::RED).unwrap();

        for i in (9..150).step_by(10) {
            let (x, y) = old_pos[i];
            canvas.circle(x, y, 30, Color::RED).unwrap();
        }

        canvas.present();

        let (should_exit, player_input) = process_events(&mut event_pump);

        if player_input.up {
            y -= SPEED;
        }
        if player_input.down {
            y += SPEED;
        }
        if player_input.left {
            x -= SPEED;
        }
        if player_input.right {
            x += SPEED;
        }
        old_pos.insert(0, (x, y));
        old_pos.remove(150);

        if should_exit {
            break 'running;
        }

        fps_manager.delay();
    }
}
