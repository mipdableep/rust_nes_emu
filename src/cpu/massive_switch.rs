use super::mem_utils::AddressingMode;
use super::CPU;

#[allow(unused_variables)]
impl CPU {
    pub fn massive_switch(&mut self, op_code: u8, program: &Vec<u8>) -> bool {
        match op_code {
            ///////////////////////
            //// register_ops /////
            ///////////////////////

            // CPX : Compare X Register
            0xE0 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.CPX();
            }
            0xE4 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.CPX();
            }
            0xEC => {
                let addressing_mode = AddressingMode::Absolute;
                // self.CPX();
            }

            // CPY : Compare Y Register
            0xC0 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.CPY();
            }
            0xC4 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.CPY();
            }
            0xCC => {
                let addressing_mode = AddressingMode::Absolute;
                // self.CPY();
            }

            // DEX : Decrement X Register
            0xCA => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.DEX();
            }

            // DEY : Decrement Y Register
            0x88 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.DEY();
            }

            // INC : Increment Memory
            0xE6 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.INC();
            }
            0xF6 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.INC();
            }
            0xEE => {
                let addressing_mode = AddressingMode::Absolute;
                // self.INC();
            }
            0xFE => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.INC();
            }

            // INX : Increment X Register
            0xE8 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.INX();
            }

            // INY : Increment Y Register
            0xC8 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.INY();
            }

            // LDA : Load Accumulator
            0xA9 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.LDA();
            }
            0xA5 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.LDA();
            }
            0xB5 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.LDA();
            }
            0xAD => {
                let addressing_mode = AddressingMode::Absolute;
                // self.LDA();
            }
            0xBD => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.LDA();
            }
            0xB9 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.LDA();
            }
            0xA1 => {
                let addressing_mode = AddressingMode::Indirect_X;
                // self.LDA();
            }
            0xB1 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                // self.LDA();
            }

            // LDX : Load X Register
            0xA2 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.LDX();
            }
            0xA6 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.LDX();
            }
            0xB6 => {
                let addressing_mode = AddressingMode::ZeroPage_Y;
                // self.LDX();
            }
            0xAE => {
                let addressing_mode = AddressingMode::Absolute;
                // self.LDX();
            }
            0xBE => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.LDX();
            }

            // LDY : Load Y Register
            0xA0 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.LDY();
            }
            0xA4 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.LDY();
            }
            0xB4 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.LDY();
            }
            0xAC => {
                let addressing_mode = AddressingMode::Absolute;
                // self.LDY();
            }
            0xBC => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.LDY();
            }

            // STA : Store Accumulator
            0x85 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.STA();
            }
            0x95 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                self.STA();
            }
            0x8D => {
                let addressing_mode = AddressingMode::Absolute;
                self.STA();
            }
            0x9D => {
                let addressing_mode = AddressingMode::Absolute_X;
                self.STA();
            }
            0x99 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                self.STA();
            }
            0x81 => {
                let addressing_mode = AddressingMode::Indirect_X;
                self.STA();
            }
            0x91 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                self.STA();
            }

            // STX : Store X Register
            0x86 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.STX();
            }
            0x96 => {
                let addressing_mode = AddressingMode::ZeroPage_Y;
                self.STX();
            }
            0x8E => {
                let addressing_mode = AddressingMode::Absolute;
                self.STX();
            }

            // STY : Store Y Register
            0x84 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.STY();
            }
            0x94 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                self.STY();
            }
            0x8C => {
                let addressing_mode = AddressingMode::Absolute;
                self.STY();
            }

            // TAX : Transfer Accumulator to X
            0xAA => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TAX();
            }

            // TAY : Transfer Accumulator to Y
            0xA8 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TAY();
            }

            // TSX : Transfer Stack Pointer to X
            0xBA => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TSX();
            }

            // TXA : Transfer X to Accumulator
            0x8A => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TXA();
            }

            // TXS : Transfer X to Stack Pointer
            0x9A => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TXS();
            }

            // TYA : Transfer Y to Accumulator
            0x98 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.TYA();
            }

            /////////////////////
            //// interrupts /////
            /////////////////////

            // RTI : Return from Interrupt
            0x40 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.RTI();
            }

            // BRK : Force Interrupt
            0x00 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.BRK();
                return false; // we return if we want to continue. By returning false, brk will stop the program
            }

            ////////////////
            //// no_op /////
            ////////////////

            // NOP : No Operation
            0xEA => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.NOP();
            }

            ///////////////////////////
            //// arithmatic_logic /////
            ///////////////////////////

            // ADC : Add with Carry
            0x69 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.ADC();
            }
            0x65 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.ADC();
            }
            0x75 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.ADC();
            }
            0x6D => {
                let addressing_mode = AddressingMode::Absolute;
                // self.ADC();
            }
            0x7D => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.ADC();
            }
            0x79 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.ADC();
            }
            0x61 => {
                let addressing_mode = AddressingMode::Indirect_X;
                // self.ADC();
            }
            0x71 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                // self.ADC();
            }

            // AND : Logical AND
            0x29 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.AND();
            }
            0x25 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.AND();
            }
            0x35 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.AND();
            }
            0x2D => {
                let addressing_mode = AddressingMode::Absolute;
                // self.AND();
            }
            0x3D => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.AND();
            }
            0x39 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.AND();
            }
            0x21 => {
                let addressing_mode = AddressingMode::Indirect_X;
                // self.AND();
            }
            0x31 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                // self.AND();
            }

            // ASL : Arithmetic Shift Left
            0x0A => {
                let addressing_mode = AddressingMode::Accumulator;
                self.ASL_accumulator();
            }
            0x06 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.ASL_memory();
            }
            0x16 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.ASL_memory();
            }
            0x0E => {
                let addressing_mode = AddressingMode::Absolute;
                // self.ASL_memory();
            }
            0x1E => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.ASL_memory();
            }

            // BIT : Bit Test
            0x24 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.BIT();
            }
            0x2C => {
                let addressing_mode = AddressingMode::Absolute;
                // self.BIT();
            }

            // CMP : Compare
            0xC9 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.CMP();
            }
            0xC5 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.CMP();
            }
            0xD5 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.CMP();
            }
            0xCD => {
                let addressing_mode = AddressingMode::Absolute;
                // self.CMP();
            }
            0xDD => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.CMP();
            }
            0xD9 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.CMP();
            }
            0xC1 => {
                let addressing_mode = AddressingMode::Indirect_X;
                // self.CMP();
            }
            0xD1 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                // self.CMP();
            }

            // DEC : Decrement Memory
            0xC6 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.DEC();
            }
            0xD6 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.DEC();
            }
            0xCE => {
                let addressing_mode = AddressingMode::Absolute;
                // self.DEC();
            }
            0xDE => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.DEC();
            }

            // EOR : Exclusive OR
            0x49 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.EOR();
            }
            0x45 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.EOR();
            }
            0x55 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.EOR();
            }
            0x4D => {
                let addressing_mode = AddressingMode::Absolute;
                // self.EOR();
            }
            0x5D => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.EOR();
            }
            0x59 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.EOR();
            }
            0x41 => {
                let addressing_mode = AddressingMode::Indirect_X;
                // self.EOR();
            }
            0x51 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                // self.EOR();
            }

            // LSR : Logical Shift Right
            0x4A => {
                let addressing_mode = AddressingMode::Accumulator;
                self.LSR_accumulator();
            }
            0x46 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.LSR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }
            0x56 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                self.LSR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }
            0x4E => {
                let addressing_mode = AddressingMode::Absolute;
                self.LSR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }
            0x5E => {
                let addressing_mode = AddressingMode::Absolute_X;
                self.LSR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }

            // ORA : Logical Inclusive OR
            0x09 => {
                let addressing_mode = AddressingMode::Immediate;
                // self.ORA();
            }
            0x05 => {
                let addressing_mode = AddressingMode::ZeroPage;
                // self.ORA();
            }
            0x15 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                // self.ORA();
            }
            0x0D => {
                let addressing_mode = AddressingMode::Absolute;
                // self.ORA();
            }
            0x1D => {
                let addressing_mode = AddressingMode::Absolute_X;
                // self.ORA();
            }
            0x19 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                // self.ORA();
            }
            0x01 => {
                let addressing_mode = AddressingMode::Indirect_X;
                // self.ORA();
            }
            0x11 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                // self.ORA();
            }

            // ROL : Rotate Left
            0x2A => {
                let addressing_mode = AddressingMode::Accumulator;
                self.ROL();
            }
            0x26 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.ROL();
            }
            0x36 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                self.ROL();
            }
            0x2E => {
                let addressing_mode = AddressingMode::Absolute;
                self.ROL();
            }
            0x3E => {
                let addressing_mode = AddressingMode::Absolute_X;
                self.ROL();
            }

            // ROR : Rotate Right
            0x6A => {
                let addressing_mode = AddressingMode::Accumulator;
                self.ROR_accumulator();
            }
            0x66 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.ROR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }
            0x76 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                self.ROR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }
            0x6E => {
                let addressing_mode = AddressingMode::Absolute;
                self.ROR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }
            0x7E => {
                let addressing_mode = AddressingMode::Absolute_X;
                self.ROR_memory(self.convert_mode_to_operand_mem_address(addressing_mode));
            }

            // SBC : Subtract with Carry
            0xE9 => {
                let addressing_mode = AddressingMode::Immediate;
                self.SBC();
            }
            0xE5 => {
                let addressing_mode = AddressingMode::ZeroPage;
                self.SBC();
            }
            0xF5 => {
                let addressing_mode = AddressingMode::ZeroPage_X;
                self.SBC();
            }
            0xED => {
                let addressing_mode = AddressingMode::Absolute;
                self.SBC();
            }
            0xFD => {
                let addressing_mode = AddressingMode::Absolute_X;
                self.SBC();
            }
            0xF9 => {
                let addressing_mode = AddressingMode::Absolute_Y;
                self.SBC();
            }
            0xE1 => {
                let addressing_mode = AddressingMode::Indirect_X;
                self.SBC();
            }
            0xF1 => {
                let addressing_mode = AddressingMode::Indirect_Y;
                self.SBC();
            }

            ///////////////////////
            //// control_flow /////
            ///////////////////////

            // BCC : Branch if Carry Clear
            0x90 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BCC();
            }

            // BCS : Branch if Carry Set
            0xB0 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BCS();
            }

            // BEQ : Branch if Equal
            0xF0 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BEQ();
            }

            // BMI : Branch if Minus
            0x30 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BMI();
            }

            // BNE : Branch if Not Equal
            0xD0 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BNE();
            }

            // BPL : Branch if Positive
            0x10 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BPL();
            }

            // BVC : Branch if Overflow Clear
            0x50 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BVC();
            }

            // BVS : Branch if Overflow Set
            0x70 => {
                let addressing_mode = AddressingMode::Relative;
                // self.BVS();
            }

            // JMP : Jump
            0x4C => {
                let addressing_mode = AddressingMode::Absolute;
                self.JMP();
            }
            0x6C => {
                let addressing_mode = AddressingMode::Indirect;
                self.JMP();
            }

            // JSR : Jump to Subroutine
            0x20 => {
                let addressing_mode = AddressingMode::Absolute;
                self.JSR();
            }

            // RTS : Return from Subroutine
            0x60 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.RTS();
            }

            ////////////////////////
            //// stack_related /////
            ////////////////////////

            // PHA : Push Accumulator
            0x48 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PHA();
            }

            // PHP : Push Processor Status
            0x08 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PHP();
            }

            // PLA : Pull Accumulator
            0x68 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PLA();
            }

            // PLP : Pull Processor Status
            0x28 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.PLP();
            }

            //////////////////////////
            //// status_register /////
            //////////////////////////

            // CLC : Clear Carry Flag
            0x18 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLC();
            }

            // CLD : Clear Decimal Mode
            0xD8 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLD();
            }

            // CLI : Clear Interrupt Disable
            0x58 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLI();
            }

            // CLV : Clear Overflow Flag
            0xB8 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.CLV();
            }

            // SEC : Set Carry Flag
            0x38 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.SEC();
            }

            // SED : Set Decimal Flag
            0xF8 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.SED();
            }

            // SEI : Set Interrupt Disable
            0x78 => {
                let addressing_mode = AddressingMode::NoneAddressing;
                self.SEI();
            }

            _ => {
                todo!()
            }
        }
        true
    }
}
