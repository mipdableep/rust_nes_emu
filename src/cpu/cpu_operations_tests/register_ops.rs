use crate::bus::Bus;
use crate::cpu::cpu_operations_tests::arithmetic_logic::get_random_u8_and_u16_pairs;
use crate::cpu::CPU;

fn set_compare_tests(
    cpu: &mut CPU,
    register_name: &str,
    register_value: u8,
    operand: u8,
    should_negative: bool,
    should_carry: bool,
) {
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
        _ => panic!("Unknown register called {:?}", register_name),
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
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    test_compare(&mut cpu, "A");
}

#[test]
#[allow(non_snake_case)]
fn CPX() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    test_compare(&mut cpu, "X");
}

#[test]
#[allow(non_snake_case)]
fn CPY() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    test_compare(&mut cpu, "Y");
}

fn set_dec_test(cpu: &mut CPU, memory_address: u16, memory_value: u8) {
    cpu.write_memory(memory_address, memory_value);
    cpu.DEC(memory_address);
    assert_eq!(
        cpu.read_memory(memory_address).wrapping_add(1),
        memory_value
    );
    assert_eq!(cpu.get_status_z(), cpu.read_memory(memory_address) == 0);
    assert_eq!(
        cpu.get_status_n(),
        (cpu.read_memory(memory_address) >> 7) & 1 == 1
    );
}

#[test]
#[allow(non_snake_case)]
fn DEC() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    // test for some memory addresses and values:
    for (address, value) in get_random_u8_and_u16_pairs() {
        set_dec_test(&mut cpu, address, value);
    }

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

fn set_inc_test(cpu: &mut CPU, memory_address: u16, memory_value: u8) {
    cpu.write_memory(memory_address, memory_value);
    cpu.INC(memory_address);
    assert_eq!(
        cpu.read_memory(memory_address).wrapping_sub(1),
        memory_value
    );
    assert_eq!(cpu.get_status_z(), cpu.read_memory(memory_address) == 0);
    assert_eq!(
        cpu.get_status_n(),
        (cpu.read_memory(memory_address) >> 7) & 1 == 1
    );
}

#[test]
#[allow(non_snake_case)]
fn INC() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    // test for some memory addresses and values:
    for (address, value) in get_random_u8_and_u16_pairs() {
        set_inc_test(&mut cpu, address, value);
    }

    // check for multiple decreases of the same memory address
    let mem_addr: u16 = 0x57af;
    cpu.write_memory(mem_addr, 0xfe);
    assert_eq!(cpu.read_memory(mem_addr), 0xfe);
    cpu.INC(mem_addr);
    assert_eq!(cpu.read_memory(mem_addr), 0xff);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
    cpu.INC(mem_addr);
    assert_eq!(cpu.read_memory(mem_addr), 0x00);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.INC(mem_addr);
    assert_eq!(cpu.read_memory(mem_addr), 0x01);
    assert!(!cpu.get_status_z());
    assert!(!cpu.get_status_n());
}

fn get_random_u8_values() -> Vec<u8> {
    vec![
        3, 0, 0xFF, 0xFF, 0x84, 0xde, 0x0a, 0x53, 0xde, 0xde, 0x75, 0x27, 0x00, 0, 0x81, 0xff,
        0x00, 0x00, 0xff, 0x2a,
    ]
}

fn set_dex_test(cpu: &mut CPU, register_x_value: u8) {
    cpu.register_x = register_x_value;
    cpu.DEX();
    assert_eq!(cpu.register_x.wrapping_add(1), register_x_value);
    assert_eq!(cpu.get_status_z(), cpu.register_x == 0);
    assert_eq!(cpu.get_status_n(), (cpu.register_x >> 7) & 1 == 1);
}

#[test]
#[allow(non_snake_case)]
fn DEX() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    // test for some  values:
    for value in get_random_u8_values() {
        set_dex_test(&mut cpu, value);
    }

    // check for multiple decreases
    cpu.register_x = 0x01;
    assert_eq!(cpu.register_x, 0x01);
    cpu.DEX();
    assert_eq!(cpu.register_x, 0x00);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.DEX();
    assert_eq!(cpu.register_x, 0xff);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
}

fn set_dey_test(cpu: &mut CPU, register_y_value: u8) {
    cpu.register_y = register_y_value;
    cpu.DEY();
    assert_eq!(cpu.register_y.wrapping_add(1), register_y_value);
    assert_eq!(cpu.get_status_z(), cpu.register_y == 0);
    assert_eq!(cpu.get_status_n(), (cpu.register_y >> 7) & 1 == 1);
}

#[test]
#[allow(non_snake_case)]
fn DEY() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    // test for some  values:
    for value in get_random_u8_values() {
        set_dey_test(&mut cpu, value);
    }

    // check for multiple decreases
    cpu.register_y = 0x01;
    assert_eq!(cpu.register_y, 0x01);
    cpu.DEY();
    assert_eq!(cpu.register_y, 0x00);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.DEY();
    assert_eq!(cpu.register_y, 0xff);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
}

fn set_inx_test(cpu: &mut CPU, register_x_value: u8) {
    cpu.register_x = register_x_value;
    cpu.INX();
    assert_eq!(cpu.register_x.wrapping_sub(1), register_x_value);
    assert_eq!(cpu.get_status_z(), cpu.register_x == 0);
    assert_eq!(cpu.get_status_n(), (cpu.register_x >> 7) & 1 == 1);
}

#[test]
#[allow(non_snake_case)]
fn INX() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    // test for some  values:
    for value in get_random_u8_values() {
        set_inx_test(&mut cpu, value);
    }

    // check for multiple decreases
    cpu.register_x = 0xfe;
    assert_eq!(cpu.register_x, 0xfe);
    cpu.INX();
    assert_eq!(cpu.register_x, 0xff);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
    cpu.INX();
    assert_eq!(cpu.register_x, 0x00);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.INX();
    assert_eq!(cpu.register_x, 0x01);
    assert!(!cpu.get_status_z());
    assert!(!cpu.get_status_n());
}

fn set_iny_test(cpu: &mut CPU, register_y_value: u8) {
    cpu.register_y = register_y_value;
    cpu.INY();
    assert_eq!(cpu.register_y.wrapping_sub(1), register_y_value);
    assert_eq!(cpu.get_status_z(), cpu.register_y == 0);
    assert_eq!(cpu.get_status_n(), (cpu.register_y >> 7) & 1 == 1);
}

#[test]
#[allow(non_snake_case)]
fn INY() {
    let mut bus: Bus = Bus::new();
    let mut cpu: CPU = CPU::new(&mut bus);
    // test for some  values:
    for value in get_random_u8_values() {
        set_iny_test(&mut cpu, value);
    }

    // check for multiple decreases
    cpu.register_y = 0xfe;
    assert_eq!(cpu.register_y, 0xfe);
    cpu.INY();
    assert_eq!(cpu.register_y, 0xff);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
    cpu.INY();
    assert_eq!(cpu.register_y, 0x00);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.INY();
    assert_eq!(cpu.register_y, 0x01);
    assert!(!cpu.get_status_z());
    assert!(!cpu.get_status_n());
}

fn test_load(cpu: &mut CPU, register_name: &str, register_value: u8) {
    match register_name {
        "A" => {
            cpu.LDA(register_value);
            assert_eq!(cpu.register_a, register_value);
            assert_eq!(cpu.register_a == 0, cpu.get_status_z());
            assert_eq!(cpu.register_a >= 0x80, cpu.get_status_n());
        }
        "X" => {
            cpu.LDX(register_value);
            assert_eq!(cpu.register_x, register_value);
            assert_eq!(cpu.register_x == 0, cpu.get_status_z());
            assert_eq!(cpu.register_x >= 0x80, cpu.get_status_n());
        }
        "Y" => {
            cpu.LDY(register_value);
            assert_eq!(cpu.register_y, register_value);
            assert_eq!(cpu.register_y == 0, cpu.get_status_z());
            assert_eq!(cpu.register_y >= 0x80, cpu.get_status_n());
        }
        _ => panic!("Unknown register {:} in load", register_name),
    }
}

#[allow(non_snake_case)]
#[test]
fn LDA() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for value in get_random_u8_values() {
        test_load(&mut cpu, "A", value)
    }
}

#[allow(non_snake_case)]
#[test]
fn LDX() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for value in get_random_u8_values() {
        test_load(&mut cpu, "X", value)
    }
}

#[allow(non_snake_case)]
#[test]
fn LDY() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for value in get_random_u8_values() {
        test_load(&mut cpu, "Y", value)
    }
}

#[allow(non_snake_case)]
#[test]
fn STA() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for (address, value) in get_random_u8_and_u16_pairs() {
        cpu.register_a = value;
        cpu.STA(address);
        assert_eq!(value, cpu.read_memory(address));
    }
}

#[allow(non_snake_case)]
#[test]
fn STX() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for (address, value) in get_random_u8_and_u16_pairs() {
        cpu.register_x = value;
        cpu.STX(address);
        assert_eq!(value, cpu.read_memory(address));
    }
}

#[allow(non_snake_case)]
#[test]
fn STY() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for (address, value) in get_random_u8_and_u16_pairs() {
        cpu.register_y = value;
        cpu.STY(address);
        assert_eq!(value, cpu.read_memory(address));
    }
}

fn prepare_transfer_tests(cpu: &mut CPU, source_name: &str, destination_name: &str, value: u8) {
    let old_flags = cpu.status;
    // set source
    match source_name {
        "A" => cpu.register_a = value,
        "X" => cpu.register_x = value,
        "Y" => cpu.register_y = value,
        "S" => cpu.stack_pointer = value,
        _ => {
            panic!("Unknown source register {:} in transfer", source_name)
        }
    }
    // transfer correct flags
    match (source_name, destination_name) {
        ("A", "X") => cpu.TAX(),
        ("A", "Y") => cpu.TAY(),
        ("S", "X") => cpu.TSX(),
        ("X", "A") => cpu.TXA(),
        ("X", "S") => cpu.TXS(),
        ("Y", "A") => cpu.TYA(),
        _ => panic!(
            "Unknown command T{}{} in tests transfer",
            source_name, destination_name
        ),
    }

    //check target
    match destination_name {
        "A" => {
            assert_eq!(cpu.register_a, value)
        }
        "X" => {
            assert_eq!(cpu.register_x, value)
        }
        "Y" => {
            assert_eq!(cpu.register_y, value)
        }
        "S" => {
            assert_eq!(cpu.stack_pointer, value)
        }
        _ => {
            panic!("Unknown destination register {:} in transfer", source_name)
        }
    }

    //check for flags. should be set unless the source is sp
    if destination_name == "S" {
        assert_eq!(cpu.status, old_flags);
    } else {
        // check we did not change the old flags
        let zero_negative_mask = 0x82_u8;
        assert_eq!(
            cpu.status | zero_negative_mask,
            old_flags | zero_negative_mask
        );
        // check that the zero and negative flags were changed
        assert_eq!(cpu.get_status_n(), value >= 0x80);
        assert_eq!(cpu.get_status_z(), value == 0);
    }
}

#[test]
fn test_transfer() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    for value in get_random_u8_values() {
        prepare_transfer_tests(&mut cpu, "A", "X", value)
    }
    for value in get_random_u8_values() {
        prepare_transfer_tests(&mut cpu, "A", "Y", value)
    }
    for value in get_random_u8_values() {
        prepare_transfer_tests(&mut cpu, "S", "X", value)
    }
    for value in get_random_u8_values() {
        prepare_transfer_tests(&mut cpu, "X", "A", value)
    }
    for value in get_random_u8_values() {
        prepare_transfer_tests(&mut cpu, "X", "S", value)
    }
    for value in get_random_u8_values() {
        prepare_transfer_tests(&mut cpu, "Y", "A", value)
    }
}
