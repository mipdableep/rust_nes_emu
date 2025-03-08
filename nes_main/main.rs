mod cli_config;
mod visual_config;

use nes_emulator::bus::{Bus, Config};
use nes_emulator::cpu::CPU;
use nes_emulator::generate_texture_canvas_event_pump;
use nes_emulator::ppu::frame::Frame;
use nes_emulator::ppu::PPU;

use clap::{Parser, Subcommand};
use std::fs::{read_to_string, write};
use std::path::PathBuf;

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
    #[arg(short = 'c', long, value_name = "config_file")]
    config_file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// a visual configurator
    // ConfigVisual {
    //     /// costum output path for the new config
    //     #[clap(long, short = 'o')]
    //     #[clap(default_value = "./nes_config.toml")]
    //     output_path: PathBuf,
    // },
    /// a cli configurator
    ConfigCli {
        /// costum output path for the new config
        #[clap(long, short = 'o')]
        #[clap(default_value = "./nes_config.toml")]
        output_path: PathBuf,
    },
    DefaultConfig {
        /// costum output path for the new config
        #[clap(long, short = 'o')]
        #[clap(default_value = "./nes_config.toml")]
        output_path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut conf = Config::default();

    if !cli.game_file.exists() {
        panic!("path {:?} does not exist", cli.game_file);
    }

    if let Some(comm) = cli.command {
        match comm {
            Commands::DefaultConfig { output_path } => {
                let t_s = toml::to_string(&Config::default())
                    .expect("should always work - a serde operation");
                match write(&output_path, t_s) {
                    Ok(_) => println!("writen default config to {output_path:?}"),
                    Err(e) => println!("failed to write to file {output_path:?} - {e}"),
                }
            }
            Commands::ConfigCli { output_path } => cli_config::create_config(output_path),
        }
        return;
    }

    if let Some(c) = &cli.config_file {
        if let Ok(s) = read_to_string(c) {
            if let Ok(t_conf) = toml::from_str(&s) {
                conf = t_conf;
            }
        }
    }

    run_emu(cli.game_file, conf);
}

fn run_emu(game_path: PathBuf, conf: Config) {
    let mut bus: Bus = Bus {
        config: conf,
        ..Default::default()
    };
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
