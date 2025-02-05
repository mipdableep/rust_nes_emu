#[macro_export]
macro_rules! generate_cpu {
    ($var: ident) => {
        let mut bus: crate::bus::Bus = crate::bus::Bus::new();
        let mut $var = crate::cpu::CPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_ppu {
    ($var: ident) => {
        let mut bus: crate::bus::Bus = crate::bus::Bus::new();
        let mut $var = crate::ppu::PPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_cpu_and_set_vertical_mirroring {
    ($var: ident) => {
        let mut bus: crate::bus::Bus = crate::bus::Bus::new();
        bus.cartridge.screen_mirroring = Mirroring::Vertical;
        let mut $var = crate::cpu::CPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_cpu_and_set_horizontal_mirroring {
    ($var: ident) => {
        let mut bus: crate::bus::Bus = crate::bus::Bus::new();
        bus.cartridge.screen_mirroring = Mirroring::Horizontal;
        let mut $var = crate::cpu::CPU::new(&mut bus);
    };
}

#[macro_export]
macro_rules! generate_texture_canvas_event_pump {
    ($texture: ident, $canvas: ident, $event_pump: ident) => {
        let screen_width = $crate::ppu::SCREEN_WIDTH;
        let screen_height = $crate::ppu::SCREEN_HEIGHT;

        const SCREEN_FACTOR: usize = 2;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(
                "Test Frame",
                (screen_width * SCREEN_FACTOR) as u32,
                (screen_height * SCREEN_FACTOR) as u32,
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
                sdl2::pixels::PixelFormatEnum::RGB24,
                screen_width as u32,
                screen_height as u32,
            )
            .unwrap();
    };
}
