use crate::ppu::colors_palette::SYSTEM_PALETTE;
use crate::ppu::frame::Frame;
use crate::ppu::render_nes::assert_screen_state;
use crate::ppu::render_nes::ppu_render_constants::SCREEN_WIDTH_TILE;
use crate::ppu::{PPU, SCANLINES_PER_FRAME, SCANLINE_LENGTH_PIXELS};
use crate::prelude::Mirroring;
use crate::{bus_mut, generate_ppu, generate_texture_canvas_event_pump};
use serial_test::serial;

fn prepare_diamond_sprite(ppu: &mut PPU, tile_number: usize, bank_start: usize) {
    { // set sprite 5 to be our tile
         // this has its own scope to be collapsable in the IDE
         // Bit Planes            Pixel Pattern
         // $0xx0=$41  10000000
         // $0xx1=$C2  11000000
         // $0xx2=$44  11100000
         // $0xx3=$48  11110000
         // $0xx4=$10  11111000         12222222
         // $0xx5=$20  11111100         11222222
         // $0xx6=$40  11111110         11122222
         // $0xx7=$80  01111111  =====  11112222
         //     .1..3...
         // $0xx8=$01  01111111  =====  11111222
         // $0xx9=$02  00111111         11111122
         // $0xxA=$04  00011111         11111112
         // $0xxB=$08  00001111         01111111
         // $0xxC=$16  00000111
         // $0xxD=$21  00000011
         // $0xxE=$42  00000001
         // $0xxF=$87  00000000
    }
    let our_tile = [
        0x80_u8, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC, 0xFE, 0x7F, !0x80_u8, !0xC0, !0xE0, !0xF0, !0xF8,
        !0xFC, !0xFE, 0,
    ];
    bus_mut!(ppu).cartridge.chr_rom = vec![0; 0x2000];
    bus_mut!(ppu).cartridge.chr_rom
        [bank_start + tile_number * 16..bank_start + (tile_number + 1) * 16]
        .copy_from_slice(&our_tile);
}

fn set_diamond_sprites(ppu: &mut PPU, center_x: u8, center_y: u8, is_background: bool) {
    let top_right_attr = prepare_attribute_byte(false, false, is_background, 1);
    set_sprite(ppu, 12, 5, center_x, center_y - 8, top_right_attr);

    let top_left_attr = prepare_attribute_byte(true, false, is_background, 1);
    set_sprite(ppu, 17, 5, center_x - 8, center_y - 8, top_left_attr);

    let bottom_left_attr = prepare_attribute_byte(true, true, is_background, 1);
    set_sprite(ppu, 26, 5, center_x - 8, center_y, bottom_left_attr);

    let bottom_right_attr = prepare_attribute_byte(false, true, is_background, 1);
    set_sprite(ppu, 63, 5, center_x, center_y, bottom_right_attr);
}

fn get_color_index_for_diamond_sprite(x: i32, y: i32) -> usize {
    let x_offset_from_center = (x as f32 + 0.5).abs();
    // per multiple sources, including https://forums.nesdev.org/viewtopic.php?t=15890
    // there is a 1 line offset in the sprite line
    let y_offset_from_center = (-1.0 + y as f32 + 0.5).abs();

    match x_offset_from_center < 8.0 && y_offset_from_center < 8.0 {
        true => {
            match x_offset_from_center + y_offset_from_center {
                i if i < 1.5 => {
                    // the center has no color
                    // the distance is 0.5 + 0.5, but floating point
                    0
                }
                i if i < 8.5 => {
                    // in the diamond, use color 1
                    // this has distance 0.5 + 7.5, but again floating point scare me
                    1
                }
                _ => {
                    // color 2 of the palette
                    2
                }
            }
        }
        false => {
            // outside this square we should not have a sprite
            0
        }
    }
}

fn prepare_attribute_byte(flip_h: bool, flip_v: bool, is_bkg: bool, palette: u8) -> u8 {
    // 76543210
    // ||||||||
    // ||||||++- Palette (4 to 7) of sprite
    // |||+++--- Unimplemented (read 0)
    // ||+------ Priority (0: in front of background; 1: behind background)
    // |+------- Flip sprite horizontally
    // +-------- Flip sprite vertically

    ((flip_v as u8) << 7) + ((flip_h as u8) << 6) + ((is_bkg as u8) << 5) + palette
}

fn set_sprite(ppu: &mut PPU, sprite_number: usize, tile_number: u8, x: u8, y: u8, attr: u8) {
    bus_mut!(ppu).ppu_memory.oam_data[4 * sprite_number] = y;
    bus_mut!(ppu).ppu_memory.oam_data[4 * sprite_number + 1] = tile_number;
    bus_mut!(ppu).ppu_memory.oam_data[4 * sprite_number + 2] = attr;
    bus_mut!(ppu).ppu_memory.oam_data[4 * sprite_number + 3] = x;
}

#[test]
#[serial]
fn sprites_rendering() {
    // this tests the drawing of the character sprites
    // the result frame should have one tile that kinda looks like a diamond
    generate_ppu!(ppu);
    generate_texture_canvas_event_pump!(texture, canvas, event_pump);

    bus_mut!(ppu).cartridge.screen_mirroring = Mirroring::Horizontal;

    // set the sprite bank to 0x1000
    bus_mut!(ppu)
        .ppu_registers
        .control_register
        .write_byte(0x08);

    // put our sprite as tile 5
    prepare_diamond_sprite(&mut ppu, 5, 0x1000);

    // prepare the palette
    bus_mut!(ppu).ppu_memory.palette_table[21] = 12;
    bus_mut!(ppu).ppu_memory.palette_table[22] = 27;
    bus_mut!(ppu).ppu_memory.palette_table[23] = 20;

    let wanted_palette = [
        SYSTEM_PALETTE[0],
        SYSTEM_PALETTE[12],
        SYSTEM_PALETTE[27],
        SYSTEM_PALETTE[20],
    ];

    // we will now create 4 sprites of this tile, rotated to make a diamond shape
    // the center will be at 121.5, 69.5
    set_diamond_sprites(&mut ppu, 122, 70, false);

    let mut frame = Frame::new();
    for _ in 0..SCANLINE_LENGTH_PIXELS {
        for _ in 0..SCANLINES_PER_FRAME {
            ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
        }
    }

    // in the end, we should have a pretty diamond shape
    // with the center at 121.5, 69.5
    assert_screen_state!(
        frame,
        122,
        70,
        get_color_index_for_diamond_sprite,
        wanted_palette
    );
}

#[test]
#[serial]
fn sprites_background() {
    // a test to check we handle the background process correctly
    generate_ppu!(ppu);
    generate_texture_canvas_event_pump!(texture, canvas, event_pump);

    bus_mut!(ppu).cartridge.screen_mirroring = Mirroring::Horizontal;

    // set the sprite bank to 0x1000
    bus_mut!(ppu)
        .ppu_registers
        .control_register
        .write_byte(0x08);

    // put our sprite as tile 5
    prepare_diamond_sprite(&mut ppu, 5, 0x1000);

    // prepare the palette
    bus_mut!(ppu).ppu_memory.palette_table[21] = 12;
    bus_mut!(ppu).ppu_memory.palette_table[22] = 27;
    bus_mut!(ppu).ppu_memory.palette_table[23] = 20;

    // we will now create 4 sprites of this tile, rotated to make a diamond shape
    // the center will be at 121.5, 69.5
    set_diamond_sprites(&mut ppu, 122, 70, true);

    // we now give a background tile that collides with the sprites
    // the background tile is like a window: has 1 color in the edges, and empty inside
    let our_tile = [
        0xFF_u8, 0x81, 0x81, 0x81, 0x81, 0x81, 0x81, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    bus_mut!(ppu).cartridge.chr_rom[7 * 16..8 * 16].copy_from_slice(&our_tile);

    // tells the tile in row 8 col 15 to be our tile
    bus_mut!(ppu).ppu_memory.vram[8 * SCREEN_WIDTH_TILE + 15] = 7;

    // now color this tile using a different palette
    bus_mut!(ppu).ppu_memory.vram[960 + 2 * 8 + 3] = 0x0C; // mask of our tile meta tile
    bus_mut!(ppu).ppu_memory.palette_table[13] = 35;

    let mut frame = Frame::new();
    for _ in 0..SCANLINE_LENGTH_PIXELS {
        for _ in 0..SCANLINES_PER_FRAME {
            ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
        }
    }

    fn get_color_index_for_diamond_sprite_with_background(x: i32, y: i32) -> usize {
        // if we are in the top/bottom rows
        if ((y == -6) || (y == 1)) && (-2 <= x && x < 6) {
            return 3;
        }
        // if we are in the right/left columns
        if ((x == -2) || (x == 5)) && (-6 <= y && y < 2) {
            return 3;
        }
        // we are in the diamond
        get_color_index_for_diamond_sprite(x, y)
    }

    // we cheat a little bit - we use color 3 as the color for the background (e.g. the window)
    // we can do it since the diamond does not use all the 3 colors
    let wanted_palette = [
        SYSTEM_PALETTE[0],
        SYSTEM_PALETTE[12],
        SYSTEM_PALETTE[27],
        SYSTEM_PALETTE[35],
    ];

    assert_screen_state!(
        frame,
        122,
        70,
        get_color_index_for_diamond_sprite_with_background,
        wanted_palette
    );
}
