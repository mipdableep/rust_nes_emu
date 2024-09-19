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

#[test]
#[allow(non_snake_case)]
fn CLD_and_SED() {
    let mut cpu = CPU::new();
    cpu.SED();
    assert!(cpu.get_status_d());
    cpu.CLD();
    assert!(!cpu.get_status_d());
    cpu.SED();
    assert!(cpu.get_status_d());
    cpu.CLD();
    assert!(!cpu.get_status_d());
}