use crate::bus::cartridge::Mirroring;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::generate_cpu;
use crate::generate_cpu_and_vram;

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
    generate_cpu_and_vram!(cpu, vram);
    // generate the memory
    vram[0xa0] = 0x01;
    vram[0xa1] = 0x02;
    vram[0xa2] = 0x03;

    prepare_for_ppu_memory_read(&mut cpu, 0x20a0);

    assert_eq!(cpu.read_memory(0x2007), 0x01);
    assert_eq!(cpu.read_memory(0x2007), 0x02);
    assert_eq!(cpu.read_memory(0x2007), 0x03);
}

#[test]
pub fn test_address_increment_on_ppu_data_read() {
    generate_cpu_and_vram!(cpu, vram);
    // set the increment to 32
    cpu.bus.ppu_registers.control_register.write_byte(0x04);
    // generate the memory
    vram[0xa0 + 32 * 0] = 0x01;
    vram[0xa0 + 32 * 1] = 0x02;
    vram[0xa0 + 32 * 2] = 0x03;

    prepare_for_ppu_memory_read(&mut cpu, 0x20a0);

    assert_eq!(cpu.read_memory(0x2007), 0x01);
    assert_eq!(cpu.read_memory(0x2007), 0x02);
    assert_eq!(cpu.read_memory(0x2007), 0x03);
}

#[test]
pub fn test_write_to_ppu_memory() {
    generate_cpu!(cpu);
    cpu.bus.cartridge.screen_mirroring = Mirroring::Vertical;
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
    generate_cpu!(cpu);
    cpu.bus.cartridge.screen_mirroring = Mirroring::Vertical;

    // since we read the ppu status register, we should have the address at 0x20a0 and not 0xa020
    cpu.write_memory(0x2006, 0x00);
    cpu.read_memory(0x2002);
    cpu.write_memory(0x2006, 0x20);
    cpu.write_memory(0x2006, 0xa0);

    cpu.write_memory(0x2007, 0xab);
    assert_eq!(cpu.bus.ppu_memory.vram[0xa0], 0xab)
}

#[test]
fn test_vertical_mirroring() {
    generate_cpu_and_vram!(cpu, vram);
    cpu.bus.cartridge.screen_mirroring = Mirroring::Vertical;

    vram[0x00] = 0x12;
    vram[0x123] = 0x34;
    vram[0x3FF] = 0x56;
    vram[0x400] = 0x78;
    vram[0x7FF] = 0x9A;

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
    generate_cpu_and_vram!(cpu, vram);
    cpu.bus.cartridge.screen_mirroring = Mirroring::Horizontal;

    vram[0x00] = 0x12;
    vram[0x123] = 0x34;
    vram[0x3FF] = 0x56;
    vram[0x800] = 0x78;
    vram[0xBFF] = 0x9A;

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
