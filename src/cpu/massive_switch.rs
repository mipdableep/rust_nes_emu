use super::mem_utils::AddressingMode;
use super::CPU;

#[allow(unused_variables)]
impl CPU {
    pub fn massive_switch(&mut self, op_code: u8) -> bool {
        match op_code {

            ///////////////////////
            //// register_ops /////
            ///////////////////////


            // CPX : Compare X Register
            0xE0 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CPX(value);
            }
            0xE4 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CPX(value);
            }
            0xEC => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.CPX(value);
            }

            // CPY : Compare Y Register
            0xC0 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CPY(value);
            }
            0xC4 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CPY(value);
            }
            0xCC => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.CPY(value);
            }

            // DEX : Decrement X Register
            0xCA => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.DEX();
            }

            // DEY : Decrement Y Register
            0x88 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.DEY();
            }

            // INC : Increment Memory
            0xE6 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.INC(address);
            }
            0xF6 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.INC(address);
            }
            0xEE => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.INC(address);
            }
            0xFE => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.INC(address);
            }

            // INX : Increment X Register
            0xE8 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.INX();
            }

            // INY : Increment Y Register
            0xC8 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.INY();
            }

            // LDA : Load Accumulator
            0xA9 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDA(value);
            }
            0xA5 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDA(value);
            }
            0xB5 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDA(value);
            }
            0xAD => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDA(value);
            }
            0xBD => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDA(value);
            }
            0xB9 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDA(value);
            }
            0xA1 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDA(value);
            }
            0xB1 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDA(value);
            }

            // LDX : Load X Register
            0xA2 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDX(value);
            }
            0xA6 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDX(value);
            }
            0xB6 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDX(value);
            }
            0xAE => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDX(value);
            }
            0xBE => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDX(value);
            }

            // LDY : Load Y Register
            0xA0 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDY(value);
            }
            0xA4 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDY(value);
            }
            0xB4 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.LDY(value);
            }
            0xAC => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDY(value);
            }
            0xBC => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.LDY(value);
            }

            // STA : Store Accumulator
            0x85 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STA(address);
            }
            0x95 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STA(address);
            }
            0x8D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.STA(address);
            }
            0x9D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.STA(address);
            }
            0x99 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.STA(address);
            }
            0x81 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STA(address);
            }
            0x91 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STA(address);
            }

            // STX : Store X Register
            0x86 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STX(address);
            }
            0x96 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_Y;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STX(address);
            }
            0x8E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.STX(address);
            }

            // STY : Store Y Register
            0x84 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STY(address);
            }
            0x94 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.STY(address);
            }
            0x8C => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.STY(address);
            }

            // TAX : Transfer Accumulator to X
            0xAA => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TAX();
            }

            // TAY : Transfer Accumulator to Y
            0xA8 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TAY();
            }

            // TSX : Transfer Stack Pointer to X
            0xBA => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TSX();
            }

            // TXA : Transfer X to Accumulator
            0x8A => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TXA();
            }

            // TXS : Transfer X to Stack Pointer
            0x9A => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TXS();
            }

            // TYA : Transfer Y to Accumulator
            0x98 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TYA();
            }

            /////////////////////
            //// interrupts /////
            /////////////////////


            // RTI : Return from Interrupt
            0x40 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.RTI();
            }

            // BRK : Force Interrupt
            0x00 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.BRK();
                return false
            }

            ////////////////
            //// no_op /////
            ////////////////


            // NOP : No Operation
            0xEA => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.NOP();
            }

            ///////////////////////////
            //// arithmatic_logic /////
            ///////////////////////////


            // ADC : Add with Carry
            0x69 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ADC(value);
            }
            0x65 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ADC(value);
            }
            0x75 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ADC(value);
            }
            0x6D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.ADC(value);
            }
            0x7D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.ADC(value);
            }
            0x79 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.ADC(value);
            }
            0x61 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ADC(value);
            }
            0x71 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ADC(value);
            }

            // AND : Logical AND
            0x29 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.AND(value);
            }
            0x25 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.AND(value);
            }
            0x35 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.AND(value);
            }
            0x2D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.AND(value);
            }
            0x3D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.AND(value);
            }
            0x39 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.AND(value);
            }
            0x21 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.AND(value);
            }
            0x31 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.AND(value);
            }

            // ASL : Arithmetic Shift Left
            0x0A => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Accumulator;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 0;
                self.ASL_accumulator();
            }
            0x06 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.ASL_memory(address);
            }
            0x16 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.ASL_memory(address);
            }
            0x0E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.ASL_memory(address);
            }
            0x1E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.ASL_memory(address);
            }

            // BIT : Bit Test
            0x24 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BIT(value);
            }
            0x2C => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.BIT(value);
            }

            // CMP : Compare
            0xC9 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CMP(value);
            }
            0xC5 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CMP(value);
            }
            0xD5 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CMP(value);
            }
            0xCD => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.CMP(value);
            }
            0xDD => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.CMP(value);
            }
            0xD9 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.CMP(value);
            }
            0xC1 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CMP(value);
            }
            0xD1 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.CMP(value);
            }

            // DEC : Decrement Memory
            0xC6 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.DEC(address);
            }
            0xD6 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.DEC(address);
            }
            0xCE => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.DEC(address);
            }
            0xDE => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.DEC(address);
            }

            // EOR : Exclusive OR
            0x49 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.EOR(value);
            }
            0x45 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.EOR(value);
            }
            0x55 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.EOR(value);
            }
            0x4D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.EOR(value);
            }
            0x5D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.EOR(value);
            }
            0x59 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.EOR(value);
            }
            0x41 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.EOR(value);
            }
            0x51 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.EOR(value);
            }

            // LSR : Logical Shift Right
            0x4A => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Accumulator;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 0;
                self.LSR_accumulator();
            }
            0x46 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.LSR_memory(address);
            }
            0x56 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.LSR_memory(address);
            }
            0x4E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.LSR_memory(address);
            }
            0x5E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.LSR_memory(address);
            }

            // ORA : Logical Inclusive OR
            0x09 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ORA(value);
            }
            0x05 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ORA(value);
            }
            0x15 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ORA(value);
            }
            0x0D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.ORA(value);
            }
            0x1D => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.ORA(value);
            }
            0x19 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.ORA(value);
            }
            0x01 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ORA(value);
            }
            0x11 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.ORA(value);
            }

            // ROL : Rotate Left
            0x2A => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Accumulator;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 0;
                self.ROL_accumulator();
            }
            0x26 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.ROL_memory(address);
            }
            0x36 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.ROL_memory(address);
            }
            0x2E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.ROL_memory(address);
            }
            0x3E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.ROL_memory(address);
            }

            // ROR : Rotate Right
            0x6A => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Accumulator;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 0;
                self.ROR_accumulator();
            }
            0x66 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.ROR_memory(address);
            }
            0x76 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 1;
                self.ROR_memory(address);
            }
            0x6E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.ROR_memory(address);
            }
            0x7E => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.ROR_memory(address);
            }

            // SBC : Subtract with Carry
            0xE9 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Immediate;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.SBC(value);
            }
            0xE5 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.SBC(value);
            }
            0xF5 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::ZeroPage_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.SBC(value);
            }
            0xED => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.SBC(value);
            }
            0xFD => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.SBC(value);
            }
            0xF9 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 2;
                self.SBC(value);
            }
            0xE1 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_X;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.SBC(value);
            }
            0xF1 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect_Y;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.SBC(value);
            }

            ///////////////////////
            //// control_flow /////
            ///////////////////////


            // BCC : Branch if Carry Clear
            0x90 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BCC(value);
            }

            // BCS : Branch if Carry Set
            0xB0 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BCS(value);
            }

            // BEQ : Branch if Equal
            0xF0 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BEQ(value);
            }

            // BMI : Branch if Minus
            0x30 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BMI(value);
            }

            // BNE : Branch if Not Equal
            0xD0 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BNE(value);
            }

            // BPL : Branch if Positive
            0x10 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BPL(value);
            }

            // BVC : Branch if Overflow Clear
            0x50 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BVC(value);
            }

            // BVS : Branch if Overflow Set
            0x70 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Relative;
                let value = self.convert_mode_to_val(addressing_mode);
                self.program_counter += 1;
                self.BVS(value);
            }

            // JMP : Jump
            0x4C => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.JMP(address);
            }
            0x6C => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Indirect;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.JMP(address);
            }

            // JSR : Jump to Subroutine
            0x20 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::Absolute;
                let address = self.convert_mode_to_operand_mem_address(addressing_mode);
                self.program_counter += 2;
                self.JSR(address);
            }

            // RTS : Return from Subroutine
            0x60 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.RTS();
            }

            ////////////////////////
            //// stack_related /////
            ////////////////////////


            // PHA : Push Accumulator
            0x48 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PHA();
            }

            // PHP : Push Processor Status
            0x08 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PHP();
            }

            // PLA : Pull Accumulator
            0x68 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PLA();
            }

            // PLP : Pull Processor Status
            0x28 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PLP();
            }

            //////////////////////////
            //// status_register /////
            //////////////////////////


            // CLC : Clear Carry Flag
            0x18 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLC();
            }

            // CLD : Clear Decimal Mode
            0xD8 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLD();
            }

            // CLI : Clear Interrupt Disable
            0x58 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLI();
            }

            // CLV : Clear Overflow Flag
            0xB8 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLV();
            }

            // SEC : Set Carry Flag
            0x38 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.SEC();
            }

            // SED : Set Decimal Flag
            0xF8 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.SED();
            }

            // SEI : Set Interrupt Disable
            0x78 => {
                self.program_counter += 1;
                let addressing_mode = AddressingMode::NoneAddressing;
                self.SEI();
            }
            _ => panic!("opcode {:} is not supported", op_code)
        }
        true
    }
}
