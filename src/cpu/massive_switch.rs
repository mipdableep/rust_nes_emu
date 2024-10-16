use super::CPU;
use crate::cpu::opcodes::OPCODES_METADATA;

#[allow(unused_variables)]
impl<'a> CPU<'a> {
    pub fn massive_switch(&mut self, op_code: u8) -> bool {
        self.program_counter += 1;

        let opcode_metadata = &OPCODES_METADATA[&op_code];
        assert_eq!(opcode_metadata.opcode, op_code);

        let addressing_mode = opcode_metadata.addressing_mode;
        let address = self.convert_mode_to_operand_mem_address(addressing_mode);
        let value = self.read_memory(address);
        let new_address = address;

        self.increase_cpu_idle_cycles(opcode_metadata.base_cycles);
        if opcode_metadata.increase_on_page_cross {
            if self.detect_page_cross(addressing_mode) {
                self.increase_cpu_idle_cycles(1);
            }
        }

        self.program_counter += opcode_metadata.bytes - 1;

        match op_code {
            ///////////////////////
            //// register_ops /////
            ///////////////////////

            // CPX : Compare X Register
            0xE0 => {
                self.CPX(value);
            }
            0xE4 => {
                self.CPX(value);
            }
            0xEC => {
                self.CPX(value);
            }

            // CPY : Compare Y Register
            0xC0 => {
                self.CPY(value);
            }
            0xC4 => {
                self.CPY(value);
            }
            0xCC => {
                self.CPY(value);
            }

            // DEX : Decrement X Register
            0xCA => {
                self.DEX();
            }

            // DEY : Decrement Y Register
            0x88 => {
                self.DEY();
            }

            // INC : Increment Memory
            0xE6 => {
                self.INC(address);
            }
            0xF6 => {
                self.INC(address);
            }
            0xEE => {
                self.INC(address);
            }
            0xFE => {
                self.INC(address);
            }

            // INX : Increment X Register
            0xE8 => {
                self.INX();
            }

            // INY : Increment Y Register
            0xC8 => {
                self.INY();
            }

            // LDA : Load Accumulator
            0xA9 => {
                self.LDA(value);
            }
            0xA5 => {
                self.LDA(value);
            }
            0xB5 => {
                self.LDA(value);
            }
            0xAD => {
                self.LDA(value);
            }
            0xBD => {
                self.LDA(value);
            }
            0xB9 => {
                self.LDA(value);
            }
            0xA1 => {
                self.LDA(value);
            }
            0xB1 => {
                self.LDA(value);
            }

            // LDX : Load X Register
            0xA2 => {
                self.LDX(value);
            }
            0xA6 => {
                self.LDX(value);
            }
            0xB6 => {
                self.LDX(value);
            }
            0xAE => {
                self.LDX(value);
            }
            0xBE => {
                self.LDX(value);
            }

            // LDY : Load Y Register
            0xA0 => {
                self.LDY(value);
            }
            0xA4 => {
                self.LDY(value);
            }
            0xB4 => {
                self.LDY(value);
            }
            0xAC => {
                self.LDY(value);
            }
            0xBC => {
                self.LDY(value);
            }

            // STA : Store Accumulator
            0x85 => {
                self.STA(address);
            }
            0x95 => {
                self.STA(address);
            }
            0x8D => {
                self.STA(address);
            }
            0x9D => {
                self.STA(address);
            }
            0x99 => {
                self.STA(address);
            }
            0x81 => {
                self.STA(address);
            }
            0x91 => {
                self.STA(address);
            }

            // STX : Store X Register
            0x86 => {
                self.STX(address);
            }
            0x96 => {
                self.STX(address);
            }
            0x8E => {
                self.STX(address);
            }

            // STY : Store Y Register
            0x84 => {
                self.STY(address);
            }
            0x94 => {
                self.STY(address);
            }
            0x8C => {
                self.STY(address);
            }

            // TAX : Transfer Accumulator to X
            0xAA => {
                self.TAX();
            }

            // TAY : Transfer Accumulator to Y
            0xA8 => {
                self.TAY();
            }

            // TSX : Transfer Stack Pointer to X
            0xBA => {
                self.TSX();
            }

            // TXA : Transfer X to Accumulator
            0x8A => {
                self.TXA();
            }

            // TXS : Transfer X to Stack Pointer
            0x9A => {
                self.TXS();
            }

            // TYA : Transfer Y to Accumulator
            0x98 => {
                self.TYA();
            }

            /////////////////////
            //// interrupts /////
            /////////////////////

            // RTI : Return from Interrupt
            0x40 => {
                self.RTI();
            }

            // BRK : Force Interrupt
            0x00 => {
                self.BRK();
                return false;
            }

            ////////////////
            //// no_op /////
            ////////////////

            // NOP : No Operation
            0xEA => {
                self.NOP();
            }

            ///////////////////////////
            //// arithmatic_logic /////
            ///////////////////////////

            // ADC : Add with Carry
            0x69 => {
                self.ADC(value);
            }
            0x65 => {
                self.ADC(value);
            }
            0x75 => {
                self.ADC(value);
            }
            0x6D => {
                self.ADC(value);
            }
            0x7D => {
                self.ADC(value);
            }
            0x79 => {
                self.ADC(value);
            }
            0x61 => {
                self.ADC(value);
            }
            0x71 => {
                self.ADC(value);
            }

            // AND : Logical AND
            0x29 => {
                self.AND(value);
            }
            0x25 => {
                self.AND(value);
            }
            0x35 => {
                self.AND(value);
            }
            0x2D => {
                self.AND(value);
            }
            0x3D => {
                self.AND(value);
            }
            0x39 => {
                self.AND(value);
            }
            0x21 => {
                self.AND(value);
            }
            0x31 => {
                self.AND(value);
            }

            // ASL : Arithmetic Shift Left
            0x0A => {
                self.ASL_accumulator();
            }
            0x06 => {
                self.ASL_memory(address);
            }
            0x16 => {
                self.ASL_memory(address);
            }
            0x0E => {
                self.ASL_memory(address);
            }
            0x1E => {
                self.ASL_memory(address);
            }

            // BIT : Bit Test
            0x24 => {
                self.BIT(value);
            }
            0x2C => {
                self.BIT(value);
            }

            // CMP : Compare
            0xC9 => {
                self.CMP(value);
            }
            0xC5 => {
                self.CMP(value);
            }
            0xD5 => {
                self.CMP(value);
            }
            0xCD => {
                self.CMP(value);
            }
            0xDD => {
                self.CMP(value);
            }
            0xD9 => {
                self.CMP(value);
            }
            0xC1 => {
                self.CMP(value);
            }
            0xD1 => {
                self.CMP(value);
            }

            // DEC : Decrement Memory
            0xC6 => {
                self.DEC(address);
            }
            0xD6 => {
                self.DEC(address);
            }
            0xCE => {
                self.DEC(address);
            }
            0xDE => {
                self.DEC(address);
            }

            // EOR : Exclusive OR
            0x49 => {
                self.EOR(value);
            }
            0x45 => {
                self.EOR(value);
            }
            0x55 => {
                self.EOR(value);
            }
            0x4D => {
                self.EOR(value);
            }
            0x5D => {
                self.EOR(value);
            }
            0x59 => {
                self.EOR(value);
            }
            0x41 => {
                self.EOR(value);
            }
            0x51 => {
                self.EOR(value);
            }

            // LSR : Logical Shift Right
            0x4A => {
                self.LSR_accumulator();
            }
            0x46 => {
                self.LSR_memory(address);
            }
            0x56 => {
                self.LSR_memory(address);
            }
            0x4E => {
                self.LSR_memory(address);
            }
            0x5E => {
                self.LSR_memory(address);
            }

            // ORA : Logical Inclusive OR
            0x09 => {
                self.ORA(value);
            }
            0x05 => {
                self.ORA(value);
            }
            0x15 => {
                self.ORA(value);
            }
            0x0D => {
                self.ORA(value);
            }
            0x1D => {
                self.ORA(value);
            }
            0x19 => {
                self.ORA(value);
            }
            0x01 => {
                self.ORA(value);
            }
            0x11 => {
                self.ORA(value);
            }

            // ROL : Rotate Left
            0x2A => {
                self.ROL_accumulator();
            }
            0x26 => {
                self.ROL_memory(address);
            }
            0x36 => {
                self.ROL_memory(address);
            }
            0x2E => {
                self.ROL_memory(address);
            }
            0x3E => {
                self.ROL_memory(address);
            }

            // ROR : Rotate Right
            0x6A => {
                self.ROR_accumulator();
            }
            0x66 => {
                self.ROR_memory(address);
            }
            0x76 => {
                self.ROR_memory(address);
            }
            0x6E => {
                self.ROR_memory(address);
            }
            0x7E => {
                self.ROR_memory(address);
            }

            // SBC : Subtract with Carry
            0xE9 => {
                self.SBC(value);
            }
            0xE5 => {
                self.SBC(value);
            }
            0xF5 => {
                self.SBC(value);
            }
            0xED => {
                self.SBC(value);
            }
            0xFD => {
                self.SBC(value);
            }
            0xF9 => {
                self.SBC(value);
            }
            0xE1 => {
                self.SBC(value);
            }
            0xF1 => {
                self.SBC(value);
            }

            ///////////////////////
            //// control_flow /////
            ///////////////////////

            // BCC : Branch if Carry Clear
            0x90 => {
                self.BCC(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BCS : Branch if Carry Set
            0xB0 => {
                self.BCS(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BEQ : Branch if Equal
            0xF0 => {
                self.BEQ(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BMI : Branch if Minus
            0x30 => {
                self.BMI(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BNE : Branch if Not Equal
            0xD0 => {
                self.BNE(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BPL : Branch if Positive
            0x10 => {
                self.BPL(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BVC : Branch if Overflow Clear
            0x50 => {
                self.BVC(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // BVS : Branch if Overflow Set
            0x70 => {
                self.BVS(new_address.wrapping_sub(1)); // we jump one to many since we already incremented the pc
            }

            // JMP : Jump
            0x4C => {
                self.JMP(address);
            }
            0x6C => {
                self.JMP(address);
            }

            // JSR : Jump to Subroutine
            0x20 => {
                self.JSR(address);
            }

            // RTS : Return from Subroutine
            0x60 => {
                self.RTS();
            }

            ////////////////////////
            //// stack_related /////
            ////////////////////////

            // PHA : Push Accumulator
            0x48 => {
                self.PHA();
            }

            // PHP : Push Processor Status
            0x08 => {
                self.PHP();
            }

            // PLA : Pull Accumulator
            0x68 => {
                self.PLA();
            }

            // PLP : Pull Processor Status
            0x28 => {
                self.PLP();
            }

            //////////////////////////
            //// status_register /////
            //////////////////////////

            // CLC : Clear Carry Flag
            0x18 => {
                self.CLC();
            }

            // CLD : Clear Decimal Mode
            0xD8 => {
                self.CLD();
            }

            // CLI : Clear Interrupt Disable
            0x58 => {
                self.CLI();
            }

            // CLV : Clear Overflow Flag
            0xB8 => {
                self.CLV();
            }

            // SEC : Set Carry Flag
            0x38 => {
                self.SEC();
            }

            // SED : Set Decimal Flag
            0xF8 => {
                self.SED();
            }

            // SEI : Set Interrupt Disable
            0x78 => {
                self.SEI();
            }
            _ => panic!("opcode {:} is not supported", op_code),
        }
        true
    }
}
