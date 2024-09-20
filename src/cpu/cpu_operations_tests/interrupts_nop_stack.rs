use crate::cpu::CPU;

const STACK_START: u16 = 0x1ff;

#[test]
#[allow(non_snake_case)]
fn PHA_and_PLA() {
    let mut cpu = CPU::new();
    cpu.stack_pointer = 0xff;
    let a_values = [0_u8, 0x98, 0xff, 0x98, 0x1d, 0xd1, 0, 0, 0x15];
    for possible_value in a_values {
        cpu.register_a = possible_value;
        cpu.PHA();
    }
    assert_eq!(cpu.stack_pointer, 0xff - a_values.len() as u8);
    for i in 0..a_values.len() {
        let ith_from_end = a_values.len() - 1 - i;
        assert_eq!(cpu.read_memory(STACK_START - ith_from_end as u16), a_values[ith_from_end]);
        cpu.PLA();
        assert_eq!(cpu.register_a, a_values[ith_from_end]);
        assert_eq!(cpu.get_status_z(), a_values[ith_from_end] == 0);
        assert_eq!(cpu.get_status_n(), a_values[ith_from_end] >= 0x80);
    }
    assert_eq!(cpu.stack_pointer, 0xff);
}


#[test]
#[allow(non_snake_case)]
fn PHP_and_PLP() {
    let mut cpu = CPU::new();
    cpu.stack_pointer = 0xff;
    let status_values = [0_u8, 0x98, 0xff, 0x98, 0x1d, 0xd1, 0, 0, 0x15];
    for possible_value in status_values {
        cpu.status = possible_value;
        cpu.PHP();
    }
    assert_eq!(cpu.stack_pointer, 0xff - status_values.len() as u8);
    for i in 0..status_values.len() {
        let ith_from_end = status_values.len() - 1 - i;
        assert_eq!(cpu.read_memory(STACK_START - ith_from_end as u16), status_values[ith_from_end]);
        cpu.PLP();
        assert_eq!(cpu.status, status_values[ith_from_end]);
    }
    assert_eq!(cpu.stack_pointer, 0xff);
}


fn check_cpu_not_changed(cpu: &CPU) {
    assert_eq!(cpu.memory, [0; 0x10000]);
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.status, 0);
}

#[test]
#[allow(non_snake_case)]
fn NOP() {
    let mut cpu: CPU = CPU::new();
    cpu.NOP();
    check_cpu_not_changed(&cpu);
}

#[test]
#[allow(non_snake_case)]
fn BRK() {
    let mut cpu: CPU = CPU::new();
    cpu.BRK();
    assert!(!cpu.massive_switch(0x00, &vec!(0_u8))); // assert the massive switch returns false
}