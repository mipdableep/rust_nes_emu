use crate::bus::ppu_registers::PPURegisters;

mod address_register;
mod control_register;

#[test]
fn test_scroll_and_addr_use_the_same_latch() {
    let mut ppu_regs = PPURegisters::new();

    ppu_regs.write_to_addr_reg(0x10);
    ppu_regs.write_to_scroll(0x20);
    ppu_regs.write_to_addr_reg(0x30);
    // should use the same latch

    assert_eq!(ppu_regs.address_register.get_address_as_u16(), 0x3000);
    assert_eq!(ppu_regs.scroll_register.get_y_scroll(), 0x20);
    assert_eq!(ppu_regs.scroll_register.get_x_scroll(), 0x00);
}
