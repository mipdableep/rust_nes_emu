#![allow(unused)]
pub use crate::{
    bus::{cartridge::Mirroring, Bus},
    cpu::CPU,
};

#[macro_export]
macro_rules! generate_cpu {
    ($var: ident) => {
        use $crate::{bus::Bus, cpu::CPU};
        let mut bus: Bus = Bus::new();
        let mut $var = CPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_cpu_and_set_vertical_mirroring {
    ($var: ident) => {
        use $crate::{bus::Bus, cpu::CPU};
        let mut bus: Bus = Bus::new();
        bus.cartridge.screen_mirroring = Mirroring::Vertical;
        let mut $var = CPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_cpu_and_set_horizontal_mirroring {
    ($var: ident) => {
        use $crate::{bus::Bus, cpu::CPU};
        let mut bus: Bus = Bus::new();
        bus.cartridge.screen_mirroring = Mirroring::Horizontal;
        let mut $var = CPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_texture_canvas_event_pump {
    ($texture: ident, $canvas: ident, $event_pump: ident) => {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::pixels::PixelFormatEnum;
        use $crate::ppu::render_sdl::screen_rendering_constants::*;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Test Frame",
                (SCREEN_WIDTH * SCREEN_FACTOR) as u32,
                (SCREEN_HEIGHT * SCREEN_FACTOR) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut $event_pump = sdl_context.event_pump().unwrap();

        let mut $canvas = window.into_canvas().present_vsync().build().unwrap();
        $canvas
            .set_scale(SCREEN_FACTOR as f32, SCREEN_FACTOR as f32)
            .unwrap();

        // telling sdl that the texture is wxh and rgb, so it is wxhx3
        let creator = $canvas.texture_creator();
        let mut $texture = creator
            .create_texture_target(
                PixelFormatEnum::RGB24,
                SCREEN_WIDTH as u32,
                SCREEN_HEIGHT as u32,
            )
            .unwrap();
    };
}
