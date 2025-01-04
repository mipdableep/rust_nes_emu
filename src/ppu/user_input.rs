use crate::ppu::PPU;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

impl<'bus> PPU<'bus> {
    pub(crate) fn handle_user_input(&mut self, event_pump: &mut EventPump) {
        let event = match event_pump.poll_event() {
            Some(i) => i,
            None => return,
        };
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => {}
        }
    }
}
