use crate::bus::ppu_registers::control_register::PPUControlRegister;

#[test]
fn test_read_and_write() {
    let mut ppu_ctrl_reg = PPUControlRegister::new();
    assert_eq!(ppu_ctrl_reg.read(), 0);
    ppu_ctrl_reg.write_byte(0x34);
    assert_eq!(ppu_ctrl_reg.read(), 0x34);
}

#[test]
fn test_get_vblank_nmi() {
    let mut ppu_ctrl_reg = PPUControlRegister::new();
    assert!(!ppu_ctrl_reg.get_vblank_nmi());
    ppu_ctrl_reg.write_byte(0x80);
    assert!(ppu_ctrl_reg.get_vblank_nmi());
    ppu_ctrl_reg.write_byte(0x7F);
    assert!(!ppu_ctrl_reg.get_vblank_nmi());
    ppu_ctrl_reg.write_byte(0x12);
    assert!(!ppu_ctrl_reg.get_vblank_nmi());
}

#[test]
fn test_get_vram_address_increment() {
    let mut ppu_ctrl_reg = PPUControlRegister::new();
    assert_eq!(ppu_ctrl_reg.get_vram_address_inc(), 1);
    ppu_ctrl_reg.write_byte(0x04);
    assert_eq!(ppu_ctrl_reg.get_vram_address_inc(), 32);
    ppu_ctrl_reg.write_byte(0xFB);
    assert_eq!(ppu_ctrl_reg.get_vram_address_inc(), 1);
    ppu_ctrl_reg.write_byte(0x12);
    assert_eq!(ppu_ctrl_reg.get_vram_address_inc(), 1);
}
