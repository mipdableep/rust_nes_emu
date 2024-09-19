use crate::cpu::CPU;

#[test]
#[allow(non_snake_case)]
fn CLC_and_SEC() {
    let mut cpu = CPU::new();
    cpu.SEC();
    assert!(cpu.get_status_c());
    cpu.CLC();
    assert!(!cpu.get_status_c());
    cpu.SEC();
    assert!(cpu.get_status_c());
    cpu.CLC();
    assert!(!cpu.get_status_c());
}