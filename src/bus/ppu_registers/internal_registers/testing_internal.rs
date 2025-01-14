use crate::{bus_mut, generate_cpu};

macro_rules! inter_regs {
    ($cpu: ident) => {
        bus_mut!($cpu).ppu_registers.internal_registers
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
