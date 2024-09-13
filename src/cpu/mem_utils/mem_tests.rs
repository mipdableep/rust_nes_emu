use super::{AddressingMode, CPU};

#[test]
fn mode_to_mem_addr_immediate() {
    let program = vec![5, 25, 3];
    let mut cpu = CPU::new();
    cpu.load(program.clone());

    assert_eq!(
        program[0],
        cpu.convert_mode_to_val(AddressingMode::Immediate)
    );
    cpu.program_counter += 1;

    assert_eq!(
        program[1],
        cpu.convert_mode_to_val(AddressingMode::Immediate)
    );
    cpu.program_counter += 1;

    assert_eq!(
        program[2],
        cpu.convert_mode_to_val(AddressingMode::Immediate)
    );
}

#[test]
fn mode_to_mem_addr_zeropage() {
    let mut cpu = CPU::new();
    cpu.memory[11] = 24;
    cpu.memory[14] = 21;
    cpu.memory[20] = 15;

    let program = vec![11, 14, 20];
    cpu.load(program);

    assert_eq!(24, cpu.convert_mode_to_val(AddressingMode::ZeroPage));
    cpu.program_counter += 1;

    assert_eq!(21, cpu.convert_mode_to_val(AddressingMode::ZeroPage));
    cpu.program_counter += 1;

    assert_eq!(15, cpu.convert_mode_to_val(AddressingMode::ZeroPage));
    cpu.program_counter += 1;
}

#[test]
fn mode_to_mem_zeropage_x() {
    let mut cpu = CPU::new();
    cpu.memory[11] = 24;
    cpu.memory[13] = 21;
    cpu.memory[20] = 15;
    cpu.register_x = 1;

    let program = vec![10, 15, 17];
    cpu.load(program);

    assert_eq!(24, cpu.convert_mode_to_val(AddressingMode::ZeroPage_X));
    cpu.program_counter += 1;
    // test the wraping
    cpu.register_x = 254;
    assert_eq!(21, cpu.convert_mode_to_val(AddressingMode::ZeroPage_X));
    cpu.program_counter += 1;

    cpu.register_x = 3;
    assert_eq!(15, cpu.convert_mode_to_val(AddressingMode::ZeroPage_X));
    cpu.program_counter += 1;
}

#[test]
fn mode_to_mem_zeropage_y() {
    let mut cpu = CPU::new();
    cpu.memory[11] = 24;
    cpu.memory[13] = 21;
    cpu.memory[20] = 15;
    cpu.register_y = 1;

    let program = vec![10, 15, 17];
    cpu.load(program);

    assert_eq!(24, cpu.convert_mode_to_val(AddressingMode::ZeroPage_Y));
    cpu.program_counter += 1;
    // test the wraping
    cpu.register_y = 254;
    assert_eq!(21, cpu.convert_mode_to_val(AddressingMode::ZeroPage_Y));
    cpu.program_counter += 1;

    cpu.register_y = 3;
    assert_eq!(15, cpu.convert_mode_to_val(AddressingMode::ZeroPage_Y));
    cpu.program_counter += 1;
}
