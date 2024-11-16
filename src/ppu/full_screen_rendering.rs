use super::PPU;
use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::render_sdl::{update_texture_from_frame, Frame};
use sdl2::render::{Texture, WindowCanvas};

impl<'bus> PPU<'bus> {
    fn copy_background_to_frame(&self, frame: &mut Frame) {
        for i in 0..0x03c0 {
            // just for now, lets use the first nametable
            let tile = self.bus.ppu_memory.vram[i] as u16;
            let tile_x = i % 32;
            let tile_y = i / 32;
            let tile = &self.bus.cartridge.chr_rom
                [(0x1000 + tile * 16) as usize..=(0x1000 + tile * 16 + 15) as usize];

            for y in 0..=7 {
                let mut upper = tile[y];
                let mut lower = tile[y + 8];

                for x in (0..=7).rev() {
                    let value = (1 & upper) << 1 | (1 & lower);
                    upper = upper >> 1;
                    lower = lower >> 1;
                    let rgb = match value {
                        0 => SYSTEM_PALETTE[0x01],
                        1 => SYSTEM_PALETTE[0x23],
                        2 => SYSTEM_PALETTE[0x27],
                        3 => SYSTEM_PALETTE[0x30],
                        _ => panic!("can't be"),
                    };
                    frame.set_pixel(tile_x * 8 + x, tile_y * 8 + y, rgb)
                }
            }
        }
    }

    pub fn render_full_screen_background(
        &self,
        texture: &mut Texture,
        frame: &mut Frame,
        canvas: &mut WindowCanvas,
    ) {
        self.copy_background_to_frame(frame);
        update_texture_from_frame(texture, frame, canvas);
        canvas.present();
    }
}
