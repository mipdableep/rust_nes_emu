use super::mem_utils::AddressingMode;
use super::CPU;

impl CPU {
    fn massive_switch(&mut self, op_code: u8) {
        match op_code {
            ///////////////////////
            //// register_ops /////
            ///////////////////////

            // CPX : Compare X Register
            0xE0 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.CPX();
            }
            0xE4 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.CPX();
            }
            0xEC => {
                let _addressing_mode = AddressingMode::Absolute;
                self.CPX();
            }

            // CPY : Compare Y Register
            0xC0 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.CPY();
            }
            0xC4 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.CPY();
            }
            0xCC => {
                let _addressing_mode = AddressingMode::Absolute;
                self.CPY();
            }

            // DEX : Decrement X Register
            0xCA => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.DEX();
            }

            // DEY : Decrement Y Register
            0x88 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.DEY();
            }

            // INC : Increment Memory
            0xE6 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.INC();
            }
            0xF6 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.INC();
            }
            0xEE => {
                let _addressing_mode = AddressingMode::Absolute;
                self.INC();
            }
            0xFE => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.INC();
            }

            // INX : Increment X Register
            0xE8 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.INX();
            }

            // INY : Increment Y Register
            0xC8 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.INY();
            }

            // LDA : Load Accumulator
            0xA9 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.LDA();
            }
            0xA5 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.LDA();
            }
            0xB5 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.LDA();
            }
            0xAD => {
                let _addressing_mode = AddressingMode::Absolute;
                self.LDA();
            }
            0xBD => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.LDA();
            }
            0xB9 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.LDA();
            }
            0xA1 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.LDA();
            }
            0xB1 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.LDA();
            }

            // LDX : Load X Register
            0xA2 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.LDX();
            }
            0xA6 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.LDX();
            }
            0xB6 => {
                let _addressing_mode = AddressingMode::ZeroPage_Y;
                self.LDX();
            }
            0xAE => {
                let _addressing_mode = AddressingMode::Absolute;
                self.LDX();
            }
            0xBE => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.LDX();
            }

            // LDY : Load Y Register
            0xA0 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.LDY();
            }
            0xA4 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.LDY();
            }
            0xB4 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.LDY();
            }
            0xAC => {
                let _addressing_mode = AddressingMode::Absolute;
                self.LDY();
            }
            0xBC => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.LDY();
            }

            // STA : Store Accumulator
            0x85 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.STA();
            }
            0x95 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.STA();
            }
            0x8D => {
                let _addressing_mode = AddressingMode::Absolute;
                self.STA();
            }
            0x9D => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.STA();
            }
            0x99 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.STA();
            }
            0x81 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.STA();
            }
            0x91 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.STA();
            }

            // STX : Store X Register
            0x86 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.STX();
            }
            0x96 => {
                let _addressing_mode = AddressingMode::ZeroPage_Y;
                self.STX();
            }
            0x8E => {
                let _addressing_mode = AddressingMode::Absolute;
                self.STX();
            }

            // STY : Store Y Register
            0x84 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.STY();
            }
            0x94 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.STY();
            }
            0x8C => {
                let _addressing_mode = AddressingMode::Absolute;
                self.STY();
            }

            // TAX : Transfer Accumulator to X
            0xAA => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.TAX();
            }

            // TAY : Transfer Accumulator to Y
            0xA8 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.TAY();
            }

            // TSX : Transfer Stack Pointer to X
            0xBA => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.TSX();
            }

            // TXA : Transfer X to Accumulator
            0x8A => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.TXA();
            }

            // TXS : Transfer X to Stack Pointer
            0x9A => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.TXS();
            }

            // TYA : Transfer Y to Accumulator
            0x98 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.TYA();
            }

            /////////////////////
            //// interrupts /////
            /////////////////////

            // RTI : Return from Interrupt
            0x40 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.RTI();
            }

            // BRK : Force Interrupt
            0x00 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.BRK();
            }

            ////////////////
            //// no_op /////
            ////////////////

            // NOP : No Operation
            0xEA => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.NOP();
            }

            ///////////////////////////
            //// arithmatic_logic /////
            ///////////////////////////

            // ADC : Add with Carry
            0x69 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.ADC();
            }
            0x65 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.ADC();
            }
            0x75 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.ADC();
            }
            0x6D => {
                let _addressing_mode = AddressingMode::Absolute;
                self.ADC();
            }
            0x7D => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.ADC();
            }
            0x79 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.ADC();
            }
            0x61 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.ADC();
            }
            0x71 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.ADC();
            }

            // AND : Logical AND
            0x29 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.AND();
            }
            0x25 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.AND();
            }
            0x35 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.AND();
            }
            0x2D => {
                let _addressing_mode = AddressingMode::Absolute;
                self.AND();
            }
            0x3D => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.AND();
            }
            0x39 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.AND();
            }
            0x21 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.AND();
            }
            0x31 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.AND();
            }

            // ASL : Arithmetic Shift Left
            0x0A => {
                let _addressing_mode = AddressingMode::Accumulator;
                self.ASL();
            }
            0x06 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.ASL();
            }
            0x16 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.ASL();
            }
            0x0E => {
                let _addressing_mode = AddressingMode::Absolute;
                self.ASL();
            }
            0x1E => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.ASL();
            }

            // BIT : Bit Test
            0x24 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.BIT();
            }
            0x2C => {
                let _addressing_mode = AddressingMode::Absolute;
                self.BIT();
            }

            // CMP : Compare
            0xC9 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.CMP();
            }
            0xC5 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.CMP();
            }
            0xD5 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.CMP();
            }
            0xCD => {
                let _addressing_mode = AddressingMode::Absolute;
                self.CMP();
            }
            0xDD => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.CMP();
            }
            0xD9 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.CMP();
            }
            0xC1 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.CMP();
            }
            0xD1 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.CMP();
            }

            // DEC : Decrement Memory
            0xC6 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.DEC();
            }
            0xD6 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.DEC();
            }
            0xCE => {
                let _addressing_mode = AddressingMode::Absolute;
                self.DEC();
            }
            0xDE => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.DEC();
            }

            // EOR : Exclusive OR
            0x49 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.EOR();
            }
            0x45 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.EOR();
            }
            0x55 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.EOR();
            }
            0x4D => {
                let _addressing_mode = AddressingMode::Absolute;
                self.EOR();
            }
            0x5D => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.EOR();
            }
            0x59 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.EOR();
            }
            0x41 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.EOR();
            }
            0x51 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.EOR();
            }

            // LSR : Logical Shift Right
            0x4A => {
                let _addressing_mode = AddressingMode::Accumulator;
                self.LSR();
            }
            0x46 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.LSR();
            }
            0x56 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.LSR();
            }
            0x4E => {
                let _addressing_mode = AddressingMode::Absolute;
                self.LSR();
            }
            0x5E => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.LSR();
            }

            // ORA : Logical Inclusive OR
            0x09 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.ORA();
            }
            0x05 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.ORA();
            }
            0x15 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.ORA();
            }
            0x0D => {
                let _addressing_mode = AddressingMode::Absolute;
                self.ORA();
            }
            0x1D => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.ORA();
            }
            0x19 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.ORA();
            }
            0x01 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.ORA();
            }
            0x11 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.ORA();
            }

            // ROL : Rotate Left
            0x2A => {
                let _addressing_mode = AddressingMode::Accumulator;
                self.ROL();
            }
            0x26 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.ROL();
            }
            0x36 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.ROL();
            }
            0x2E => {
                let _addressing_mode = AddressingMode::Absolute;
                self.ROL();
            }
            0x3E => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.ROL();
            }

            // ROR : Rotate Right
            0x6A => {
                let _addressing_mode = AddressingMode::Accumulator;
                self.ROR();
            }
            0x66 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.ROR();
            }
            0x76 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.ROR();
            }
            0x6E => {
                let _addressing_mode = AddressingMode::Absolute;
                self.ROR();
            }
            0x7E => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.ROR();
            }

            // SBC : Subtract with Carry
            0xE9 => {
                let _addressing_mode = AddressingMode::Immediate;
                self.SBC();
            }
            0xE5 => {
                let _addressing_mode = AddressingMode::ZeroPage;
                self.SBC();
            }
            0xF5 => {
                let _addressing_mode = AddressingMode::ZeroPage_X;
                self.SBC();
            }
            0xED => {
                let _addressing_mode = AddressingMode::Absolute;
                self.SBC();
            }
            0xFD => {
                let _addressing_mode = AddressingMode::Absolute_X;
                self.SBC();
            }
            0xF9 => {
                let _addressing_mode = AddressingMode::Absolute_Y;
                self.SBC();
            }
            0xE1 => {
                let _addressing_mode = AddressingMode::Indirect_X;
                self.SBC();
            }
            0xF1 => {
                let _addressing_mode = AddressingMode::Indirect_Y;
                self.SBC();
            }

            ///////////////////////
            //// control_flow /////
            ///////////////////////

            // BCC : Branch if Carry Clear
            0x90 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BCC();
            }

            // BCS : Branch if Carry Set
            0xB0 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BCS();
            }

            // BEQ : Branch if Equal
            0xF0 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BEQ();
            }

            // BMI : Branch if Minus
            0x30 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BMI();
            }

            // BNE : Branch if Not Equal
            0xD0 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BNE();
            }

            // BPL : Branch if Positive
            0x10 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BPL();
            }

            // BVC : Branch if Overflow Clear
            0x50 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BVC();
            }

            // BVS : Branch if Overflow Set
            0x70 => {
                let _addressing_mode = AddressingMode::Relative;
                self.BVS();
            }

            // JMP : Jump
            0x4C => {
                let _addressing_mode = AddressingMode::Absolute;
                self.JMP();
            }
            0x6C => {
                let _addressing_mode = AddressingMode::Indirect;
                self.JMP();
            }

            // JSR : Jump to Subroutine
            0x20 => {
                let _addressing_mode = AddressingMode::Absolute;
                self.JSR();
            }

            // RTS : Return from Subroutine
            0x60 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.RTS();
            }

            ////////////////////////
            //// stack_related /////
            ////////////////////////

            // PHA : Push Accumulator
            0x48 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.PHA();
            }

            // PHP : Push Processor Status
            0x08 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.PHP();
            }

            // PLA : Pull Accumulator
            0x68 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.PLA();
            }

            // PLP : Pull Processor Status
            0x28 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.PLP();
            }

            //////////////////////////
            //// status_register /////
            //////////////////////////

            // CLC : Clear Carry Flag
            0x18 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.CLC();
            }

            // CLD : Clear Decimal Mode
            0xD8 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.CLD();
            }

            // CLI : Clear Interrupt Disable
            0x58 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.CLI();
            }

            // CLV : Clear Overflow Flag
            0xB8 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.CLV();
            }

            // SEC : Set Carry Flag
            0x38 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.SEC();
            }

            // SED : Set Decimal Flag
            0xF8 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.SED();
            }

            // SEI : Set Interrupt Disable
            0x78 => {
                let _addressing_mode = AddressingMode::NoneAddressing;
                self.SEI();
            }

            _ => {
                todo!()
            }
        }
    }
}