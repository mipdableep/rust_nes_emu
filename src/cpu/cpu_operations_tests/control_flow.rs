use cpu::cpu_operations::control_flow::convert_u8_to_i8_2sc;
use crate::cpu;
use crate::cpu::CPU;

#[test]
fn test_2s_conversion(){
    assert_eq!(convert_u8_to_i8_2sc(0), 0);
    assert_eq!(convert_u8_to_i8_2sc(0xff), -1);
    assert_eq!(convert_u8_to_i8_2sc(0x7f), 127);
    assert_eq!(convert_u8_to_i8_2sc(0x80), -128);
    assert_eq!(convert_u8_to_i8_2sc(0xa4), -0x5c);
    assert_eq!(convert_u8_to_i8_2sc(0x5f), 0x5f);
}

#[test]
#[allow(non_snake_case)]
fn BCC_and_BCS(){
    let mut cpu = CPU::new();
    cpu.program_counter = 0x800;
    cpu.set_carry(true);
    cpu.BCS(0xff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BCC(0x5a);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.set_carry(false);
    cpu.BCS(0xff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BCC(0x5a);
    assert_eq!(cpu.program_counter, 0x859);
}