use crate::bus::memory::Mem;
use crate::bus::memory_mapping_constants::*;
use crate::bus::Bus;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn get_random_vector_from_seed(seed: u8, length: u16) -> Vec<u8> {
    let mut generator = ChaCha12Rng::from_seed([seed; 32]);
    let mut result: Vec<u8> = Vec::new();
    for _ in 0..=length {
        result.push(generator.next_u32() as u8);
    }
    result
}

#[test]
fn test_bus_memory_boundaries() {
    let mut bus = Bus::new();
    // this test ensures that we can write at the boundaries of all memory segments without errors
    bus.write_memory(CPU_RAM_MEM_END, 0x01);
    // writing to the segment that is mapped to the ppu registers is a bit wierd
    bus.write_memory(IO_AND_AUDIO_REGISTERS_END, 0x03);
    bus.write_memory(UNMAPPED_SEG_END, 0x04);
    bus.write_memory(PRG_RAM_END, 0x05);
    assert_eq!(*bus.cpu_ram.last().unwrap(), 0x01);
    // writing to the segment that is mapped to the ppu registers is a bit wierd
    assert_eq!(*bus.io_and_audio_registers.last().unwrap(), 0x03);
    assert_eq!(*bus.unmapped_seg.last().unwrap(), 0x04);
    assert_eq!(*bus.prg_ram.last().unwrap(), 0x05);
}

#[test]
fn test_cpu_ram_mirroring() {
    let mut bus = Bus::new();
    let cpu_ram = get_random_vector_from_seed(42, CPU_RAM_MEM_END - CPU_RAM_MEM_START);
    for i in 0..=CPU_RAM_MEM_END - CPU_RAM_MEM_START {
        bus.write_memory(i, cpu_ram[i as usize]);
    }
    for i in CPU_RAM_MEM_START..=CPU_RAM_MEM_END {
        let real_address = i % CPU_RAM_MEM_UNIQUE_SIZE + 1 + CPU_RAM_MEM_END
            - CPU_RAM_MEM_START
            - CPU_RAM_MEM_UNIQUE_SIZE;
        assert_eq!(bus.read_memory(i), cpu_ram[real_address as usize]);
    }
}

// #[test]
fn test_ppu_reg_mirroring() {
    todo!(); // should either fix or move to the ppu section
    let mut bus = Bus::new();
    let ppu_registers = get_random_vector_from_seed(42, PPU_REGISTERS_END - PPU_REGISTERS_START);
    for i in 0..=PPU_REGISTERS_END - PPU_REGISTERS_START {
        bus.write_memory(PPU_REGISTERS_START + i, ppu_registers[i as usize]);
    }
    for i in PPU_REGISTERS_START..=PPU_REGISTERS_END {
        let real_address = i % PPU_REGISTERS_UNIQUE_SIZE + 1 + PPU_REGISTERS_END
            - PPU_REGISTERS_START
            - PPU_REGISTERS_UNIQUE_SIZE;
        assert_eq!(bus.read_memory(i), ppu_registers[real_address as usize]);
    }
}

#[test]
fn test_memory_retrieval() {
    let mut bus = Bus::new();
    let memory = get_random_vector_from_seed(42, 0x7FFF);
    for i in 0..=0x7FFF {
        if (PPU_REGISTERS_START <= i) & (i <= PPU_REGISTERS_END) {
            // skipping testing the ppu registers
            break;
        }
        bus.write_memory(i, memory[i as usize]);
        assert_eq!(bus.read_memory(i), memory[i as usize]);
    }
}

#[test]
fn test_read_memory_2_bytes() {
    let mut bus = Bus::new();
    let memory_contents = get_random_vector_from_seed(42, CPU_RAM_MEM_UNIQUE_SIZE);
    for i in CPU_RAM_MEM_START..CPU_RAM_MEM_UNIQUE_SIZE {
        bus.write_memory(i, memory_contents[i as usize])
    }
    for i in CPU_RAM_MEM_START..CPU_RAM_MEM_UNIQUE_SIZE / 2 {
        // 6502 is little endian
        let word_addr = 2 * i + CPU_RAM_MEM_START;
        let expected_result: u16 = ((memory_contents[(word_addr + 1) as usize] as u16) << 8)
            + (memory_contents[(word_addr) as usize] as u16);
        assert_eq!(bus.read_memory_2_bytes(word_addr), expected_result);
    }
}
