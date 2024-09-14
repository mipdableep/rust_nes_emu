use crate::cpu::CPU;

fn set_add_test(cpu: &mut CPU, reg_a: u8, operand: u8, should_overflow: bool, should_carry: bool) {
    let expected_result = operand.wrapping_add(reg_a);
    cpu.register_a = reg_a;
    cpu.ADC(operand);
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_v(), should_overflow);
    assert_eq!(cpu.get_status_c(), should_carry);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
}

#[test]
#[allow(non_snake_case)]
fn ADC() {
    let mut cpu: CPU = CPU::new();
    set_add_test(&mut cpu, 5, 3, false, false);
    set_add_test(&mut cpu, 0, 0, false, false);
    set_add_test(&mut cpu, 1, 0xFF, false, true);
    set_add_test(&mut cpu, 0xFF, 0xFF, false, true);
    set_add_test(&mut cpu, 0xa5, 0x84, true, true);
    set_add_test(&mut cpu, 0x39, 0xde, false, true);
    set_add_test(&mut cpu, 0xdd, 0x0a, false, false);
    set_add_test(&mut cpu, 0xc8, 0x53, false, true);
    set_add_test(&mut cpu, 0x5b, 0xde, false, true);
    set_add_test(&mut cpu, 0x5b, 0xde, false, true);
    set_add_test(&mut cpu, 0x8c, 0x75, false, true);
    set_add_test(&mut cpu, 0x04, 0x27, false, false);
}

fn set_and_test(cpu: &mut CPU, reg_a: u8, operand: u8) {
    let expected_result = operand & reg_a;
    cpu.register_a = reg_a;
    cpu.AND(operand);
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
}

#[test]
#[allow(non_snake_case)]
fn AND() {
    let mut cpu: CPU = CPU::new();
    set_and_test(&mut cpu, 5, 3);
    set_and_test(&mut cpu, 0, 0);
    set_and_test(&mut cpu, 1, 0xFF);
    set_and_test(&mut cpu, 0xFF, 0xFF);
    set_and_test(&mut cpu, 0xa5, 0x84);
    set_and_test(&mut cpu, 0x39, 0xde);
    set_and_test(&mut cpu, 0xdd, 0x0a);
    set_and_test(&mut cpu, 0xc8, 0x53);
    set_and_test(&mut cpu, 0x5b, 0xde);
    set_and_test(&mut cpu, 0x5b, 0xde);
    set_and_test(&mut cpu, 0x8c, 0x75);
    set_and_test(&mut cpu, 0x04, 0x27);
}

fn set_asl_test(cpu: &mut CPU, reg_a: u8) {
    let expected_result = reg_a.wrapping_mul(2);
    let should_carry = reg_a >= 0x80;
    cpu.register_a = reg_a;
    cpu.ASL();
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry)
}

#[test]
#[allow(non_snake_case)]
fn ASL() {
    let mut cpu: CPU = CPU::new();
    set_asl_test(&mut cpu, 0);
    set_asl_test(&mut cpu, 128);
    set_asl_test(&mut cpu, 0x1f);
    set_asl_test(&mut cpu, 0xFF);
    set_asl_test(&mut cpu, 0x01);
}

fn set_bit_test(cpu: &mut CPU, operand: u8) {
    let old_reg_a = cpu.register_a;
    cpu.BIT(operand);
    assert_eq!(cpu.register_a, old_reg_a);
    assert_eq!(cpu.get_status_z(), cpu.register_a & operand == 0);
    assert_eq!(cpu.get_status_v() as u8, operand >> 6 & 1);
    assert_eq!(cpu.get_status_n() as u8, operand >> 7 & 1);


}

#[test]
#[allow(non_snake_case)]
fn BIT() {
    let mut cpu = CPU::new();
    set_bit_test(&mut cpu, 0);
    set_bit_test(&mut cpu, 0x80);
    set_bit_test(&mut cpu, 0xff);
    cpu.register_a = 0x5d;
    set_bit_test(&mut cpu, 0);
    set_bit_test(&mut cpu, 0x5d);
    set_bit_test(&mut cpu, 0x6a);
}
