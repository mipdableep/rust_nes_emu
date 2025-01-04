use super::CPU;
use crate::cpu::opcodes::{get_opcode_metadat_from_opcode, OpcodeMetadata};

#[allow(unused_variables)]
impl<'a> CPU<'a> {
    pub fn massive_switch(&mut self, op_code: u8) -> bool {
        self.program_counter += 1;

        let opcode_metadata: OpcodeMetadata = get_opcode_metadat_from_opcode(op_code);
        assert_eq!(opcode_metadata.opcode, op_code);

        let addressing_mode = opcode_metadata.addressing_mode;
        let address = self.convert_mode_to_operand_mem_address(addressing_mode);
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
                let value = self.read_memory(address);
                self.CPX(value);
            }
            0xE4 => {
                let value = self.read_memory(address);
                self.CPX(value);
            }
            0xEC => {
                let value = self.read_memory(address);
                self.CPX(value);
            }

            // CPY : Compare Y Register
            0xC0 => {
                let value = self.read_memory(address);
                self.CPY(value);
            }
            0xC4 => {
                let value = self.read_memory(address);
                self.CPY(value);
            }
            0xCC => {
                let value = self.read_memory(address);
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
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xA5 => {
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xB5 => {
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xAD => {
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xBD => {
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xB9 => {
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xA1 => {
                let value = self.read_memory(address);
                self.LDA(value);
            }
            0xB1 => {
                let value = self.read_memory(address);
                self.LDA(value);
            }

            // LDX : Load X Register
            0xA2 => {
                let value = self.read_memory(address);
                self.LDX(value);
            }
            0xA6 => {
                let value = self.read_memory(address);
                self.LDX(value);
            }
            0xB6 => {
                let value = self.read_memory(address);
                self.LDX(value);
            }
            0xAE => {
                let value = self.read_memory(address);
                self.LDX(value);
            }
            0xBE => {
                let value = self.read_memory(address);
                self.LDX(value);
            }

            // LDY : Load Y Register
            0xA0 => {
                let value = self.read_memory(address);
                self.LDY(value);
            }
            0xA4 => {
                let value = self.read_memory(address);
                self.LDY(value);
            }
            0xB4 => {
                let value = self.read_memory(address);
                self.LDY(value);
            }
            0xAC => {
                let value = self.read_memory(address);
                self.LDY(value);
            }
            0xBC => {
                let value = self.read_memory(address);
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
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x65 => {
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x75 => {
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x6D => {
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x7D => {
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x79 => {
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x61 => {
                let value = self.read_memory(address);
                self.ADC(value);
            }
            0x71 => {
                let value = self.read_memory(address);
                self.ADC(value);
            }

            // AND : Logical AND
            0x29 => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x25 => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x35 => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x2D => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x3D => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x39 => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x21 => {
                let value = self.read_memory(address);
                self.AND(value);
            }
            0x31 => {
                let value = self.read_memory(address);
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
                let value = self.read_memory(address);
                self.BIT(value);
            }
            0x2C => {
                let value = self.read_memory(address);
                self.BIT(value);
            }

            // CMP : Compare
            0xC9 => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xC5 => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xD5 => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xCD => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xDD => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xD9 => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xC1 => {
                let value = self.read_memory(address);
                self.CMP(value);
            }
            0xD1 => {
                let value = self.read_memory(address);
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
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x45 => {
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x55 => {
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x4D => {
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x5D => {
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x59 => {
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x41 => {
                let value = self.read_memory(address);
                self.EOR(value);
            }
            0x51 => {
                let value = self.read_memory(address);
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
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x05 => {
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x15 => {
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x0D => {
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x1D => {
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x19 => {
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x01 => {
                let value = self.read_memory(address);
                self.ORA(value);
            }
            0x11 => {
                let value = self.read_memory(address);
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
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xE5 => {
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xF5 => {
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xED => {
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xFD => {
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xF9 => {
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xE1 => {
                let value = self.read_memory(address);
                self.SBC(value);
            }
            0xF1 => {
                let value = self.read_memory(address);
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
