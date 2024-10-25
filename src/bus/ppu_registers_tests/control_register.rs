use crate::bus::ppu_registers::control_register::PPUControlRegister;

#[test]
fn test_read_and_write() {
    let mut ppu_addr_reg = PPUControlRegister::new();
    assert_eq!(ppu_addr_reg.read(), 0);
    ppu_addr_reg.write_byte(0x34);
    assert_eq!(ppu_addr_reg.read(), 0x34);
}

#[test]
fn test_get_vblank_nmi() {
    let mut ppu_addr_reg = PPUControlRegister::new();
    assert!(!ppu_addr_reg.get_vblank_nmi());
    ppu_addr_reg.write_byte(0x80);
    assert!(ppu_addr_reg.get_vblank_nmi());
    ppu_addr_reg.write_byte(0x7F);
    assert!(!ppu_addr_reg.get_vblank_nmi());
    ppu_addr_reg.write_byte(0x12);
    assert!(!ppu_addr_reg.get_vblank_nmi());
}

#[test]
fn test_get_vram_address_increment() {
    let mut ppu_addr_reg = PPUControlRegister::new();
    assert_eq!(ppu_addr_reg.get_vram_address_inc(), 1);
    ppu_addr_reg.write_byte(0x04);
    assert_eq!(ppu_addr_reg.get_vram_address_inc(), 32);
    ppu_addr_reg.write_byte(0xFB);
    assert_eq!(ppu_addr_reg.get_vram_address_inc(), 1);
    ppu_addr_reg.write_byte(0x12);
    assert_eq!(ppu_addr_reg.get_vram_address_inc(), 1);
}
