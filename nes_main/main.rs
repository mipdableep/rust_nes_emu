// use std::{thread::sleep, time};

use nes_emulator::generate_texture_canvas_event_pump;
use nes_emulator::ppu::render_sdl::Frame;
use nes_emulator::ppu::PPU;
use nes_emulator::prelude::{Bus, CPU};

fn main() {
    let mut bus: Bus = Bus::new();
    let bytes = std::fs::read("./roms/mario.nes").unwrap();
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
        ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
        ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);
        ppu.run_one_ppu_cycle(&mut texture, &mut frame, &mut canvas, &mut event_pump);

        // let sleep_time = time::Duration::from_secs_f64(0.001);
        // sleep(sleep_time);
    }
}
