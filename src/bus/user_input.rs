use crate::bus::Bus;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

impl Bus {
    pub fn handle_user_input(&mut self, event_pump: &mut EventPump) {
        while let Some(event) = event_pump.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::A) => self.p1_controller.set_a(),
                    Some(Keycode::B) => self.p1_controller.set_b(),
                    Some(Keycode::P) => self.p1_controller.set_start(),
                    Some(Keycode::O) => self.p1_controller.set_select(),
                    Some(Keycode::UP) => self.p1_controller.set_up(),
                    Some(Keycode::DOWN) => self.p1_controller.set_down(),
                    Some(Keycode::LEFT) => self.p1_controller.set_left(),
                    Some(Keycode::RIGHT) => self.p1_controller.set_right(),
                    _ => {}
                },

                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::A) => self.p1_controller.unset_a(),
                    Some(Keycode::B) => self.p1_controller.unset_b(),
                    Some(Keycode::P) => self.p1_controller.unset_start(),
                    Some(Keycode::O) => self.p1_controller.unset_select(),
                    Some(Keycode::UP) => self.p1_controller.unset_up(),
                    Some(Keycode::DOWN) => self.p1_controller.unset_down(),
                    Some(Keycode::LEFT) => self.p1_controller.unset_left(),
                    Some(Keycode::RIGHT) => self.p1_controller.unset_right(),
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
