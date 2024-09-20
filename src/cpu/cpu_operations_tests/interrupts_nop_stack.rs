use crate::cpu::CPU;

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
    let mut cpu:CPU = CPU::new();
    cpu.NOP();
    check_cpu_not_changed(&cpu);
}
