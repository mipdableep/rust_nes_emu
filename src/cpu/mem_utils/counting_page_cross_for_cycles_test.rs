use super::super::mem_utils::check_if_on_different_pages;
use super::super::CPU;
use crate::bus::Bus;
use crate::cpu::mem_utils::AddressingMode;

#[test]
fn test_page_cross() {
    assert!(check_if_on_different_pages(0xff00, 0x0000));
    assert!(check_if_on_different_pages(0xffff, 0x0000));
    assert!(check_if_on_different_pages(0x1234, 0x1334));
    assert!(!check_if_on_different_pages(0x1234, 0x1235));
    assert!(!check_if_on_different_pages(0x1200, 0x12ff));
    assert!(!check_if_on_different_pages(0x0000, 0x00ff));
    assert!(!check_if_on_different_pages(0xffff, 0xff00));
}

#[test]
fn test_page_cross_detection_absolute_x() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    let addressing_mode = AddressingMode::Absolute_X;

    cpu.write_memory(cpu.program_counter, 0xff);
    cpu.write_memory(cpu.program_counter + 1, 0x00);
    cpu.register_x = 0x34; // 0xff + 0x34 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.program_counter = 0x100;
    cpu.write_memory(cpu.program_counter, 0x89);
    cpu.write_memory(cpu.program_counter + 1, 0x00);
    cpu.register_x = 0x77; // 0x77 + 0x89 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.write_memory(cpu.program_counter, 0xf8);
    cpu.write_memory(cpu.program_counter + 1, 0x15);
    cpu.register_x = 0x25; // 0x25 + 0xf8 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.program_counter = 0x200;
    cpu.write_memory(cpu.program_counter, 0xff);
    cpu.write_memory(cpu.program_counter + 1, 0x11);
    cpu.register_x = 0x00; // 0x00 + 0xff <= 0xff
    assert!(!cpu.detect_page_cross(addressing_mode));

    cpu.write_memory(cpu.program_counter, 0x24);
    cpu.write_memory(cpu.program_counter + 1, 0x15);
    cpu.register_x = 0x25; // 0x25 + 0x24 <= 0xff
    assert!(!cpu.detect_page_cross(addressing_mode));
}

#[test]
fn test_page_cross_detection_absolute_y() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    let addressing_mode = AddressingMode::Absolute_Y;

    cpu.write_memory(cpu.program_counter, 0xff);
    cpu.write_memory(cpu.program_counter + 1, 0x00);
    cpu.register_y = 0x34; // 0xff + 0x34 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.program_counter = 0x100;
    cpu.write_memory(cpu.program_counter, 0x89);
    cpu.write_memory(cpu.program_counter + 1, 0x00);
    cpu.register_y = 0x77; // 0x77 + 0x89 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.write_memory(cpu.program_counter, 0xf8);
    cpu.write_memory(cpu.program_counter + 1, 0x15);
    cpu.register_y = 0x25; // 0x25 + 0xf8 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.program_counter = 0x200;
    cpu.write_memory(cpu.program_counter, 0xff);
    cpu.write_memory(cpu.program_counter + 1, 0x11);
    cpu.register_y = 0x00; // 0x00 + 0xff <= 0xff
    assert!(!cpu.detect_page_cross(addressing_mode));

    cpu.write_memory(cpu.program_counter, 0x24);
    cpu.write_memory(cpu.program_counter + 1, 0x15);
    cpu.register_y = 0x25; // 0x25 + 0x24 <= 0xff
    assert!(!cpu.detect_page_cross(addressing_mode));
}

#[test]
fn test_page_cross_detection_indirect_y() {
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus);
    let addressing_mode = AddressingMode::Indirect_Y;

    cpu.program_counter = 0x100;
    cpu.write_memory(cpu.program_counter, 0x9a);
    cpu.write_memory(0x9a, 0x89);
    cpu.write_memory(0x9b, 0x00);
    cpu.register_y = 0x77; // 0x77 + 0x89 > 0xff
    assert!(cpu.detect_page_cross(addressing_mode));

    cpu.program_counter = 0x200;
    cpu.write_memory(cpu.program_counter, 0x55);
    cpu.write_memory(0x55, 0x88);
    cpu.write_memory(0x56, 0x12);
    cpu.register_y = 0x77; // 0x77 + 0x88 <= 0xff
    assert!(!cpu.detect_page_cross(addressing_mode));
}
