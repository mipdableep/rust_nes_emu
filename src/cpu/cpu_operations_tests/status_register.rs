use crate::bus::Bus;
use crate::cpu::CPU;
use crate::generate_cpu;

fn get_random_status_flag_values() -> Vec<u8> {
    vec![3, 0, 0xFF, 0x84, 0xde, 0x0a, 0x53, 0x75, 0x27, 0x81, 0x2a]
}

fn assert_mostly_unchanged(old: u8, new: u8, mask: u8) {
    // this function check that the new and old values are the same, up to the mask.
    // the mask should be 1 in places we want to ignore, and 0 otherwise
    assert_eq!(old | mask, new | mask)
}

#[test]
#[allow(non_snake_case)]
fn CLC_and_SEC() {
    generate_cpu!(cpu);
    for p in get_random_status_flag_values() {
        cpu.status = p;
        cpu.SEC();
        assert!(cpu.get_status_c());
        assert_mostly_unchanged(p, cpu.status, 0x01);
        cpu.CLC();
        assert!(!cpu.get_status_c());
        assert_mostly_unchanged(p, cpu.status, 0x01);
        cpu.SEC();
        assert!(cpu.get_status_c());
        assert_mostly_unchanged(p, cpu.status, 0x01);
        cpu.CLC();
        assert!(!cpu.get_status_c());
        assert_mostly_unchanged(p, cpu.status, 0x01);
    }
}

#[test]
#[allow(non_snake_case)]
fn CLD_and_SED() {
    generate_cpu!(cpu);
    for p in get_random_status_flag_values() {
        cpu.status = p;
        cpu.SED();
        assert!(cpu.get_status_d());
        assert_mostly_unchanged(p, cpu.status, 0x08);
        cpu.CLD();
        assert!(!cpu.get_status_d());
        assert_mostly_unchanged(p, cpu.status, 0x08);
        cpu.SED();
        assert!(cpu.get_status_d());
        assert_mostly_unchanged(p, cpu.status, 0x08);
        cpu.CLD();
        assert!(!cpu.get_status_d());
        assert_mostly_unchanged(p, cpu.status, 0x08);
    }
}

#[test]
#[allow(non_snake_case)]
fn CLI_and_SEI() {
    generate_cpu!(cpu);
    for p in get_random_status_flag_values() {
        cpu.status = p;
        cpu.SEI();
        assert!(cpu.get_status_i());
        assert_mostly_unchanged(p, cpu.status, 0x04);
        cpu.CLI();
        assert!(!cpu.get_status_i());
        assert_mostly_unchanged(p, cpu.status, 0x04);
        cpu.SEI();
        assert!(cpu.get_status_i());
        assert_mostly_unchanged(p, cpu.status, 0x04);
        cpu.CLI();
        assert!(!cpu.get_status_i());
        assert_mostly_unchanged(p, cpu.status, 0x04);
    }
}

#[test]
#[allow(non_snake_case)]
fn CLV() {
    generate_cpu!(cpu);
    for p in get_random_status_flag_values() {
        cpu.status = p;
        cpu.set_overflow(true);
        assert!(cpu.get_status_v());
        assert_mostly_unchanged(p, cpu.status, 0x40);
        cpu.CLV();
        assert!(!cpu.get_status_v());
        assert_mostly_unchanged(p, cpu.status, 0x40);
        cpu.set_overflow(true);
        assert!(cpu.get_status_v());
        assert_mostly_unchanged(p, cpu.status, 0x40);
        cpu.CLV();
        assert!(!cpu.get_status_v());
        assert_mostly_unchanged(p, cpu.status, 0x40);
    }
}
