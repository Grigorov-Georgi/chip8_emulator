use std::env;
use std::fs::File;
use std::io::Read;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;
use chip8_core::*;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

const TICKS_PER_FRAME: usize = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid arguments. Usage 'cargo run path/to/game'");
    }

    // SDL setup
    let sdl_context = sdl2::init().unwrap(); //TODO: handle properly
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP-8 EMULATOR", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip_8 = Emu::new();

    // std::fs::read_to_string can corrupt the data because String expects UTF-8 format
    let mut rom = File::open(&args[1]).expect("Unable to open file"); //TODO: handle properly
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();

    chip_8.load_game(&buffer);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { //TODO: check what this syntax do, its probably _ for struct
                    break 'gameloop
                }
                _ => ()
            }
        }

        for _ in 0..TICKS_PER_FRAME {
            chip_8.tick();
        }
        chip_8.tick_timers();
        draw_scren(&chip_8, &mut canvas);
    }
}

fn draw_scren(emu: &Emu, canvas: &mut Canvas<Window>) {
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
            canvas.fill_rect(rect).unwrap(); // TODO: handle properly
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
fn key2byn(key: Keycode) -> Option<usize> {
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
        _ => None
    }
}
