use crate::bus::ppu_registers::PPURegisters;

#[test]
fn test_initialization() {
    let ppu_registers = PPURegisters::new();
    assert_eq!(ppu_registers.get_address_u16(), 0);
}

#[test]
fn test_writing_to_ppu_addr() {
    let mut ppu_registers = PPURegisters::new();

    ppu_registers.write_to_addr_reg(0x10);
    ppu_registers.write_to_addr_reg(0xab);
    assert_eq!(ppu_registers.get_address_u16(), 0x10ab);
    ppu_registers.write_to_addr_reg(0x23);
    ppu_registers.write_to_addr_reg(0x45);
    assert_eq!(ppu_registers.get_address_u16(), 0x2345);

    // test increment
    ppu_registers.increase_address(1);
    assert_eq!(ppu_registers.get_address_u16(), 0x2346);
    ppu_registers.increase_address(0xba);
    assert_eq!(ppu_registers.get_address_u16(), 0x2400);
}

#[test]
fn write_while_resetting_latch() {
    let mut ppu_registers = PPURegisters::new();

    ppu_registers.write_to_addr_reg(0x10);
    ppu_registers.write_to_addr_reg(0x12);
    ppu_registers.write_to_addr_reg(0x23);
    assert_eq!(ppu_registers.get_address_u16(), 0x2312);

    ppu_registers.reset_latch();
    ppu_registers.write_to_addr_reg(0x10);

    ppu_registers.reset_latch();
    ppu_registers.write_to_addr_reg(0x12);
    ppu_registers.write_to_addr_reg(0x23);
    assert_eq!(ppu_registers.get_address_u16(), 0x1223);
}

#[test]
fn test_mirroring() {
    let mut ppu_registers = PPURegisters::new();

    ppu_registers.write_to_addr_reg(0xab);
    ppu_registers.write_to_addr_reg(0x12);
    assert_eq!(ppu_registers.get_address_u16(), 0xab12 & 0x3fff);
}
