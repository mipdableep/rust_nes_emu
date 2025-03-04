// use std::{thread::sleep, time};

use nes_emulator::bus::Bus;
use nes_emulator::cpu::CPU;
use nes_emulator::generate_texture_canvas_event_pump;
use nes_emulator::ppu::frame::Frame;
use nes_emulator::ppu::PPU;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// optional sub command
    #[command(subcommand)]
    command: Option<Commands>,

    /// optional .nes file
    #[arg(short = 'f', long, value_name = "game_file")]
    #[arg(default_value = "./roms/mario.nes")]
    game_file: PathBuf,

    /// optional config file
    #[arg(short, long, value_name = "config_file")]
    config_file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// a visual configurator
    ConfigVisual {
        /// costum output path for the new config
        #[clap(long, short = 'o')]
        #[clap(default_value = "./nes_config.toml")]
        output_path: PathBuf,
    },
}

fn visual_config(output_file: PathBuf) {
    todo!()
}

fn main() {
    let cli = Cli::parse();
    if let Some(comm) = cli.command {
        match comm {
            Commands::ConfigVisual { output_path } => visual_config(output_path),
        };
    }

    if !cli.game_file.exists() {
        panic!("path {:?} does not exist", cli.game_file);
    }

    run_emu(cli.game_file);
}

fn run_emu(game_path: PathBuf) {
    let mut bus: Bus = Bus::default();
    let bytes = std::fs::read(game_path).unwrap();
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
