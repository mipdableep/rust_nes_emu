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

fn set_dec_test(cpu: &mut CPU, memory_address: u16, memory_value: u8) {
    cpu.write_memory(memory_address, memory_value);
    cpu.DEC(memory_address);
    assert_eq!(cpu.read_memory(memory_address).wrapping_add(1), memory_value);
    assert_eq!(cpu.get_status_z(), cpu.read_memory(memory_address) == 0);
    assert_eq!(cpu.get_status_n(), (cpu.read_memory(memory_address) >> 7) & 1 == 1);
}

#[test]
#[allow(non_snake_case)]
fn DEC() {
    let mut cpu: CPU = CPU::new();
    set_dec_test(&mut cpu, 0x00fa, 0x00);
    set_dec_test(&mut cpu, 0x5501, 0);
    set_dec_test(&mut cpu, 0x1f01, 0x81);
    set_dec_test(&mut cpu, 0xffff, 0xff);
    // check for multiple decreases of the same memory address
    let mem_addr: u16 = 0x57af;
    cpu.write_memory(mem_addr, 0x01);
    assert_eq!(cpu.read_memory(mem_addr), 0x01);
    cpu.DEC(mem_addr);
    assert_eq!(cpu.read_memory(mem_addr), 0x00);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.DEC(mem_addr);
    assert_eq!(cpu.read_memory(mem_addr), 0xff);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
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