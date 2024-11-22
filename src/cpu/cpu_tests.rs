use super::CPU;
use crate::bus::Bus;
use crate::generate_cpu;

#[test]
#[allow(unused_mut)]
fn new() {
    generate_cpu!(c);
    assert_eq!(c.register_a, 0);
    assert_eq!(c.status, 0);
    assert_eq!(c.program_counter, 0);
    assert_eq!(c.register_x, 0);
    assert_eq!(c.register_y, 0);
    assert_eq!(c.stack_pointer, 0xff);
}

#[test]
#[allow(unused_mut)]
#[should_panic]
fn panic_new() {
    generate_cpu!(cpu);
    assert_eq!(cpu.stack_pointer, 254);
}

#[test]
fn get_status_n() {
    let n_true = 0b10000000;
    let n_false = 0b01111111;
    generate_cpu!(cpu);
    cpu.status = n_true;
    assert!(cpu.get_status_n());
    cpu.status = n_false;
    assert!(!cpu.get_status_n());
}
#[test]
fn set_status_n() {
    generate_cpu!(cpu);
    assert!(!cpu.get_status_n());
    cpu.set_negative(true);
    assert!(cpu.get_status_n());
    cpu.set_negative(false);
    assert!(!cpu.get_status_n());
}

#[test]
fn get_status_v() {
    let v_true = 0b01000000;
    let v_false = 0b10111111;
    generate_cpu!(cpu);
    cpu.status = v_true;
    assert!(cpu.get_status_v());
    cpu.status = v_false;
    assert!(!cpu.get_status_v());
}
#[test]
fn set_status_v() {
    generate_cpu!(cpu);
    assert!(!cpu.get_status_v());
    cpu.set_overflow(true);
    assert!(cpu.get_status_v());
    cpu.set_overflow(false);
    assert!(!cpu.get_status_v());
}

#[test]
fn get_status_b() {
    let b_true = 0b00010000;
    let b_false = 0b11101111;
    generate_cpu!(cpu);
    cpu.status = b_true;
    assert!(cpu.get_status_b());
    cpu.status = b_false;
    assert!(!cpu.get_status_b());
}

#[test]
fn get_status_d() {
    let d_true = 0b00001000;
    let d_false = 0b11110111;
    generate_cpu!(cpu);
    cpu.status = d_true;
    assert!(cpu.get_status_d());
    cpu.status = d_false;
    assert!(!cpu.get_status_d());
}
#[test]
fn set_status_d() {
    generate_cpu!(cpu);
    assert!(!cpu.get_status_d());
    cpu.set_decimal(true);
    assert!(cpu.get_status_d());
    cpu.set_decimal(false);
    assert!(!cpu.get_status_d());
}

#[test]
fn get_status_i() {
    let i_true = 0b00000100;
    let i_false = 0b11111011;
    generate_cpu!(cpu);
    cpu.status = i_true;
    assert!(cpu.get_status_i());
    cpu.status = i_false;
    assert!(!cpu.get_status_i());
}
#[test]
fn set_status_i() {
    generate_cpu!(cpu);
    assert!(!cpu.get_status_i());
    cpu.set_interrupt(true);
    assert!(cpu.get_status_i());
    cpu.set_interrupt(false);
    assert!(!cpu.get_status_i());
}

#[test]
fn get_status_z() {
    let z_true = 0b00000010;
    let z_false = 0b11111101;
    generate_cpu!(cpu);
    cpu.status = z_true;
    assert!(cpu.get_status_z());
    cpu.status = z_false;
    assert!(!cpu.get_status_z());
}
#[test]
fn set_status_z() {
    generate_cpu!(cpu);
    assert!(!cpu.get_status_z());
    cpu.set_zero(true);
    assert!(cpu.get_status_z());
    cpu.set_zero(false);
    assert!(!cpu.get_status_z());
}

#[test]
fn get_status_c() {
    let c_true = 0b00000001;
    let c_false = 0b11111110;
    generate_cpu!(cpu);
    cpu.status = c_true;
    assert!(cpu.get_status_c());
    cpu.status = c_false;
    assert!(!cpu.get_status_c());
}

#[test]
fn set_status_c() {
    generate_cpu!(cpu);
    assert!(!cpu.get_status_c());
    cpu.set_carry(true);
    assert!(cpu.get_status_c());
    cpu.set_carry(false);
    assert!(!cpu.get_status_c());
}

#[test]
fn set_negative_and_zero() {
    generate_cpu!(cpu);
    cpu.set_zero_and_negative_flag(0);
    assert!(cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.set_zero_and_negative_flag(1);
    assert!(!cpu.get_status_z());
    assert!(!cpu.get_status_n());
    cpu.set_zero_and_negative_flag(0x80);
    assert!(!cpu.get_status_z());
    assert!(cpu.get_status_n());
}

#[test]
fn stack_u8() {
    generate_cpu!(cpu);
    let values = [0_u8, 0x98, 0xff, 0x98, 0x1d, 0xd1, 0, 0, 0x15];
    for value in values {
        cpu.stack_push(value);
    }
    let mut i: usize = values.len();
    while i > 1 {
        i -= 1;
        assert_eq!(cpu.stack_pull(), values[i]);
    }
}

#[test]
fn stack_u16() {
    generate_cpu!(cpu);
    let values = [
        0_u16, 0xad98, 0xffff, 0x4d98, 0x1dd1, 0xd11d, 0, 0xd11d, 0x15,
    ];
    for value in values {
        cpu.stack_push_u16(value);
    }
    let mut i: usize = values.len();
    while i > 1 {
        i -= 1;
        assert_eq!(cpu.stack_pull_u16(), values[i]);
    }
}

#[test]
fn nmi_attendance() {
    generate_cpu!(cpu);
    // this test flow is:
    // prepare memory -> trigger nmi (the address will only have RTI) -> run cpu cycle
    // -> check we jumped to the correct memory location -> check we returned just fine
    cpu.status = 0x10;
    cpu.program_counter = 0x13;

    // write 0050 to 0xFFA
    // we don't use the normal write function since this address is in the ROM
    let mut rom = vec![0_u8; 0x8000];
    rom[0x7FFA] = 0x50;
    rom[0x7FFB] = 0x00;
    cpu.bus.as_mut().unwrap().cartridge.prg_rom = rom;
    cpu.write_memory(0x50, 0x40); // 0x40 is RTI
    cpu.bus.as_mut().unwrap().nmi_generated = true;
    cpu.run_one_cycle();

    // check we did all nmi things correctly
    assert_eq!(cpu.program_counter, 0x50); // jumped to the right location

    // check that the two values in the stack are correct
    // the top should be sp
    let stack_top = cpu.stack_pull();
    assert_eq!(stack_top, 0x20); // should set the non-existing bit (5), and write b flag as 0
    let old_pc = cpu.stack_pull_u16();
    assert_eq!(old_pc, 0x13);
    //don't forget to return them to the stack!
    cpu.stack_push_u16(old_pc);
    cpu.stack_push(stack_top);

    // set the status to check that we changed it correctly while pulling back from the stack
    cpu.status = 0xFF;

    while cpu.bus.as_mut().unwrap().cpu_idle_cycles > 0 {
        cpu.run_one_cycle();
    }

    // now check the return with RTi
    cpu.run_one_cycle();
    assert_eq!(cpu.program_counter, 0x13);
    assert_eq!(cpu.status, 0x30);
    assert_eq!(cpu.stack_pointer, 0xFF);
}
