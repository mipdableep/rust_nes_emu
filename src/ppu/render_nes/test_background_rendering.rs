use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::frame::Frame;
use crate::ppu::render_nes::assert_screen_state;
use crate::ppu::render_nes::ppu_render_constants::SCREEN_WIDTH_TILE;
use crate::ppu::{SCANLINES_PER_FRAME, SCANLINE_LENGTH_PIXELS};
use crate::prelude::Mirroring;
use crate::{bus_mut, generate_ppu, generate_texture_canvas_event_pump};
use serial_test::serial;

fn get_half_tile() -> [u8; 16] {
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

    [
        0x41_u8, 0xC2, 0x44, 0x48, 0x10, 0x20, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x16, 0x21,
        0x42, 0x87,
    ]
}

fn get_color_index_for_half_tile(x: i32, y: i32) -> usize {
    // gets coordinates of a point, and return the expected color index according to
    // $0xx5=$20  00100000         .1.....3
    // $0xx6=$40  01000000         11....3.
    // $0xx7=$80  10000000  =====  .1...3..
    //                             .1..3...
    // $0xx8=$01  00000001  =====  ...3.22.
    // $0xx9=$02  00000010         ..3....2
    // $0xxA=$04  00000100         .3....2.
    // $0xxB=$08  00001000         3....222
    match (x, y) {
        (0, 1) | (1, 1) | (1, 0) | (1, 2) | (1, 3) => 1,
        (7, 7) | (6, 7) | (5, 7) | (6, 6) | (7, 5) | (5, 4) | (6, 4) => 2,
        (0, 7) | (1, 6) | (2, 5) | (3, 4) | (4, 3) | (5, 2) | (6, 1) | (7, 0) => 3,
        _ => 0,
    }
}

#[test]
#[serial]
fn background_drawing() {
    // this tests the drawing of the background
    // the result frame should have one tile that kinda draws the symbol for 1/2
    generate_ppu!(ppu);
    generate_texture_canvas_event_pump!(texture, canvas, event_pump);

    bus_mut!(ppu).cartridge.screen_mirroring = Mirroring::Horizontal;

    let our_tile = get_half_tile();
    bus_mut!(ppu).cartridge.chr_rom = vec![0; 0x2000];
    bus_mut!(ppu).cartridge.chr_rom[7 * 16..8 * 16].copy_from_slice(&our_tile);

    // tells the tile in row 4 col 9 to be our tile
    bus_mut!(ppu).ppu_memory.vram[4 * SCREEN_WIDTH_TILE + 9] = 7;

    // now color this tile using a different palette
    bus_mut!(ppu).ppu_memory.vram[960 + 2 + 8] = 0x03; // mask of our tile meta tile
    bus_mut!(ppu).ppu_memory.palette_table[13] = 3;
    bus_mut!(ppu).ppu_memory.palette_table[14] = 19;
    bus_mut!(ppu).ppu_memory.palette_table[15] = 24;

    let wanted_palette = [
        SYSTEM_PALETTE[0],
        SYSTEM_PALETTE[3],
        SYSTEM_PALETTE[19],
        SYSTEM_PALETTE[24],
    ];

    let mut frame = Frame::new();
    for _ in 0..SCANLINE_LENGTH_PIXELS {
        for _ in 0..SCANLINES_PER_FRAME {
            ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
        }
    }

    assert_screen_state!(frame, 72, 32, get_color_index_for_half_tile, wanted_palette);
}

#[test]
#[serial]
fn test_scrolling() {
    // this tests the scrolling
    generate_ppu!(ppu);
    generate_texture_canvas_event_pump!(texture, canvas, event_pump);

    bus_mut!(ppu).cartridge.screen_mirroring = Mirroring::Vertical;

    let our_tile = get_half_tile();
    bus_mut!(ppu).cartridge.chr_rom = vec![0; 0x2000];
    // again, tile 7 is our tile
    bus_mut!(ppu).cartridge.chr_rom[7 * 16..8 * 16].copy_from_slice(&our_tile);

    // tells the tile in row 17 col 23 to be our tile, in the bottom right nametable
    // due to the mirroring, we should put it in the nametable with index 1 (top right)
    bus_mut!(ppu).ppu_memory.vram[17 * SCREEN_WIDTH_TILE + 23 + 1024] = 7;

    // now color this tile using a different palette
    // we are in meta tile (4*4 tiles) of row 4 col 5
    // each row has 8 meta tiles
    bus_mut!(ppu).ppu_memory.vram[1024 + 960 + 4 * 8 + 5] = 0x0C; // we are part of the top right group
    bus_mut!(ppu).ppu_memory.palette_table[13] = 3;
    bus_mut!(ppu).ppu_memory.palette_table[14] = 19;
    bus_mut!(ppu).ppu_memory.palette_table[15] = 24;

    let wanted_palette = [
        SYSTEM_PALETTE[0],
        SYSTEM_PALETTE[3],
        SYSTEM_PALETTE[19],
        SYSTEM_PALETTE[24],
    ];

    // we want to add a shift
    // let's say 350 x shift and 333 y shift
    // remember - we should set the nametable bytes in the control if we want to add more than 256 shift
    bus_mut!(ppu).ppu_registers.write_to_scroll(94); // 350 - 256
    bus_mut!(ppu).ppu_registers.write_to_scroll(93); // 333 - 240

    bus_mut!(ppu)
        .ppu_registers
        .control_register
        .write_byte(0x03); // use the second nametable both in x and in y

    let mut frame = Frame::new();

    for _ in 0..2 {
        // the y scroll only kicks into action after 1 full cycle
        for _ in 0..SCANLINE_LENGTH_PIXELS {
            for _ in 0..SCANLINES_PER_FRAME {
                ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
            }
        }
    }

    // before any scroll, our tile was at coordinates x = 23*8 + 256, y = 17*8 + 256
    // when we looked at it as a tile in the bottom right nametable (mirroring!)
    // so now it should be in y = 17*8 + 240 - 333 = 43
    // and in x = 23 * 8 + 256 - 350 = 90
    assert_screen_state!(frame, 90, 43, get_color_index_for_half_tile, wanted_palette);

    // as an extra cherry on top (and due to a previous bug): we check that adding y nametable
    // in vertical mirroring does not change the result
    bus_mut!(ppu)
        .ppu_registers
        .control_register
        .write_byte(0x01); // use the second nametable both in x and in y

    for _ in 0..2 {
        for _ in 0..SCANLINE_LENGTH_PIXELS {
            for _ in 0..SCANLINES_PER_FRAME {
                ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
            }
        }
    }
    assert_screen_state!(frame, 90, 43, get_color_index_for_half_tile, wanted_palette);
}
