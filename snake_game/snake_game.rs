use nes_emulator::cpu::CPU;
use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::EventPump;
use std::thread::sleep;

const SCREEN_START: u16 = 0x0200;
const SCREEN_SIZE: usize = 32;
const SLEEP_TIME_NANOS: u128 = 5_000;
const SCREEN_FACTOR: f32 = 10.0;

struct SnakeGame<'a, 'sdl> {
    cpu: CPU,
    event_pump: EventPump,
    screen_state: [u8; 32 * 32 * 3],
    rng_gen: ThreadRng,
    texture: &'a mut Texture<'sdl>,
    canvas: &'a mut WindowCanvas,
    is_paused: bool,
}

impl<'a, 'sdl> SnakeGame<'a, 'sdl> {
    fn load_snake_game(&mut self) {
        let game_code: Vec<u8> = vec![
            0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9,
            0x02, 0x85, 0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85,
            0x12, 0xa9, 0x0f, 0x85, 0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60,
            0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe, 0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60,
            0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3, 0x06, 0x20, 0x19, 0x07, 0x20, 0x20,
            0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9, 0x77, 0xf0, 0x0d, 0xc9,
            0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60, 0xa9, 0x04,
            0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
            0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04,
            0x85, 0x02, 0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60,
            0x60, 0x20, 0x94, 0x06, 0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d,
            0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07, 0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60,
            0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06, 0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09,
            0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c, 0x35, 0x07, 0x60, 0xa6,
            0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02, 0x4a, 0xb0,
            0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
            0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28,
            0x60, 0xe6, 0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69,
            0x20, 0x85, 0x10, 0xb0, 0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c,
            0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29, 0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35,
            0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60, 0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10,
            0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea, 0xea, 0xca, 0xd0, 0xfb,
            0x60,
        ];
        for i in 0x0600..0x0600 + game_code.len() as u16 {
            self.cpu.write_memory(i, game_code[i as usize]);
        }
        self.cpu.program_counter = 0x600;
        self.cpu.write_memory(0xFFFC, 0x00);
        self.cpu.write_memory(0xFFFD, 0x06); // mem[0xFFFC] = 0x0600, little endian
    }

    fn handle_user_input(&mut self) {
        // the collect is to be able to not borrow self in the poll iter, maybe will change in the future
        for event in self.event_pump.poll_iter().collect::<Vec<Event>>() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    self.cpu.write_memory(0xff, 0x77);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    self.cpu.write_memory(0xff, 0x73);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    self.cpu.write_memory(0xff, 0x61);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.cpu.write_memory(0xff, 0x64);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    self.is_paused = !self.is_paused;
                    if self.is_paused {
                        self.draw_pause_symbol();
                        self.update_actual_texture();
                    }
                }
                _ => { /* do nothing */ }
            }
        }
    }

    fn run_snake(&mut self, debug: bool) {
        loop {
            let now = std::time::Instant::now();
            self.handle_user_input();
            if self.is_paused {
                let elapsed = now.elapsed();
                if elapsed.as_nanos() < SLEEP_TIME_NANOS {
                    std::thread::sleep(std::time::Duration::new(
                        0,
                        (SLEEP_TIME_NANOS - elapsed.as_nanos()) as u32,
                    ));
                }
                continue;
            }
            self.cpu.write_memory(0xfe, self.rng_gen.gen_range(1..16));
            if self.update_screen_state() {
                self.update_actual_texture();
            }

            let opcode = self.cpu.read_memory(self.cpu.program_counter);
            if debug {
                println!(
                    "pc {:}, opcode {:}, args {:} {:}",
                    self.cpu.program_counter,
                    opcode,
                    self.cpu.read_memory(self.cpu.program_counter + 1),
                    self.cpu.read_memory(self.cpu.program_counter + 2)
                );
            }
            if !self.cpu.massive_switch(opcode) {
                // game over
                self.is_paused = true;
                self.draw_pause_symbol(); // draw pause
                self.update_actual_texture();

                while self.is_paused && now.elapsed().as_secs() < 10 {
                    // give the user 10 sec to manually exit
                    self.handle_user_input();
                    sleep(std::time::Duration::new(0, SLEEP_TIME_NANOS as u32));
                }
                return;
            }
            let elapsed = now.elapsed();
            if elapsed.as_nanos() < SLEEP_TIME_NANOS {
                sleep(std::time::Duration::new(
                    0,
                    (SLEEP_TIME_NANOS - elapsed.as_nanos()) as u32,
                ));
            }
        }
    }

    fn update_screen_state(&mut self) -> bool {
        // this function returns true iff the screen should be updated
        let mut frame_idx = 0;
        let mut update = false;
        for i in SCREEN_START..SCREEN_START + 0x0400 {
            let color = byte_to_color(self.cpu.read_memory(i));
            let (r, g, b) = color.rgb();
            if r != self.screen_state[frame_idx]
                || g != self.screen_state[frame_idx + 1]
                || b != self.screen_state[frame_idx + 2]
            {
                update = true;
                self.screen_state[frame_idx] = r;
                self.screen_state[frame_idx + 1] = g;
                self.screen_state[frame_idx + 2] = b;
            }
            frame_idx += 3;
        }
        update
    }

    fn update_actual_texture(&mut self) {
        self.texture
            .update(None, &self.screen_state, 32 * 3)
            .unwrap();
        self.canvas.copy(self.texture, None, None).unwrap();
        self.canvas.present();
    }

    fn draw_pause_symbol(&mut self) {
        // draw two line (the pause symbol) on screen, like ||
        let line_start = SCREEN_SIZE / 4;
        let line_x = (SCREEN_SIZE * 2 / 5) * 3;
        for row in line_start..SCREEN_SIZE - line_start {
            self.screen_state[row * SCREEN_SIZE * 3 + line_x] = 128;
            self.screen_state[row * SCREEN_SIZE * 3 + line_x + 1] = 128;
            self.screen_state[row * SCREEN_SIZE * 3 + line_x + 2] = 128;

            self.screen_state[row * SCREEN_SIZE * 3 + SCREEN_SIZE * 3 - line_x] = 128;
            self.screen_state[row * SCREEN_SIZE * 3 + SCREEN_SIZE * 3 - line_x + 1] = 128;
            self.screen_state[row * SCREEN_SIZE * 3 + SCREEN_SIZE * 3 - line_x + 2] = 128;
        }
    }
}

fn byte_to_color(byte: u8) -> Color {
    match byte {
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn main() {
    // init sdl2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(
            "Snake game",
            (32.0 * SCREEN_FACTOR) as u32,
            (32.0 * SCREEN_FACTOR) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(SCREEN_FACTOR, SCREEN_FACTOR).unwrap();

    // telling sdl that the texture is 32x32 and rgb, so it is 32x32x3
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
        .unwrap();

    let current_frame = [0_u8; 32 * 32 * 3];
    let rng = thread_rng();
    let mut snake_game = SnakeGame {
        cpu: CPU::new(),
        event_pump,
        screen_state: current_frame,
        rng_gen: rng,
        texture: &mut texture,
        canvas: &mut canvas,
        is_paused: false,
    };
    snake_game.load_snake_game();
    snake_game.run_snake(false);
}
