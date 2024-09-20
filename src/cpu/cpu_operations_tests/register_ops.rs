use crate::cpu::CPU;

fn set_cmp_test(cpu: &mut CPU, reg_a: u8, operand: u8, should_negative: bool, should_carry: bool) {
    cpu.register_a = reg_a;
    cpu.CMP(operand);
    assert_eq!(cpu.get_status_n(), should_negative);
    assert_eq!(cpu.get_status_c(), should_carry);
    assert_eq!(cpu.get_status_z(), cpu.register_a == operand);
}

#[test]
#[allow(non_snake_case)]
fn CMP() {
    let mut cpu: CPU = CPU::new();
    set_cmp_test(&mut cpu, 0, 0, false, true);
    set_cmp_test(&mut cpu, 0xd2, 0xa7, false, true);
    set_cmp_test(&mut cpu, 0xee, 0x69, true, true);
    set_cmp_test(&mut cpu, 0x94, 0x72, false, true);
    set_cmp_test(&mut cpu, 0x3f, 0x80, true, false);
    set_cmp_test(&mut cpu, 0x18, 0x09, false, true);
    set_cmp_test(&mut cpu, 0x63, 0xf3, false, false);
    set_cmp_test(&mut cpu, 0x63, 0x63, false, true);
}