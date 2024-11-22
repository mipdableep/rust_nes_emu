use std::{thread::sleep, time};

use nes_emulator::ppu::render_sdl::{
    screen_rendering_constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    update_texture_from_frame, Frame,
};
use nes_emulator::ppu::PPU;
use nes_emulator::prelude::{Bus, CPU};
use nes_emulator::{generate_cpu, generate_texture_canvas_event_pump};

fn main() {
    let mut bus: Bus = Bus::new();
    let bytes = std::fs::read("./roms/pacman.nes").unwrap();
    bus.cartridge.load_from_dump(&bytes);

    let mut bus_ref = &mut bus;

    let mut cpu = CPU::new(bus_ref);
    cpu.program_counter = cpu.read_memory_2_bytes(0xFFFC); //TODO: create cpu.reset or something?
    bus_ref = cpu.bus.take().unwrap();

    generate_texture_canvas_event_pump!(texture, canvas, event_pump);
    let mut frame = Frame::new();
    let mut ppu = PPU::new(bus_ref);

    loop {
        bus_ref = ppu.bus.take().unwrap();
        cpu.bus = Some(bus_ref);
        cpu.run_one_cycle();

        bus_ref = cpu.bus.take().unwrap();
        ppu.bus = Some(bus_ref);
        ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas);
        ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas);
        ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas);

        let sleep_time = time::Duration::from_secs_f64(0.001);
        // sleep(sleep_time);

        let event = match event_pump.poll_event() {
            Some(i) => i,
            None => continue,
        };
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => {}
        }
    }
}
