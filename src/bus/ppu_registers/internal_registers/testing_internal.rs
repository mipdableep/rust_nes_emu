use crate::{bus_mut, generate_cpu};

macro_rules! ppu_regs {
    ($cpu: ident) => {
        bus_mut!($cpu).ppu_registers
    };
}
macro_rules! inter_regs {
    ($cpu: ident) => {
        ppu_regs!($cpu).internal_registers
    };
}

#[test]
fn test_writing_nametable() {
    generate_cpu!(cpu);
    inter_regs!(cpu).temporary_vram = 0xffff;
    cpu.write_memory(0x2000, 0);
    assert_eq!(inter_regs!(cpu).temporary_vram, 0xf3ff)
}

#[test]
fn test_read_status_clear_latch() {
    generate_cpu!(cpu);
    inter_regs!(cpu).write_toggle = true;
    cpu.read_memory(0x2002);
    assert_eq!(inter_regs!(cpu).write_toggle, false);
    cpu.read_memory(0x2002);
    assert_eq!(inter_regs!(cpu).write_toggle, false);
}

#[test]
// based on https://www.nesdev.org/wiki/PPU_scrolling#Summary
fn test_internal_full() {
    generate_cpu!(cpu);
    let old_v: u16 = 0b1010110_10111100;
    let old_t: u16 = 0b1001100_00010010;
    inter_regs!(cpu).current_vram = old_v;
    inter_regs!(cpu).temporary_vram = old_t;
    inter_regs!(cpu).write_toggle = true;

    cpu.write_memory(0x2000, 0);
    assert_eq!(inter_regs!(cpu).temporary_vram, 0b1000000_00010010);
    assert_eq!(inter_regs!(cpu).current_vram, old_v);

    cpu.read_memory(0x2002);
    assert_eq!(inter_regs!(cpu).write_toggle, false);

    cpu.write_memory(0x2005, 0b01111101);
    assert_eq!(inter_regs!(cpu).write_toggle, true);
    assert_eq!(inter_regs!(cpu).temporary_vram, 0b1000000_00001111);
    assert_eq!(inter_regs!(cpu).fine_x, 0b101);
    assert_eq!(inter_regs!(cpu).current_vram, old_v);

    cpu.write_memory(0x2005, 0b01011110);
    assert_eq!(inter_regs!(cpu).write_toggle, false);
    assert_eq!(inter_regs!(cpu).temporary_vram, 0b1100001_01101111);
    assert_eq!(inter_regs!(cpu).current_vram, old_v);

    cpu.write_memory(0x2006, 0b00111101);
    assert_eq!(inter_regs!(cpu).write_toggle, true);
    assert_eq!(inter_regs!(cpu).temporary_vram, 0b0111101_01101111);
    assert_eq!(inter_regs!(cpu).current_vram, old_v);

    cpu.write_memory(0x2006, 0b11110000);
    assert_eq!(inter_regs!(cpu).write_toggle, false);
    assert_eq!(inter_regs!(cpu).temporary_vram, 0b0111101_11110000);
    assert_eq!(inter_regs!(cpu).current_vram, 0b0111101_11110000);
    assert_eq!(inter_regs!(cpu).fine_x, 0b101);
}
