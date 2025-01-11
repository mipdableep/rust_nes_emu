use crate::bus::ppu_registers::address_register::PPUAddressReg;

#[test]
fn test_initialization() {
    let addr_reg = PPUAddressReg::new();
    assert_eq!(addr_reg.get_address_as_u16(), 0);
}

#[test]
fn test_writing_to_ppu_addr() {
    let mut addr_reg = PPUAddressReg::new();
    let mut latch = true;
    addr_reg.write_byte(0x10, &mut latch);
    addr_reg.write_byte(0xab, &mut latch);
    assert_eq!(addr_reg.get_address_as_u16(), 0x10ab);
    addr_reg.write_byte(0x23, &mut latch);
    addr_reg.write_byte(0x45, &mut latch);
    assert_eq!(addr_reg.get_address_as_u16(), 0x2345);
    // test increment
    addr_reg.increment(1);
    assert_eq!(addr_reg.get_address_as_u16(), 0x2346);
    addr_reg.increment(0xba);
    assert_eq!(addr_reg.get_address_as_u16(), 0x2400);
}

#[test]
fn write_while_resetting_latch() {
    let mut addr_reg = PPUAddressReg::new();
    let mut latch = true;

    addr_reg.write_byte(0x10, &mut latch);
    addr_reg.write_byte(0x12, &mut latch);
    addr_reg.write_byte(0x23, &mut latch);
    assert_eq!(addr_reg.get_address_as_u16(), 0x2312);

    latch = true;
    addr_reg.write_byte(0x10, &mut latch);

    latch = true;
    addr_reg.write_byte(0x12, &mut latch);
    addr_reg.write_byte(0x23, &mut latch);
    assert_eq!(addr_reg.get_address_as_u16(), 0x1223);
}

#[test]
fn test_mirroring() {
    let mut addr_reg = PPUAddressReg::new();
    let mut latch = true;

    addr_reg.write_byte(0xab, &mut latch);
    addr_reg.write_byte(0x12, &mut latch);
    assert_eq!(addr_reg.get_address_as_u16(), 0xab12 & 0x3fff);
}

#[test]
fn test_read_last_value() {
    let mut addr_reg = PPUAddressReg::new();
    let mut latch = true;

    addr_reg.write_byte(0xab, &mut latch);
    assert_eq!(addr_reg.read(), 0xab);
    addr_reg.write_byte(0x12, &mut latch);
    assert_eq!(addr_reg.read(), 0x12);
}
