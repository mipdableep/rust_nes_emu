pub mod cartridge;
mod controller;
#[cfg(test)]
pub mod mem_tests;
pub mod memory;
mod ppu_memory;
#[cfg(test)]
mod ppu_memory_tests;
pub mod ppu_registers;
#[cfg(test)]
mod ppu_registers_tests;
mod user_input;

use crate::bus::cartridge::Cartridge;
use crate::bus::ppu_memory::PPUMemory;
use controller::ControllerByte;
use memory_mapping_constants::*;
pub use ppu_memory::{NUMBER_OF_SPRITE, PPU_NAMETABLE_SIZE, PPU_NAMETABLE_START};
use ppu_registers::PPURegisters;
mod sdl2_keycode_serde;

#[macro_export]
macro_rules! bus {
    ($self: ident) => {
        $self.bus.as_ref().unwrap()
    };
}

#[macro_export]
macro_rules! bus_mut {
    ($self: ident) => {
        $self.bus.as_mut().unwrap()
    };
}

pub(crate) mod memory_mapping_constants {
    pub const CPU_RAM_MEM_START: u16 = 0x0000;
    pub const CPU_RAM_MEM_UNIQUE_SIZE: u16 = 0x0800;
    pub const CPU_RAM_MEM_END: u16 = 0x1FFF;
    pub const PPU_REGISTERS_START: u16 = 0x2000;
    pub const PPU_REGISTERS_UNIQUE_SIZE: u16 = 0x0008;
    pub const PPU_REGISTERS_END: u16 = 0x3FFF;
    pub const IO_AND_AUDIO_REGISTERS_START: u16 = 0x4000;
    pub const OAM_DMA: u16 = 0x4014;
    pub const P1_CONTROLLER: u16 = 0x4016;
    pub const P2_CONTROLLER: u16 = 0x4017;
    pub const IO_AND_AUDIO_REGISTERS_END: u16 = 0x401F;
    pub const UNMAPPED_SEG_START: u16 = 0x4020;
    pub const UNMAPPED_SEG_END: u16 = 0x5FFF;
    pub const PRG_RAM_START: u16 = 0x6000;
    pub const PRG_RAM_END: u16 = 0x7FFF;
    pub const PRG_ROM_START: u16 = 0x8000;
    pub const PRG_ROM_END: u16 = 0xFFFF;
}

use sdl2::keyboard::Keycode;
use serde::{Deserialize, Serialize};
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(with = "sdl2_keycode_serde")]
    pub up: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub down: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub left: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub right: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub a: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub b: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub select: Keycode,
    #[serde(with = "sdl2_keycode_serde")]
    pub start: Keycode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            up: Keycode::UP,
            down: Keycode::DOWN,
            left: Keycode::LEFT,
            right: Keycode::RIGHT,
            a: Keycode::A,
            b: Keycode::B,
            select: Keycode::O,
            start: Keycode::P,
        }
    }
}

impl Config {
    pub fn sensible_defaults() -> Self {
        Self {
            up: Keycode::W,
            down: Keycode::S,
            left: Keycode::A,
            right: Keycode::D,
            a: Keycode::N,
            b: Keycode::M,
            select: Keycode::U,
            start: Keycode::I,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Bus {
    // cpu stuff
    pub cpu_idle_cycles: u8,
    // something to hold if cpu should attend nmi
    pub nmi_generated: bool,
    // hold the values for oam dma
    pub oam_dma_page: u8,
    pub number_of_copies_in_current_oam_dma: u8,

    pub cpu_ram: [u8; CPU_RAM_MEM_UNIQUE_SIZE as usize],
    pub ppu_registers: PPURegisters,
    pub p1_controller: ControllerByte,
    pub p2_controller: ControllerByte,
    pub io_and_audio_registers:
        [u8; (IO_AND_AUDIO_REGISTERS_END - IO_AND_AUDIO_REGISTERS_START + 1) as usize],
    pub unmapped_seg: [u8; (UNMAPPED_SEG_END - UNMAPPED_SEG_START + 1) as usize],
    pub prg_ram: [u8; (PRG_RAM_END - PRG_RAM_START + 1) as usize],
    pub cartridge: Cartridge,
    pub ppu_memory: PPUMemory,
    pub config: Config,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            cpu_idle_cycles: 0,
            nmi_generated: false,
            oam_dma_page: 0,
            number_of_copies_in_current_oam_dma: 0,
            cpu_ram: [0; CPU_RAM_MEM_UNIQUE_SIZE as usize],
            ppu_registers: PPURegisters::new(),
            p1_controller: ControllerByte::new(),
            p2_controller: ControllerByte::new(),
            io_and_audio_registers: [0; (IO_AND_AUDIO_REGISTERS_END - IO_AND_AUDIO_REGISTERS_START
                + 1) as usize],
            unmapped_seg: [0; (UNMAPPED_SEG_END - UNMAPPED_SEG_START + 1) as usize],
            prg_ram: [0; (PRG_RAM_END - PRG_RAM_START + 1) as usize],
            cartridge: Cartridge::new(),
            ppu_memory: PPUMemory::new(),
            config: Config::default(),
        }
    }
}
