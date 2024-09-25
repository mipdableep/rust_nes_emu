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

#[test]
#[allow(non_snake_case)]
fn CLI_and_SEI() {
    let mut cpu = CPU::new();
    cpu.SEI();
    assert!(cpu.get_status_i());
    cpu.CLI();
    assert!(!cpu.get_status_i());
    cpu.SEI();
    assert!(cpu.get_status_i());
    cpu.CLI();
    assert!(!cpu.get_status_i());
}

#[test]
#[allow(non_snake_case)]
fn CLV() {
    let mut cpu = CPU::new();
    cpu.set_overflow(true);
    assert!(cpu.get_status_v());
    cpu.CLV();
    assert!(!cpu.get_status_v());
    cpu.set_overflow(true);
    assert!(cpu.get_status_v());
    cpu.CLV();
    assert!(!cpu.get_status_v());
}
