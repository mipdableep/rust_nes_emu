use super::CPU;

#[test]
fn test_new() {
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
fn panic_test_new() {
    let c = CPU::new();
    assert_eq!(c.stack_pointer, 254);
}

#[test]
fn test_get_s_n() {
    let n_true = 0b10010100;
    let n_false = 0b00010100;
    let mut c = CPU::new();
    c.status = n_true;
    assert!(c.get_status_n());
    c.status = n_false;
    assert!(!c.get_status_n());
}
