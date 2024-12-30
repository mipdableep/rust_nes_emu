use crate::generate_cpu;

#[test]
#[allow(non_snake_case)]
fn BCC_and_BCS() {
    generate_cpu!(cpu);
    cpu.program_counter = 0x800;
    cpu.set_carry(true);
    cpu.BCS(0x7ff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BCC(0x5a);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.set_carry(false);
    cpu.BCS(0xff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BCC(0x859);
    assert_eq!(cpu.program_counter, 0x859);
}

#[test]
#[allow(non_snake_case)]
fn BEQ_and_BNQ() {
    generate_cpu!(cpu);
    cpu.program_counter = 0x800;
    cpu.set_zero(true);
    cpu.BEQ(0x7ff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BNE(0x5a);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.set_zero(false);
    cpu.BEQ(0xff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BNE(0x859);
    assert_eq!(cpu.program_counter, 0x859);
}

#[test]
#[allow(non_snake_case)]
fn BMI_and_BPL() {
    generate_cpu!(cpu);
    cpu.program_counter = 0x800;
    cpu.set_negative(true);
    cpu.BMI(0x7ff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BPL(0x5a);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.set_negative(false);
    cpu.BMI(0xff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BPL(0x859);
    assert_eq!(cpu.program_counter, 0x859);
}

#[test]
#[allow(non_snake_case)]
fn BVC_and_BVS() {
    generate_cpu!(cpu);
    cpu.program_counter = 0x800;
    cpu.set_overflow(true);
    cpu.BVS(0x7ff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BVC(0x5a);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.set_overflow(false);
    cpu.BVS(0xff);
    assert_eq!(cpu.program_counter, 0x7ff);
    cpu.BNE(0x859);
    assert_eq!(cpu.program_counter, 0x859);
}

#[allow(non_snake_case)]
#[test]
fn JMP() {
    generate_cpu!(cpu);
    cpu.JMP(0x31a9);
    assert_eq!(cpu.program_counter, 0x31a9);
    cpu.JMP(0x15ff);
    assert_eq!(cpu.program_counter, 0x15ff);
}

#[allow(non_snake_case)]
#[test]
fn JSR() {
    generate_cpu!(cpu);
    cpu.JSR(0x31a9);
    assert_eq!(cpu.program_counter, 0x31a9);
    cpu.JSR(0x15ff);
    assert_eq!(cpu.program_counter, 0x15ff);
    assert_eq!(cpu.stack_pull_u16(), 0x31a8); // should push pc-1
}

#[allow(non_snake_case)]
#[test]
fn RTS() {
    generate_cpu!(cpu);
    cpu.stack_pointer = 0xff;
    cpu.program_counter = 0x15df;
    cpu.JSR(0x41f5);
    assert_eq!(cpu.stack_pointer, 0xfd);
    cpu.RTS();
    assert_eq!(cpu.program_counter, 0x15df); // JSR push pc-1, RTS adds 1
    assert_eq!(cpu.stack_pointer, 0xff);
}
