use crate::bus::Bus;

pub struct PPU<'a> {
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub bus: &'a mut Bus,
}

impl<'a> PPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        PPU {
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 256],
            bus: bus,
        }
    }
}
