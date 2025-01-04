use crate::cpu::mem_utils::AddressingMode;

pub struct OpcodeMetadata {
    pub opcode: u8,                      // the opcodes
    pub bytes: u16,                      // the number of bytes the opcode read
    pub base_cycles: u16,                // number of cycle the opcode takes
    pub addressing_mode: AddressingMode, // the addressing mod
    pub increase_on_page_cross: bool, // should the opcode take +1 cycle if the page boundary is crossed
}
const OPCODE_00: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x00,
    bytes: 1,
    base_cycles: 7,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_01: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x01,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_05: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x05,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_06: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x06,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_08: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x08,
    bytes: 1,
    base_cycles: 3,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_09: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x09,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_0A: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x0A,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::Accumulator,
    increase_on_page_cross: false,
};
const OPCODE_0D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x0D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_0E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x0E,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_10: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x10,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_11: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x11,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_15: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x15,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_16: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x16,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_18: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x18,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_19: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x19,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_1D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x1D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_1E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x1E,
    bytes: 3,
    base_cycles: 7,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
const OPCODE_20: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x20,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_21: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x21,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_24: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x24,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_25: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x25,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_26: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x26,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_28: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x28,
    bytes: 1,
    base_cycles: 4,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_29: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x29,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_2A: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x2A,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::Accumulator,
    increase_on_page_cross: false,
};
const OPCODE_2C: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x2C,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_2D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x2D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_2E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x2E,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_30: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x30,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_31: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x31,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_35: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x35,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_36: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x36,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_38: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x38,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_39: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x39,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_3D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x3D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_3E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x3E,
    bytes: 3,
    base_cycles: 7,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
const OPCODE_40: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x40,
    bytes: 1,
    base_cycles: 6,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_41: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x41,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_45: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x45,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_46: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x46,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_48: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x48,
    bytes: 1,
    base_cycles: 3,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_49: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x49,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_4A: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x4A,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::Accumulator,
    increase_on_page_cross: false,
};
const OPCODE_4C: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x4C,
    bytes: 3,
    base_cycles: 3,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_4D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x4D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_4E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x4E,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_50: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x50,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_51: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x51,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_55: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x55,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_56: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x56,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_58: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x58,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_59: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x59,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_5D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x5D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_5E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x5E,
    bytes: 3,
    base_cycles: 7,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
const OPCODE_60: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x60,
    bytes: 1,
    base_cycles: 6,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_61: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x61,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_65: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x65,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_66: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x66,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_68: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x68,
    bytes: 1,
    base_cycles: 4,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_69: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x69,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_6A: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x6A,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::Accumulator,
    increase_on_page_cross: false,
};
const OPCODE_6C: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x6C,
    bytes: 3,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect,
    increase_on_page_cross: false,
};
const OPCODE_6D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x6D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_6E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x6E,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_70: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x70,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_71: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x71,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_75: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x75,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_76: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x76,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_78: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x78,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_79: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x79,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_7D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x7D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_7E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x7E,
    bytes: 3,
    base_cycles: 7,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
const OPCODE_81: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x81,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_84: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x84,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_85: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x85,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_86: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x86,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_88: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x88,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_8A: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x8A,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_8C: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x8C,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_8D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x8D,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_8E: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x8E,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_90: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x90,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_91: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x91,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: false,
};
const OPCODE_94: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x94,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_95: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x95,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_96: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x96,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_Y,
    increase_on_page_cross: false,
};
const OPCODE_98: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x98,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_99: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x99,
    bytes: 3,
    base_cycles: 5,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: false,
};
const OPCODE_9A: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x9A,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_9D: OpcodeMetadata = OpcodeMetadata {
    opcode: 0x9D,
    bytes: 3,
    base_cycles: 5,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
const OPCODE_A0: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA0,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_A1: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA1,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_A2: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA2,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_A4: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA4,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_A5: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA5,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_A6: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA6,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_A8: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA8,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_A9: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xA9,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_AA: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xAA,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_AC: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xAC,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_AD: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xAD,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_AE: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xAE,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_B0: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB0,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_B1: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB1,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_B4: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB4,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_B5: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB5,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_B6: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB6,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_Y,
    increase_on_page_cross: false,
};
const OPCODE_B8: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB8,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_B9: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xB9,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_BA: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xBA,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_BC: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xBC,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_BD: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xBD,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_BE: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xBE,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_C0: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC0,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_C1: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC1,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_C4: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC4,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_C5: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC5,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_C6: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC6,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_C8: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC8,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_C9: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xC9,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_CA: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xCA,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_CC: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xCC,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_CD: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xCD,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_CE: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xCE,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_D0: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xD0,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_D1: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xD1,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_D5: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xD5,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_D6: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xD6,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_D8: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xD8,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_D9: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xD9,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_DD: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xDD,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_DE: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xDE,
    bytes: 3,
    base_cycles: 7,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
const OPCODE_E0: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE0,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_E1: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE1,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::Indirect_X,
    increase_on_page_cross: false,
};
const OPCODE_E4: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE4,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_E5: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE5,
    bytes: 2,
    base_cycles: 3,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_E6: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE6,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::ZeroPage,
    increase_on_page_cross: false,
};
const OPCODE_E8: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE8,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_E9: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xE9,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Immediate,
    increase_on_page_cross: false,
};
const OPCODE_EA: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xEA,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_EC: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xEC,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_ED: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xED,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_EE: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xEE,
    bytes: 3,
    base_cycles: 6,
    addressing_mode: AddressingMode::Absolute,
    increase_on_page_cross: false,
};
const OPCODE_F0: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xF0,
    bytes: 2,
    base_cycles: 2,
    addressing_mode: AddressingMode::Relative,
    increase_on_page_cross: true,
};
const OPCODE_F1: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xF1,
    bytes: 2,
    base_cycles: 5,
    addressing_mode: AddressingMode::Indirect_Y,
    increase_on_page_cross: true,
};
const OPCODE_F5: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xF5,
    bytes: 2,
    base_cycles: 4,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_F6: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xF6,
    bytes: 2,
    base_cycles: 6,
    addressing_mode: AddressingMode::ZeroPage_X,
    increase_on_page_cross: false,
};
const OPCODE_F8: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xF8,
    bytes: 1,
    base_cycles: 2,
    addressing_mode: AddressingMode::NoneAddressing,
    increase_on_page_cross: false,
};
const OPCODE_F9: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xF9,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_Y,
    increase_on_page_cross: true,
};
const OPCODE_FD: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xFD,
    bytes: 3,
    base_cycles: 4,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: true,
};
const OPCODE_FE: OpcodeMetadata = OpcodeMetadata {
    opcode: 0xFE,
    bytes: 3,
    base_cycles: 7,
    addressing_mode: AddressingMode::Absolute_X,
    increase_on_page_cross: false,
};
pub fn get_opcode_metadat_from_opcode(opcode: u8) -> OpcodeMetadata {
    match opcode {
        0x00 => OPCODE_00,
        0x01 => OPCODE_01,
        0x05 => OPCODE_05,
        0x06 => OPCODE_06,
        0x08 => OPCODE_08,
        0x09 => OPCODE_09,
        0x0A => OPCODE_0A,
        0x0D => OPCODE_0D,
        0x0E => OPCODE_0E,
        0x10 => OPCODE_10,
        0x11 => OPCODE_11,
        0x15 => OPCODE_15,
        0x16 => OPCODE_16,
        0x18 => OPCODE_18,
        0x19 => OPCODE_19,
        0x1D => OPCODE_1D,
        0x1E => OPCODE_1E,
        0x20 => OPCODE_20,
        0x21 => OPCODE_21,
        0x24 => OPCODE_24,
        0x25 => OPCODE_25,
        0x26 => OPCODE_26,
        0x28 => OPCODE_28,
        0x29 => OPCODE_29,
        0x2A => OPCODE_2A,
        0x2C => OPCODE_2C,
        0x2D => OPCODE_2D,
        0x2E => OPCODE_2E,
        0x30 => OPCODE_30,
        0x31 => OPCODE_31,
        0x35 => OPCODE_35,
        0x36 => OPCODE_36,
        0x38 => OPCODE_38,
        0x39 => OPCODE_39,
        0x3D => OPCODE_3D,
        0x3E => OPCODE_3E,
        0x40 => OPCODE_40,
        0x41 => OPCODE_41,
        0x45 => OPCODE_45,
        0x46 => OPCODE_46,
        0x48 => OPCODE_48,
        0x49 => OPCODE_49,
        0x4A => OPCODE_4A,
        0x4C => OPCODE_4C,
        0x4D => OPCODE_4D,
        0x4E => OPCODE_4E,
        0x50 => OPCODE_50,
        0x51 => OPCODE_51,
        0x55 => OPCODE_55,
        0x56 => OPCODE_56,
        0x58 => OPCODE_58,
        0x59 => OPCODE_59,
        0x5D => OPCODE_5D,
        0x5E => OPCODE_5E,
        0x60 => OPCODE_60,
        0x61 => OPCODE_61,
        0x65 => OPCODE_65,
        0x66 => OPCODE_66,
        0x68 => OPCODE_68,
        0x69 => OPCODE_69,
        0x6A => OPCODE_6A,
        0x6C => OPCODE_6C,
        0x6D => OPCODE_6D,
        0x6E => OPCODE_6E,
        0x70 => OPCODE_70,
        0x71 => OPCODE_71,
        0x75 => OPCODE_75,
        0x76 => OPCODE_76,
        0x78 => OPCODE_78,
        0x79 => OPCODE_79,
        0x7D => OPCODE_7D,
        0x7E => OPCODE_7E,
        0x81 => OPCODE_81,
        0x84 => OPCODE_84,
        0x85 => OPCODE_85,
        0x86 => OPCODE_86,
        0x88 => OPCODE_88,
        0x8A => OPCODE_8A,
        0x8C => OPCODE_8C,
        0x8D => OPCODE_8D,
        0x8E => OPCODE_8E,
        0x90 => OPCODE_90,
        0x91 => OPCODE_91,
        0x94 => OPCODE_94,
        0x95 => OPCODE_95,
        0x96 => OPCODE_96,
        0x98 => OPCODE_98,
        0x99 => OPCODE_99,
        0x9A => OPCODE_9A,
        0x9D => OPCODE_9D,
        0xA0 => OPCODE_A0,
        0xA1 => OPCODE_A1,
        0xA2 => OPCODE_A2,
        0xA4 => OPCODE_A4,
        0xA5 => OPCODE_A5,
        0xA6 => OPCODE_A6,
        0xA8 => OPCODE_A8,
        0xA9 => OPCODE_A9,
        0xAA => OPCODE_AA,
        0xAC => OPCODE_AC,
        0xAD => OPCODE_AD,
        0xAE => OPCODE_AE,
        0xB0 => OPCODE_B0,
        0xB1 => OPCODE_B1,
        0xB4 => OPCODE_B4,
        0xB5 => OPCODE_B5,
        0xB6 => OPCODE_B6,
        0xB8 => OPCODE_B8,
        0xB9 => OPCODE_B9,
        0xBA => OPCODE_BA,
        0xBC => OPCODE_BC,
        0xBD => OPCODE_BD,
        0xBE => OPCODE_BE,
        0xC0 => OPCODE_C0,
        0xC1 => OPCODE_C1,
        0xC4 => OPCODE_C4,
        0xC5 => OPCODE_C5,
        0xC6 => OPCODE_C6,
        0xC8 => OPCODE_C8,
        0xC9 => OPCODE_C9,
        0xCA => OPCODE_CA,
        0xCC => OPCODE_CC,
        0xCD => OPCODE_CD,
        0xCE => OPCODE_CE,
        0xD0 => OPCODE_D0,
        0xD1 => OPCODE_D1,
        0xD5 => OPCODE_D5,
        0xD6 => OPCODE_D6,
        0xD8 => OPCODE_D8,
        0xD9 => OPCODE_D9,
        0xDD => OPCODE_DD,
        0xDE => OPCODE_DE,
        0xE0 => OPCODE_E0,
        0xE1 => OPCODE_E1,
        0xE4 => OPCODE_E4,
        0xE5 => OPCODE_E5,
        0xE6 => OPCODE_E6,
        0xE8 => OPCODE_E8,
        0xE9 => OPCODE_E9,
        0xEA => OPCODE_EA,
        0xEC => OPCODE_EC,
        0xED => OPCODE_ED,
        0xEE => OPCODE_EE,
        0xF0 => OPCODE_F0,
        0xF1 => OPCODE_F1,
        0xF5 => OPCODE_F5,
        0xF6 => OPCODE_F6,
        0xF8 => OPCODE_F8,
        0xF9 => OPCODE_F9,
        0xFD => OPCODE_FD,
        0xFE => OPCODE_FE,
        _ => panic!("Received opcode {opcode} which is not supported currently"),
    }
}
