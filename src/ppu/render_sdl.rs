use screen_rendering_constants::*;
use sdl2::render::{Texture, WindowCanvas};

pub(crate) mod screen_rendering_constants {
    pub const SCREEN_WIDTH: usize = 240;
    pub const SCREEN_HEIGHT: usize = 260;
    pub const SCREEN_FACTOR: usize = 10;
    pub const SCREEN_WIDTH: usize = 256;
    pub const SCREEN_HEIGHT: usize = 240;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    pub screen_state: [u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
}

impl Frame {
    pub fn new() -> Self {
        Self {
            screen_state: [0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, (r, g, b): (u8, u8, u8)) {
        if x >= SCREEN_WIDTH {
            panic!("Trying to set pixel {x}, which is more than width {SCREEN_WIDTH}")
        }
        if y >= SCREEN_HEIGHT {
            panic!("Trying to set pixel {y}, which is more than width {SCREEN_HEIGHT}")
        }
        self.screen_state[3 * (x + y * SCREEN_WIDTH)] = r;
        self.screen_state[3 * (x + y * SCREEN_WIDTH) + 1] = g;
        self.screen_state[3 * (x + y * SCREEN_WIDTH) + 2] = b;
    }
}

pub fn update_texture_from_frame(texture: &mut Texture, frame: &Frame, canvas: &mut WindowCanvas) {
    texture
        .update(None, &frame.screen_state, SCREEN_WIDTH * 3)
        .unwrap();
    canvas.copy(texture, None, None).unwrap();
}

#[macro_export]
macro_rules! generate_texture_and_canvas {
    ($texture: ident, $canvas: ident, $event_pump: ident) => {
        use crate::ppu::render_sdl::screen_rendering_constants::*;
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::pixels::PixelFormatEnum;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Test Frame",
                (SCREEN_WIDTH * SCREEN_FACTOR) as u32,
                (SCREEN_HEIGHT * SCREEN_FACTOR) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut $event_pump = sdl_context.event_pump().unwrap();

        let mut $canvas = window.into_canvas().present_vsync().build().unwrap();
        $canvas
            .set_scale(SCREEN_FACTOR as f32, SCREEN_FACTOR as f32)
            .unwrap();

        // telling sdl that the texture is wxh and rgb, so it is wxhx3
        let creator = $canvas.texture_creator();
        let mut $texture = creator
            .create_texture_target(
                PixelFormatEnum::RGB24,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();
    };
}

fn render_sdl() {
    let current_frame = [0_u8; SCREEN_WIDTH * SCREEN_HEIGHT * 3];
}
