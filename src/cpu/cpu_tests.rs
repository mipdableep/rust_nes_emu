use super::CPU;

#[test]
fn new() {
    let c = CPU::new();
    assert_eq!(c.register_a, 0);
    assert_eq!(c.status, 0);
    assert_eq!(c.program_counter, 0);
    assert_eq!(c.register_x, 0);
    assert_eq!(c.register_y, 0);
    assert_eq!(c.stack_pointer, 0);
    assert_eq!(c.memory, [0; 0xFFFF]);
}

#[test]
#[should_panic]
fn panic_new() {
    let c = CPU::new();
    assert_eq!(c.stack_pointer, 254);
}

#[test]
fn get_status_n() {
    let n_true = 0b10000000;
    let n_false = 0b01111111;
    let mut cpu = CPU::new();
    cpu.status = n_true;
    assert!(cpu.get_status_n());
    cpu.status = n_false;
    assert!(!cpu.get_status_n());
}

#[test]
fn get_status_v() {
    let v_true  = 0b01000000;
    let v_false = 0b10111111;
    let mut cpu = CPU::new();
    cpu.status = v_true;
    assert!(cpu.get_status_v());
    cpu.status = v_false;
    assert!(!cpu.get_status_v());
}


#[test]
fn get_status_b() {
    let b_true  = 0b00010000;
    let b_false = 0b11101111;
    let mut cpu = CPU::new();
    cpu.status = b_true;
    assert!(cpu.get_status_b());
    cpu.status = b_false;
    assert!(!cpu.get_status_b());
}

#[test]
fn get_status_d() {
    let d_true  = 0b00001000;
    let d_false = 0b11110111;
    let mut cpu = CPU::new();
    cpu.status = d_true;
    assert!(cpu.get_status_d());
    cpu.status = d_false;
    assert!(!cpu.get_status_d());
}

#[test]
fn get_status_i() {
    let i_true  = 0b00000100;
    let i_false = 0b11111011;
    let mut cpu = CPU::new();
    cpu.status = i_true;
    assert!(cpu.get_status_i());
    cpu.status = i_false;
    assert!(!cpu.get_status_i());
}

#[test]
fn get_status_z() {
    let z_true  = 0b00000010;
    let z_false = 0b11111101;
    let mut cpu = CPU::new();
    cpu.status = z_true;
    assert!(cpu.get_status_z());
    cpu.status = z_false;
    assert!(!cpu.get_status_z());
}

#[test]
fn get_status_c() {
    let c_true  = 0b00000001;
    let c_false = 0b11111110;
    let mut cpu = CPU::new();
    cpu.status = c_true;
    assert!(cpu.get_status_c());
    cpu.status = c_false;
    assert!(!cpu.get_status_c());
}