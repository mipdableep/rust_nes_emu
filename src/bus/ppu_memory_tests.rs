use crate::bus::cartridge::Mirroring;
use crate::bus::memory::Mem;
use crate::bus::memory_mapping_constants::{PPU_REGISTERS_END, PPU_REGISTERS_START};
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::{generate_cpu_and_set_horizontal_mirroring, generate_cpu_and_set_vertical_mirroring};

fn prepare_for_ppu_memory_read(cpu: &mut CPU, address: u16) {
    // a helper function to prepare for read from a specific ppu memory
    // needed because reading from the ppu memory is hard
    // this sets the address register in the ppu for future usage, and does a dummy read

    // prepare reading the memory
    cpu.write_memory(0x2006, (address >> 8) as u8);
    cpu.write_memory(0x2006, (address & 0xFF) as u8);

    // dummy read
    cpu.read_memory(0x2007);
}

#[test]
pub fn test_access_ppu_memory_from_cpu() {
    generate_cpu_and_set_vertical_mirroring!(cpu);

    // generate the memory
    let bus = cpu.bus.take().unwrap();

    let vram = &mut bus.ppu_memory.vram;
    vram[0xa0] = 0x01;
    vram[0xa1] = 0x02;
    vram[0xa2] = 0x03;

    cpu.bus = Some(bus);

    prepare_for_ppu_memory_read(&mut cpu, 0x20a0);

    assert_eq!(cpu.read_memory(0x2007), 0x01);
    assert_eq!(cpu.read_memory(0x2007), 0x02);
    assert_eq!(cpu.read_memory(0x2007), 0x03);
}

#[test]
pub fn test_address_increment_on_ppu_data_read() {
    generate_cpu_and_set_vertical_mirroring!(cpu);

    // generate the memory
    let bus = cpu.bus.take().unwrap();

    let vram = &mut bus.ppu_memory.vram;
    vram[0xa0 + 32 * 0] = 0x01;
    vram[0xa0 + 32 * 1] = 0x02;
    vram[0xa0 + 32 * 2] = 0x03;

    cpu.bus = Some(bus);

    // set the increment to 32
    cpu.bus
        .as_mut()
        .unwrap()
        .ppu_registers
        .control_register
        .write_byte(0x04);

    prepare_for_ppu_memory_read(&mut cpu, 0x20a0);

    assert_eq!(cpu.read_memory(0x2007), 0x01);
    assert_eq!(cpu.read_memory(0x2007), 0x02);
    assert_eq!(cpu.read_memory(0x2007), 0x03);
}

#[test]
pub fn test_write_to_ppu_memory() {
    generate_cpu_and_set_vertical_mirroring!(cpu);
    // generate the memory
    cpu.write_memory(0x2006, 0x20);
    cpu.write_memory(0x2006, 0xa0);

    cpu.write_memory(0x2007, 0x01);
    cpu.write_memory(0x2007, 0x02);
    cpu.write_memory(0x2007, 0x03);

    prepare_for_ppu_memory_read(&mut cpu, 0x20a0);

    assert_eq!(cpu.read_memory(0x2007), 0x01);
    assert_eq!(cpu.read_memory(0x2007), 0x02);
    assert_eq!(cpu.read_memory(0x2007), 0x03);
}

#[test]
fn test_read_from_status_resets_latch() {
    generate_cpu_and_set_vertical_mirroring!(cpu);

    // since we read the ppu status register, we should have the address at 0x20a0 and not 0xa020
    cpu.write_memory(0x2006, 0x00);
    cpu.read_memory(0x2002);
    cpu.write_memory(0x2006, 0x20);
    cpu.write_memory(0x2006, 0xa0);

    cpu.write_memory(0x2007, 0xab);
    assert_eq!(cpu.bus.as_mut().unwrap().ppu_memory.vram[0xa0], 0xab)
}

#[test]
fn test_ppu_mirroring() {
    let mut bus = Bus::new();

    bus.write_memory(PPU_REGISTERS_START + 8 * 5, 0xa4); //PPU control

    // the 8 ppu register should be mirrored across a very large region
    // we use a register which is both readable and writeable, and doesn't arbitrarily change its data
    // (rare requirement for a ppu register)
    assert_eq!(bus.read_memory(PPU_REGISTERS_END - 7), 0xa4);
}

#[test]
fn test_vertical_mirroring() {
    generate_cpu_and_set_vertical_mirroring!(cpu);

    let bus = cpu.bus.take().unwrap();

    let vram = &mut bus.ppu_memory.vram;
    vram[0x00] = 0x12;
    vram[0x123] = 0x34;
    vram[0x3FF] = 0x56;
    vram[0x400] = 0x78;
    vram[0x7FF] = 0x9A;

    cpu.bus = Some(bus);

    prepare_for_ppu_memory_read(&mut cpu, 0x2800); // mirror of 0x2000
    assert_eq!(cpu.read_memory(0x2007), 0x12);
    prepare_for_ppu_memory_read(&mut cpu, 0x2923); //mirror of 0x2123
    assert_eq!(cpu.read_memory(0x2007), 0x34);
    prepare_for_ppu_memory_read(&mut cpu, 0x2BFF); //mirror of 0x23FF
    assert_eq!(cpu.read_memory(0x2007), 0x56);
    prepare_for_ppu_memory_read(&mut cpu, 0x2C00); //mirror of 0x2400
    assert_eq!(cpu.read_memory(0x2007), 0x78);

    prepare_for_ppu_memory_read(&mut cpu, 0x2FFF); //mirror of 0x27FF

    // we cannot read directly - it will read from 0x3000, which is outside the bounds
    // so we will read from a random address and use the internal buffer
    cpu.write_memory(0x2006, 0x20);
    cpu.write_memory(0x2006, 0x20);

    assert_eq!(cpu.read_memory(0x2007), 0x9a);
}

#[test]
fn test_horizontal_mirroring() {
    generate_cpu_and_set_horizontal_mirroring!(cpu);

    let bus = cpu.bus.take().unwrap();
    let vram = &mut bus.ppu_memory.vram;
    vram[0x00] = 0x12;
    vram[0x123] = 0x34;
    vram[0x3FF] = 0x56;
    vram[0x800] = 0x78;
    vram[0xBFF] = 0x9A;

    cpu.bus = Some(bus);

    prepare_for_ppu_memory_read(&mut cpu, 0x2400); // mirror of 0x2000
    assert_eq!(cpu.read_memory(0x2007), 0x12);
    prepare_for_ppu_memory_read(&mut cpu, 0x2523); //mirror of 0x2123
    assert_eq!(cpu.read_memory(0x2007), 0x34);
    prepare_for_ppu_memory_read(&mut cpu, 0x27FF); //mirror of 0x23FF
    assert_eq!(cpu.read_memory(0x2007), 0x56);
    prepare_for_ppu_memory_read(&mut cpu, 0x2C00); //mirror of 0x2800
    assert_eq!(cpu.read_memory(0x2007), 0x78);

    prepare_for_ppu_memory_read(&mut cpu, 0x2FFF); //mirror of 0x2BFF

    // we cannot read directly - it will read from 0x3000, which is outside the bounds
    // so we will read from a random address and use the internal buffer
    cpu.write_memory(0x2006, 0x20);
    cpu.write_memory(0x2006, 0x20);

    assert_eq!(cpu.read_memory(0x2007), 0x9a);
}
