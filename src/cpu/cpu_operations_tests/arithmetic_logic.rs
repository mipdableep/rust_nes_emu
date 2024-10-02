use crate::cpu::CPU;

fn set_add_test(
    cpu: &mut CPU,
    reg_a: u8,
    operand: u8,
    should_overflow: bool,
    should_carry: bool,
    old_carry: bool,
) {
    let expected_result = operand.wrapping_add(reg_a).wrapping_add(match old_carry {
        true => 1,
        false => 0,
    });
    cpu.set_carry(old_carry);
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
    set_add_test(&mut cpu, 5, 3, false, false, false);
    set_add_test(&mut cpu, 0, 0, false, false, false);
    set_add_test(&mut cpu, 1, 0xFF, false, true, false);
    set_add_test(&mut cpu, 0xFF, 0xFF, false, true, false);
    set_add_test(&mut cpu, 0x40, 0x40, true, false, false);
    set_add_test(&mut cpu, 0x3f, 0x40, false, false, false);

    set_add_test(&mut cpu, 0xa5, 0x84, true, true, false);
    set_add_test(&mut cpu, 0x39, 0xde, false, true, false);
    set_add_test(&mut cpu, 0xdd, 0x0a, false, false, false);
    set_add_test(&mut cpu, 0xc8, 0x53, false, true, false);
    set_add_test(&mut cpu, 0x5b, 0xde, false, true, false);
    set_add_test(&mut cpu, 0x5b, 0xde, false, true, false);
    set_add_test(&mut cpu, 0x8c, 0x75, false, true, false);
    set_add_test(&mut cpu, 0x04, 0x27, false, false, false);

    set_add_test(&mut cpu, 5, 3, false, false, true);
    set_add_test(&mut cpu, 0, 0, false, false, true);
    set_add_test(&mut cpu, 1, 0xFF, false, true, true);
    set_add_test(&mut cpu, 0xFF, 0xFF, false, true, true);
    set_add_test(&mut cpu, 0xa5, 0x84, true, true, true);
    set_add_test(&mut cpu, 0x39, 0xde, false, true, true);
    set_add_test(&mut cpu, 0xdd, 0x0a, false, false, true);
    set_add_test(&mut cpu, 0x00, 0xFF, false, true, true);
    set_add_test(&mut cpu, 0x080, 0x7f, false, true, true);
    set_add_test(&mut cpu, 0x40, 0x40, true, false, true);
    set_add_test(&mut cpu, 0x3f, 0x40, true, false, true);
}

fn get_random_u8_pairs() -> Vec<[u8; 2]> {
    let mut res: Vec<[u8; 2]> = Vec::new();
    res.push([5, 3]);
    res.push([0, 0]);
    res.push([1, 0xFF]);
    res.push([0xFF, 0xFF]);
    res.push([0xa5, 0x84]);
    res.push([0x39, 0xde]);
    res.push([0xdd, 0x0a]);
    res.push([0xc8, 0x53]);
    res.push([0x5b, 0xde]);
    res.push([0x5b, 0xde]);
    res.push([0x8c, 0x75]);
    res.push([0x04, 0x27]);
    res
}

pub fn get_random_u8_and_u16_pairs() -> Vec<(u16, u8)> {
    let mut res: Vec<(u16, u8)> = Vec::new();
    res.push((0x00fa, 0x00));
    res.push((0x5501, 0));
    res.push((0x1f01, 0x81));
    res.push((0x7fff, 0xff));
    res.push((0x7fff, 0x00));
    res.push((0x0000, 0x00));
    res.push((0x0000, 0xff));
    res.push((0x5501, 0x2a));
    res
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
    for test_pair in get_random_u8_pairs() {
        set_and_test(&mut cpu, test_pair[0], test_pair[1]);
    }
}

fn set_asl_accumulator_test(cpu: &mut CPU, reg_a: u8) {
    let expected_result = reg_a.wrapping_mul(2);
    let should_carry = reg_a >= 0x80;
    cpu.register_a = reg_a;
    cpu.ASL_accumulator();
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn ASL_accumulator() {
    let mut cpu: CPU = CPU::new();
    set_asl_accumulator_test(&mut cpu, 0);
    set_asl_accumulator_test(&mut cpu, 128);
    set_asl_accumulator_test(&mut cpu, 0x1f);
    set_asl_accumulator_test(&mut cpu, 0xFF);
    set_asl_accumulator_test(&mut cpu, 0x01);
}

fn set_asl_memory_tests(cpu: &mut CPU, value: u8, address: u16) {
    cpu.write_memory(address, value);
    let should_carry = value >= 0x80;
    let expected_result = value.wrapping_mul(2);
    cpu.ASL_memory(address);
    assert_eq!(cpu.read_memory(address), expected_result);
    assert_eq!(cpu.get_status_z(), cpu.read_memory(address) == 0);
    assert_eq!(cpu.get_status_n(), cpu.read_memory(address) >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn ASL_memory() {
    let mut cpu: CPU = CPU::new();
    // test for some memory addresses and values:
    for (address, value) in get_random_u8_and_u16_pairs() {
        set_asl_memory_tests(&mut cpu, value, address);
    }
    // test for multiple shifts of the same memory
    let address: u16 = 0x51d0;
    cpu.write_memory(address, 0b10110010);
    cpu.ASL_memory(address);
    assert!(cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b01100100);
    cpu.ASL_memory(address);
    assert!(!cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b11001000);
}

fn set_lsr_accumulator_test(cpu: &mut CPU, reg_a: u8) {
    let expected_result = reg_a / 2; // integer division
    let should_carry = reg_a % 2 == 1;
    cpu.register_a = reg_a;
    cpu.LSR_accumulator();
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn LSR_accumulator() {
    let mut cpu: CPU = CPU::new();
    set_lsr_accumulator_test(&mut cpu, 0);
    set_lsr_accumulator_test(&mut cpu, 128);
    set_lsr_accumulator_test(&mut cpu, 0x1f);
    set_lsr_accumulator_test(&mut cpu, 0xFF);
    set_lsr_accumulator_test(&mut cpu, 0x01);
}

fn set_lsr_memory_tests(cpu: &mut CPU, value: u8, address: u16) {
    cpu.write_memory(address, value);
    let should_carry = value % 2 == 1;
    let expected_result = value / 2; // integer division
    cpu.LSR_memory(address);
    assert_eq!(cpu.read_memory(address), expected_result);
    assert_eq!(cpu.get_status_z(), cpu.read_memory(address) == 0);
    assert_eq!(cpu.get_status_n(), cpu.read_memory(address) >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn LSR_memory() {
    let mut cpu: CPU = CPU::new();
    // test for some memory addresses and values:
    for (address, value) in get_random_u8_and_u16_pairs() {
        set_lsr_memory_tests(&mut cpu, value, address);
    }
    // test for multiple shifts of the same memory
    let address: u16 = 0x51d0;
    cpu.write_memory(address, 0b10110010);
    cpu.LSR_memory(address);
    assert!(!cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b01011001);
    cpu.LSR_memory(address);
    assert!(cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b00101100);
}

fn set_ror_accumulator_test(cpu: &mut CPU, reg_a: u8) {
    let expected_result = reg_a >> 1
        | match cpu.get_status_c() {
            true => 0x80,
            false => 0x00,
        }; // integer division
    let should_carry = reg_a % 2 == 1;
    cpu.register_a = reg_a;
    cpu.ROR_accumulator();
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn ROR_accumulator() {
    let mut cpu: CPU = CPU::new();
    set_ror_accumulator_test(&mut cpu, 0);
    set_ror_accumulator_test(&mut cpu, 128);
    set_ror_accumulator_test(&mut cpu, 0x1f);
    set_ror_accumulator_test(&mut cpu, 0xFF);
    set_ror_accumulator_test(&mut cpu, 0x01);
}

fn set_ror_memory_tests(cpu: &mut CPU, value: u8, address: u16) {
    cpu.write_memory(address, value);
    let should_carry = value % 2 == 1;
    let expected_result = value >> 1
        | match cpu.get_status_c() {
            true => 0x80,
            false => 0x00,
        }; // integer division
    cpu.ROR_memory(address);
    assert_eq!(cpu.read_memory(address), expected_result);
    assert_eq!(cpu.get_status_z(), cpu.read_memory(address) == 0);
    assert_eq!(cpu.get_status_n(), cpu.read_memory(address) >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn ROR_memory() {
    let mut cpu: CPU = CPU::new();
    // test for some memory addresses and values:
    for (address, value) in get_random_u8_and_u16_pairs() {
        set_ror_memory_tests(&mut cpu, value, address);
    }
    // test for multiple shifts of the same memory
    let address: u16 = 0x51d0;
    cpu.set_carry(true);
    cpu.write_memory(address, 0b10110010);
    cpu.ROR_memory(address);
    assert!(!cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b11011001);
    cpu.ROR_memory(address);
    assert!(cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b01101100);
}

fn set_rol_accumulator_test(cpu: &mut CPU, reg_a: u8) {
    let expected_result = reg_a << 1
        | match cpu.get_status_c() {
            true => 0x01,
            false => 0x00,
        }; // integer division
    let should_carry = reg_a & 0x80 == 0x80;
    cpu.register_a = reg_a;
    cpu.ROL_accumulator();
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn ROL_accumulator() {
    let mut cpu: CPU = CPU::new();
    set_rol_accumulator_test(&mut cpu, 0);
    set_rol_accumulator_test(&mut cpu, 128);
    set_rol_accumulator_test(&mut cpu, 0x1f);
    set_rol_accumulator_test(&mut cpu, 0xFF);
    set_rol_accumulator_test(&mut cpu, 0x01);
}

fn set_rol_memory_tests(cpu: &mut CPU, value: u8, address: u16) {
    cpu.write_memory(address, value);
    let should_carry = value & 0x80 == 0x80;
    let expected_result = value << 1
        | match cpu.get_status_c() {
            true => 0x01,
            false => 0x00,
        }; // integer division
    cpu.ROL_memory(address);
    assert_eq!(cpu.read_memory(address), expected_result);
    assert_eq!(cpu.get_status_z(), cpu.read_memory(address) == 0);
    assert_eq!(cpu.get_status_n(), cpu.read_memory(address) >> 7 == 1);
    assert_eq!(cpu.get_status_c(), should_carry);
}

#[test]
#[allow(non_snake_case)]
fn ROL_memory() {
    let mut cpu: CPU = CPU::new();
    // test for some memory addresses and values:
    for (address, value) in get_random_u8_and_u16_pairs() {
        set_rol_memory_tests(&mut cpu, value, address);
    }
    // test for multiple shifts of the same memory
    let address: u16 = 0x51d0;
    cpu.set_carry(false);
    cpu.write_memory(address, 0b10110010);
    cpu.ROL_memory(address);
    assert!(cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b01100100);
    cpu.ROL_memory(address);
    assert!(!cpu.get_status_c());
    assert_eq!(cpu.read_memory(address), 0b11001001);
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

fn set_eor_test(cpu: &mut CPU, reg_a: u8, operand: u8) {
    let expected_result = operand ^ reg_a;
    cpu.register_a = reg_a;
    cpu.EOR(operand);
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
}

#[test]
#[allow(non_snake_case)]
fn EOR() {
    let mut cpu: CPU = CPU::new();
    for test_pair in get_random_u8_pairs() {
        set_eor_test(&mut cpu, test_pair[0], test_pair[1]);
    }
}

fn set_ora_test(cpu: &mut CPU, reg_a: u8, operand: u8) {
    let expected_result = operand | reg_a;
    cpu.register_a = reg_a;
    cpu.ORA(operand);
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
}

#[test]
#[allow(non_snake_case)]
fn ORA() {
    let mut cpu: CPU = CPU::new();
    for test_pair in get_random_u8_pairs() {
        set_ora_test(&mut cpu, test_pair[0], test_pair[1]);
    }
}

fn set_sub_test(
    cpu: &mut CPU,
    reg_a: u8,
    operand: u8,
    should_overflow: bool,
    should_carry: bool,
    old_carry: bool,
) {
    let expected_result = reg_a.wrapping_sub(operand.wrapping_add(match old_carry {
        true => 0,
        false => 1,
    }));
    cpu.set_carry(old_carry);
    cpu.register_a = reg_a;
    cpu.SBC(operand);
    assert_eq!(cpu.register_a, expected_result);
    assert_eq!(cpu.get_status_v(), should_overflow);
    assert_eq!(cpu.get_status_c(), should_carry);
    assert_eq!(cpu.get_status_z(), cpu.register_a == 0);
    assert_eq!(cpu.get_status_n(), cpu.register_a >> 7 == 1);
}

#[test]
#[allow(non_snake_case)]
fn SBC() {
    let mut cpu: CPU = CPU::new();
    // manual tests
    set_sub_test(&mut cpu, 0x00, 0x00, false, true, true);
    set_sub_test(&mut cpu, 0x00, 0x00, false, false, false);
    set_sub_test(&mut cpu, 0x40, 0xbf, true, false, false);
    set_sub_test(&mut cpu, 0x40, 0x7f, false, false, false);
    set_sub_test(&mut cpu, 0x40, 0x7f, false, false, true);

    // generated some random values using the python script and chugged them to https://skilldrick.github.io/easy6502/
    set_sub_test(&mut cpu, 0x34, 0xCC, false, false, true);
    set_sub_test(&mut cpu, 0xCA, 0x77, true, true, true);
    set_sub_test(&mut cpu, 0x97, 0x77, true, true, true);
    set_sub_test(&mut cpu, 0xAF, 0xFF, false, false, false);
    set_sub_test(&mut cpu, 0x35, 0xBB, false, false, true);
    set_sub_test(&mut cpu, 0x79, 0x33, false, true, false);
    set_sub_test(&mut cpu, 0x08, 0x66, false, false, true);
    set_sub_test(&mut cpu, 0xE7, 0x66, false, true, false);
    set_sub_test(&mut cpu, 0xE1, 0xAA, false, true, false);
}
