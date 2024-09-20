use crate::cpu::CPU;

fn set_compare_tests(cpu: &mut CPU, register_name: &str, register_value: u8, operand: u8, should_negative: bool, should_carry: bool) {
    match register_name {
        "A" => {
            cpu.register_a = register_value;
            cpu.CMP(operand);
            assert_eq!(cpu.get_status_n(), should_negative);
            assert_eq!(cpu.get_status_c(), should_carry);
            assert_eq!(cpu.get_status_z(), cpu.register_a == operand);
        }
        "X" => {
            cpu.register_x = register_value;
            cpu.CPX(operand);
            assert_eq!(cpu.get_status_n(), should_negative);
            assert_eq!(cpu.get_status_c(), should_carry);
            assert_eq!(cpu.get_status_z(), cpu.register_x == operand);
        }
        "Y" => {
            cpu.register_y = register_value;
            cpu.CPY(operand);
            assert_eq!(cpu.get_status_n(), should_negative);
            assert_eq!(cpu.get_status_c(), should_carry);
            assert_eq!(cpu.get_status_z(), cpu.register_y == operand);
        }
        _ => panic!("Unknown register called {:?}", register_name)
    }

}

fn test_compare(cpu: &mut CPU, register_name: &str) {
    set_compare_tests(cpu, register_name, 0, 0, false, true);
    set_compare_tests(cpu, register_name, 0xd2, 0xa7, false, true);
    set_compare_tests(cpu, register_name, 0xee, 0x69, true, true);
    set_compare_tests(cpu, register_name, 0x94, 0x72, false, true);
    set_compare_tests(cpu, register_name, 0x3f, 0x80, true, false);
    set_compare_tests(cpu, register_name, 0x18, 0x09, false, true);
    set_compare_tests(cpu, register_name, 0x63, 0xf3, false, false);
    set_compare_tests(cpu, register_name, 0x63, 0x63, false, true);
}

#[test]
#[allow(non_snake_case)]
fn CMP() {
    let mut cpu: CPU = CPU::new();
    test_compare(&mut cpu, "A");
}

#[test]
#[allow(non_snake_case)]
fn CPX() {
    let mut cpu: CPU = CPU::new();
    test_compare(&mut cpu, "X");
}

#[test]
#[allow(non_snake_case)]
fn CPY() {
    let mut cpu: CPU = CPU::new();
    test_compare(&mut cpu, "Y");
}