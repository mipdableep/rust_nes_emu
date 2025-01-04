use crate::ppu::PPU;
use crate::{bus_mut};
use sdl2::EventPump;

impl<'bus> PPU<'bus> {
    pub(crate) fn handle_user_input(&mut self, event_pump: &mut EventPump) {
        bus_mut!(self).handle_user_input(event_pump);
    }
}
