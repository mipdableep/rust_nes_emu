use crate::bus::ppu_registers::PPURegisters;

#[test]
fn test_scroll_setting() {
    let mut ppu_registers = PPURegisters::new();

    assert_eq!(ppu_registers.internal_registers.get_x_scroll(), 0);
    assert_eq!(ppu_registers.internal_registers.get_y_scroll(), 0);

    ppu_registers.write_to_scroll(0x10);
    ppu_registers.write_to_scroll(0xab);
    ppu_registers.write_to_scroll(0x20);

    // copy from temp vram to cur vram
    ppu_registers.copy_t_to_v();

    assert_eq!(ppu_registers.internal_registers.get_x_scroll(), 0x20);
    assert_eq!(ppu_registers.internal_registers.get_y_scroll(), 0xab);
}
