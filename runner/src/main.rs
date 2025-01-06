use chip8_core::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};
use std::{env, fs::File, io::Read};

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: usize = 10;

fn main() -> Result<(), String> {
    let game_name = env::args()
        .nth(1)
        .expect("Invalid arguments. Please use 'cargo run ./games/GAME_NAME'");

    // SDL setup
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("CHIP-8 EMULATOR", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .expect("Unable to create SDL window");

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("Unable to create SDL canvas");

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let mut chip_8 = Emu::default();

    // std::fs::read_to_string can corrupt the data because String expects UTF-8 format
    let mut rom = File::open(&game_name)
        .expect("Unable to open the specified file");

    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)
        .expect("Unable to read the game ROM");

    chip_8.load_game(&buffer);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'gameloop,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip_8.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        chip_8.keypress(k, false);
                    }
                }
                _ => (),
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip_8.tick();
        }

        chip_8.tick_timers();
        draw_screen(&chip_8, &mut canvas);
    }

    Ok(())
}

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    // Clear canvas as black
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();

    // Now se draw color to white, iterate through each point and see if it should be drawn
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            // Translate 1D array into 2D array
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            // Draw a rectangle at (x,y), scaled up by our SCALE value
            let rect = Rect::new((x * SCALE) as i32, (y * SCALE) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap_or_else(|err| panic!("{err}"));
        }
    }

    canvas.present();
}

// Keyboard to Chip-8 Key Mapping
// +---+---+---+---+       +---+---+---+---+
// | 1 | 2 | 3 | 4 |       | 1 | 2 | 3 | C |
// +---+---+---+---+       +---+---+---+---+
// | Q | W | E | R |       | 4 | 5 | 6 | D |
// +---+---+---+---+  -->  +---+---+---+---+
// | A | S | D | F |       | 7 | 8 | 9 | E |
// +---+---+---+---+       +---+---+---+---+
// | Z | X | C | V |       | A | 0 | B | F |
// +---+---+---+---+       +---+---+---+---+
fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
