use crate::ppu::render_sdl::{update_texture_from_frame, Frame};

pub mod bus;
pub mod cpu;
pub mod ppu;
pub mod prelude;

fn main() {
    generate_texture_and_canvas!(texture, canvas);
    let mut frame = Frame::new();
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            match (x < SCREEN_WIDTH / 2, y < SCREEN_HEIGHT / 2) {
                (true, true) => frame.set_pixel(x, y, (128, 0, 0)),
                (true, false) => frame.set_pixel(x, y, (0, 128, 0)),
                (false, true) => frame.set_pixel(x, y, (0, 0, 128)),
                (false, false) => frame.set_pixel(x, y, (255, 255, 255)),
            }
        }
    }
    update_texture_from_frame(&mut texture, &mut frame, &mut canvas);
    loop {
        canvas.present();
    }
}
