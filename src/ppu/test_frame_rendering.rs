use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::full_screen_rendering::SCREEN_WIDTH_TILE;
use crate::ppu::render_sdl::screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ppu::render_sdl::Frame;
use crate::prelude::Mirroring;
use crate::{bus_mut, generate_ppu};

#[test]
fn background_drawing() {
    // this tests the drawing of the background
    // the result frame should have one tile that kinda draws the symbol for 1/2
    generate_ppu!(ppu);
    bus_mut!(ppu).cartridge.screen_mirroring = Mirroring::Horizontal;

    { // set tile 7 to be our tile
         // this has its own scope to be collapsable in the IDE
         // Bit Planes            Pixel Pattern
         // $0xx0=$41  01000001
         // $0xx1=$C2  11000010
         // $0xx2=$44  01000100
         // $0xx3=$48  01001000
         // $0xx4=$10  00010000
         // $0xx5=$20  00100000         .1.....3
         // $0xx6=$40  01000000         11....3.
         // $0xx7=$80  10000000  =====  .1...3..
         //     .1..3...
         // $0xx8=$01  00000001  =====  ...3.22.
         // $0xx9=$02  00000010         ..3....2
         // $0xxA=$04  00000100         .3....2.
         // $0xxB=$08  00001000         3....222
         // $0xxC=$16  00010110
         // $0xxD=$21  00100001
         // $0xxE=$42  01000010
         // $0xxF=$87  10000111
    }
    let our_tile = [
        0x41_u8, 0xC2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21,
        0x42, 0x87,
    ];
    bus_mut!(ppu).cartridge.chr_rom = vec![0; 0x2000];
    bus_mut!(ppu).cartridge.chr_rom[7 * 16..8 * 16].copy_from_slice(&our_tile);

    // tells the tile in row 4 col 9 to be our tile
    bus_mut!(ppu).ppu_memory.vram[4 * SCREEN_WIDTH_TILE + 9] = 7;

    // now color this tile using a different palette
    bus_mut!(ppu).ppu_memory.vram[960 + 2 + 8] = 0x03; // mask of our tile meta tile
    bus_mut!(ppu).ppu_memory.palette_table[13] = 3;
    bus_mut!(ppu).ppu_memory.palette_table[14] = 19;
    bus_mut!(ppu).ppu_memory.palette_table[15] = 24;

    let mut frame = Frame::new();
    ppu.copy_background_to_frame(&mut frame);
    // $0xx5=$20  00100000         .1.....3
    // $0xx6=$40  01000000         11....3.
    // $0xx7=$80  10000000  =====  .1...3..
    //                             .1..3...
    // $0xx8=$01  00000001  =====  ...3.22.
    // $0xx9=$02  00000010         ..3....2
    // $0xxA=$04  00000100         .3....2.
    // $0xxB=$08  00001000         3....222
    for x in 0..SCREEN_WIDTH as i32 {
        for y in 0..SCREEN_HEIGHT as i32 {
            let color = frame.get_pixel(x as usize, y as usize);
            assert_eq!(
                color,
                match (x - 9 * 8, y - 4 * 8) {
                    (0, 1) | (1, 1) | (1, 0) | (1, 2) | (1, 3) => SYSTEM_PALETTE[3],
                    (7, 7) | (6, 7) | (5, 7) | (6, 6) | (7, 5) | (5, 4) | (6, 4) =>
                        SYSTEM_PALETTE[19],
                    (0, 7) | (1, 6) | (2, 5) | (3, 4) | (4, 3) | (5, 2) | (6, 1) | (7, 0) =>
                        SYSTEM_PALETTE[24],
                    _ => SYSTEM_PALETTE[0],
                }
            )
        }
    }
}
