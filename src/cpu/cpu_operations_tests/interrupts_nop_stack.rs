use crate::bus::Bus;
use crate::cpu::CPU;
use crate::generate_cpu;

const STACK_START: u16 = 0x1ff;
const BRK_ADDRESS: u16 = 0xfffe;

#[test]
#[allow(non_snake_case)]
fn PHA_and_PLA() {
    generate_cpu!(cpu);
    cpu.stack_pointer = 0xff;
    let a_values = [0_u8, 0x98, 0xff, 0x98, 0x1d, 0xd1, 0, 0, 0x15];
    for possible_value in a_values {
        cpu.register_a = possible_value;
        cpu.PHA();
    }
    assert_eq!(cpu.stack_pointer, 0xff - a_values.len() as u8);
    for i in 0..a_values.len() {
        let ith_from_end = a_values.len() - 1 - i;
        assert_eq!(
            cpu.read_memory(STACK_START - ith_from_end as u16),
            a_values[ith_from_end]
        );
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
    generate_cpu!(cpu);
    cpu.stack_pointer = 0xff;
    let status_values = [0_u8, 0x98, 0xff, 0x98, 0x1d, 0xd1, 0, 0, 0x15];
    for possible_value in status_values {
        cpu.status = possible_value;
        cpu.PHP();
    }
    assert_eq!(cpu.stack_pointer, 0xff - status_values.len() as u8);
    for i in 0..status_values.len() {
        let ith_from_end = status_values.len() - 1 - i;
        assert_eq!(
            cpu.read_memory(STACK_START - ith_from_end as u16),
            status_values[ith_from_end] | 0x30 // the break flag should always be set (also bit 6)!
        );
        let old_bits_5_6_values = cpu.status & 0x30;
        cpu.PLP();
        let new_bits_5_6_values = cpu.status & 0x30;
        assert_eq!(cpu.status | 0x30, status_values[ith_from_end] | 0x30);
        assert_eq!(old_bits_5_6_values, new_bits_5_6_values);
    }
    assert_eq!(cpu.stack_pointer, 0xff);
}

fn check_cpu_not_changed(cpu: &CPU) {
    assert_eq!(*cpu.bus, Bus::new());
    assert_eq!(cpu.register_a, 0);
    assert_eq!(cpu.register_x, 0);
    assert_eq!(cpu.register_y, 0);
    assert_eq!(cpu.status, 0);
}

#[test]
#[allow(non_snake_case)]
fn NOP() {
    generate_cpu!(cpu);
    cpu.NOP();
    check_cpu_not_changed(&cpu);
}

#[test]
#[allow(non_snake_case)]
fn RTI() {
    generate_cpu!(cpu);
    cpu.stack_push_u16(0x58fa);
    cpu.stack_push(0xff);
    cpu.RTI();
    assert_eq!(cpu.program_counter, 0x58fa);
    assert_eq!(cpu.status, 0xcf); //ignore bits 4 and 5!
    cpu.status = 0xff;
    cpu.stack_push_u16(0x1f92);
    cpu.stack_push(0x00);
    cpu.RTI();
    assert_eq!(cpu.program_counter, 0x1f92);
    assert_eq!(cpu.status, 0x30); //ignore bits 4 and 5!
}

#[test]
#[allow(non_snake_case)]
fn BRK() {
    generate_cpu!(cpu);
    // prepare the prg rom
    let mut program_vector = vec![0_u8; 0x8000];
    program_vector[BRK_ADDRESS as usize - 0x8000] = 0xaa;
    program_vector[BRK_ADDRESS as usize + 1 - 0x8000] = 0xbb;
    cpu.bus.cartridge.raw_load(program_vector);

    cpu.BRK();

    assert_eq!(cpu.program_counter, 0xbbaa);
    cpu.program_counter = 0x800;
    cpu.write_memory(0x800, 0);
    assert!(!cpu.massive_switch(0x00)); // assert the massive switch returns false
}
