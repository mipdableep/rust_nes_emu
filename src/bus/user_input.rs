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

                Event::KeyDown {
                    keycode: Some(kc), ..
                } => match kc {
                    val if val == self.config.a => self.p1_controller.set_a(),
                    val if val == self.config.b => self.p1_controller.set_b(),
                    val if val == self.config.start => self.p1_controller.set_start(),
                    val if val == self.config.select => self.p1_controller.set_select(),
                    val if val == self.config.up => self.p1_controller.set_up(),
                    val if val == self.config.down => self.p1_controller.set_down(),
                    val if val == self.config.left => self.p1_controller.set_left(),
                    val if val == self.config.right => self.p1_controller.set_right(),
                    _ => {}
                },

                Event::KeyUp {
                    keycode: Some(kc), ..
                } => match kc {
                    val if val == self.config.a => self.p1_controller.unset_a(),
                    val if val == self.config.b => self.p1_controller.unset_b(),
                    val if val == self.config.start => self.p1_controller.unset_start(),
                    val if val == self.config.select => self.p1_controller.unset_select(),
                    val if val == self.config.up => self.p1_controller.unset_up(),
                    val if val == self.config.down => self.p1_controller.unset_down(),
                    val if val == self.config.left => self.p1_controller.unset_left(),
                    val if val == self.config.right => self.p1_controller.unset_right(),
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
