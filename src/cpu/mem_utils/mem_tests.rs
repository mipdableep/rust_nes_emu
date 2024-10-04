use super::{AddressingMode, CPU};
use rand_chacha::rand_core::{RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn get_zeropage_from_seed(seed: u8) -> [u8; 256] {
    let mut generator = ChaCha12Rng::from_seed([seed; 32]);
    let mut result: [u8; 1 << 8] = [0; 1 << 8];
    for i in 0..0xFF {
        result[i] = generator.next_u32() as u8;
    }
    result
}

#[test]
fn test_read_memory_zeropage() {
    let mut cpu = CPU::new();
    let memory_contents = get_zeropage_from_seed(42);
    for i in 0..=0xff {
        cpu.write_memory(i, memory_contents[i as usize]);
    }
    for i in 0..=0xff {
        assert_eq!(cpu.read_memory(i), memory_contents[i as usize]);
    }
}

#[test]
fn test_write_memory_zeropage() {
    let mut cpu = CPU::new();
    let memory_contents = get_zeropage_from_seed(42);
    for i in 0..=0xff {
        cpu.write_memory(i, memory_contents[i as usize]);
    }
    for i in 0..=0xff {
        assert_eq!(cpu.read_memory(i), memory_contents[i as usize]);
    }
}

#[test]
fn test_read_memory_2_bytes() {
    let mut cpu = CPU::new();
    let memory_contents: [u8; 0xff + 1] = get_zeropage_from_seed(42);
    for i in 0..=0xff {
        cpu.write_memory(i, memory_contents[i as usize]);
    }
    for i in 0u8..=0x70 {
        // 6502 is little endian
        let expected_result: u16 = ((memory_contents[(2 * i + 1) as usize] as u16) << 8)
            + (memory_contents[(2 * i) as usize] as u16);
        assert_eq!(cpu.read_memory_2_bytes(i as u16 * 2), expected_result);
    }
}

#[test]
#[should_panic]
fn test_memory_violation() {
    let cpu = CPU::new();
    cpu.read_memory_2_bytes(0xffff);
}

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
    cpu.write_memory(11, 24);
    cpu.write_memory(13, 21);
    cpu.write_memory(20, 15);

    let program = vec![11, 13, 20];
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
    cpu.write_memory(11, 24);
    cpu.write_memory(13, 21);
    cpu.write_memory(20, 15);
    cpu.register_x = 1;

    let program = vec![10, 15, 17];
    cpu.load(program);

    assert_eq!(24, cpu.convert_mode_to_val(AddressingMode::ZeroPage_X));
    cpu.program_counter += 1;
    // test the wrapping
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
    cpu.write_memory(11, 24);
    cpu.write_memory(13, 21);
    cpu.write_memory(20, 15);
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

#[test]
fn mode_to_mem_relative() {
    let mut cpu = CPU::new();
    cpu.load(vec![15]);
    let pc = cpu.program_counter;
    assert_eq!(
        pc + 2 + 15,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Relative)
    );

    cpu.load(vec![0b0100_0000]);
    let pc = cpu.program_counter;
    assert_eq!(
        pc + 2 + 0b0100_0000,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Relative)
    );

    cpu.load(vec![0b1000_0000]);
    let pc = cpu.program_counter;
    assert_eq!(
        pc + 2 - 128,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Relative)
    );

    cpu.load(vec![255 - 73]);
    let pc = cpu.program_counter;
    assert_eq!(
        pc + 2 - 74,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Relative)
    );
}

#[test]
fn mode_to_mem_absolute() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xab, 0x12, 0xfa, 0x75]);

    cpu.write_memory(0x12ab, 11);
    assert_eq!(11, cpu.convert_mode_to_val(AddressingMode::Absolute));

    cpu.program_counter += 2;
    cpu.write_memory(0x75fa, 22);
    assert_eq!(22, cpu.convert_mode_to_val(AddressingMode::Absolute));
}

#[test]
fn mode_to_mem_absolute_x() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xab, 0x12, 0xfa, 0x75]);
    cpu.register_x = 48;

    cpu.write_memory(0x12ab + 48, 11);
    assert_eq!(11, cpu.convert_mode_to_val(AddressingMode::Absolute_X));

    cpu.program_counter += 2;
    cpu.register_x = 183;
    cpu.write_memory(0x75fa + 183, 22);
    assert_eq!(22, cpu.convert_mode_to_val(AddressingMode::Absolute_X));
}

#[test]
fn mode_to_mem_absolute_y() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xab, 0x12, 0xfa, 0x75]);
    cpu.register_y = 48;

    cpu.write_memory(0x12ab + 48, 11);
    assert_eq!(11, cpu.convert_mode_to_val(AddressingMode::Absolute_Y));

    cpu.program_counter += 2;
    cpu.register_y = 183;
    cpu.write_memory(0x75fa + 183, 22);
    assert_eq!(22, cpu.convert_mode_to_val(AddressingMode::Absolute_Y));
}

#[test]
fn mode_to_mem_indirect() {
    let mut cpu = CPU::new();
    cpu.load(vec![0xab, 0x12, 0xfa, 0x75, 0xff, 0x5d]);

    cpu.write_memory(0x12ab_u16, 0x44);
    cpu.write_memory(0x12ab_u16 + 1, 0x66);
    cpu.write_memory(0x6644_u16, 11);
    assert_eq!(11, cpu.convert_mode_to_val(AddressingMode::Indirect));

    cpu.program_counter += 2;
    cpu.write_memory(0x75fa_u16, 0x8f);
    cpu.write_memory(0x75fa_u16 + 1, 0x4d);
    cpu.write_memory(0x4d8f_u16, 0x8f);
    assert_eq!(0x8f, cpu.convert_mode_to_val(AddressingMode::Indirect));

    cpu.program_counter += 2; // check that we don't cross the page boundaries
    cpu.write_memory(0x5dff_u16, 0x11);
    cpu.write_memory(0x5d00_u16, 0x00);
    cpu.write_memory(0x5e00_u16, 0x33); // should not be accessed!
    cpu.write_memory(0x0011_u16, 0xf9);
    cpu.write_memory(0x3311_u16, 0x03); // should not be accessed!

    assert_eq!(0xf9, cpu.convert_mode_to_val(AddressingMode::Indirect));
}

#[test]
fn mod_to_mem_indirect_x() {
    // based on http://www.emulator101.com/6502-addressing-modes.html
    let mut cpu = CPU::new();
    // test without wrapping
    cpu.register_x = 0x04;
    cpu.write_memory(0x24, 0x74);
    cpu.write_memory(0x25, 0x20);
    cpu.write_memory(0x2074, 0x5a);
    cpu.write_memory(0x2075, 0xbb);
    cpu.load(vec![0x20]); // 0x20 + 0x04 = 0x24
    assert_eq!(
        0x2074,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Indirect_X)
    );
    assert_eq!(0x5a, cpu.convert_mode_to_val(AddressingMode::Indirect_X));
    // test with overflow
    cpu.register_x = 0x5a;
    cpu.write_memory(0x13, 0xbb);
    cpu.write_memory(0x14, 0x11);
    cpu.write_memory(0x11bb, 0x60);
    cpu.load(vec![0xb9]); // 0xb9 + 0x5a = 0x113
    assert_eq!(
        0x11bb,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Indirect_X)
    );
    assert_eq!(0x60, cpu.convert_mode_to_val(AddressingMode::Indirect_X));
    // test with overflow on page zero
    cpu.register_x = 0x77;
    cpu.write_memory(0xff, 0x11);
    cpu.write_memory(0x00, 0x22);
    cpu.load(vec![0x88]); // 0x88 + 0x77 = 0xff
    assert_eq!(
        0x2211,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Indirect_X)
    );
}

#[test]
fn mod_to_mem_indirect_y() {
    // based on http://www.emulator101.com/6502-addressing-modes.html
    let mut cpu = CPU::new();
    // test without wrapping
    cpu.register_y = 0x10;
    cpu.write_memory(0x86, 0x28);
    cpu.write_memory(0x87, 0x40);
    cpu.write_memory(0x4038, 0x5a); // 0X4028 + 0X10 = 0X4038
    cpu.load(vec![0x86]);
    assert_eq!(
        0x4038,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Indirect_Y)
    );
    assert_eq!(0x5a, cpu.convert_mode_to_val(AddressingMode::Indirect_Y));
    // test with overflow around zero page, that should not occur
    cpu.register_y = 0x01;
    cpu.write_memory(0x13, 0xff);
    cpu.write_memory(0x14, 0x00);
    cpu.write_memory(0x0100, 0x60);
    cpu.load(vec![0x13]); // 0xb9 + 0x5a = 0x113
    assert_eq!(
        0x0100,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Indirect_Y)
    );
    assert_eq!(0x60, cpu.convert_mode_to_val(AddressingMode::Indirect_Y));
    // test with overflow around all the pages, which should occur
    cpu.register_y = 0xb9;
    cpu.write_memory(0x86, 0x5a);
    cpu.write_memory(0x87, 0xff); // 0xff5a + 0xb9 = 0x10013
    cpu.write_memory(0x13, 0x5a);
    cpu.load(vec![0x86]);
    assert_eq!(
        0x13,
        cpu.convert_mode_to_operand_mem_address(AddressingMode::Indirect_Y)
    );
    assert_eq!(0x5a, cpu.convert_mode_to_val(AddressingMode::Indirect_Y));
}
