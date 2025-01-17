mod render_background;
mod render_sprites;

use super::PPU;

pub mod ppu_render_constants {
    pub const SCREEN_WIDTH: usize = 256;
    pub const SCREEN_HEIGHT: usize = 240;

    pub const TILE_WIDTH: usize = 8;
    pub const TILE_HEIGHT: usize = 8;

    pub const SCREEN_WIDTH_TILE: usize = SCREEN_WIDTH / TILE_WIDTH;
    pub const SCREEN_HEIGHT_TILE: usize = SCREEN_HEIGHT / TILE_HEIGHT;

    pub const SCREEN_SIZE_TILE: usize = SCREEN_WIDTH_TILE * SCREEN_HEIGHT_TILE;

    pub const COPY_VERT_V_DOT_START: usize = 280;
    pub const COPY_VERT_V_DOT_END: usize = 304;
    pub const DOT_TO_START_FETCH_NEXT_LINE_TILES: usize = 321;
    pub const FIRST_TILE_FETCH_DOT: usize = 328;

    pub const SCANLINE_LENGTH_PIXELS: usize = 341;

    pub const NMI_SCANLINE: usize = 241;
    pub const SCANLINES_PER_FRAME: usize = 262;
}
