use crate::cpu::CPU;

fn set_add_test(cpu : &mut CPU, reg_a: u8, operand: u8,should_overflow: bool, should_carry: bool) {
    let expected_result = operand.wrapping_add(reg_a);
    cpu.register_a = reg_a;
    cpu.ADC(operand);
    println!("{:} {:}", reg_a, operand);
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
