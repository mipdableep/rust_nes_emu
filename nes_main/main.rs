use std::{thread::sleep, time};

use nes_emulator::{generate_cpu, generate_texture_canvas_event_pump};

use nes_emulator::ppu::render_sdl::{
    screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    update_texture_from_frame, Frame,
};

fn main() {
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

    generate_cpu!(cpu);

    generate_texture_canvas_event_pump!(texture, canvas, event_pump);

    update_texture_from_frame(&mut texture, &mut frame, &mut canvas);
    loop {
        canvas.present();

        //handle events
        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. } => std::process::exit(0),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }

        let sleep_time = time::Duration::from_secs_f64(0.2);
        sleep(sleep_time);
    }
}
