use super::super::CPU;

#[allow(dead_code, non_snake_case)]
impl CPU {
    fn compare(&mut self, register: u8, operand: u8) {
        // compare register to operand and set flags
        if register == operand {
            self.set_negative(false);
            self.set_zero(true);
            self.set_carry(true);
            return;
        } else {
            self.set_zero(false);
            // set negative as bit 7 of register - operand
            self.set_negative(register.wrapping_sub(operand) >> 7 & 1 == 1);
            if register < operand {
                self.set_carry(false);
            } else {
                self.set_carry(true);
            }
        }
    }

    /// CMP A register
    pub fn CMP(&mut self, operand: u8) {
        self.compare(self.register_a, operand);
    }

    ///  Compare X Register
    pub fn CPX(&mut self) {
        todo!()
    }

    ///  Compare Y Register
    pub fn CPY(&mut self) {
        todo!()
    }

    ///  Decrement X Register
    pub fn DEX(&mut self) {
        todo!()
    }

    ///  Decrement Y Register
    pub fn DEY(&mut self) {
        todo!()
    }

    ///  Increment Memory
    pub fn INC(&mut self) {
        todo!()
    }

    ///  Increment X Register
    pub fn INX(&mut self) {
        todo!()
    }

    ///  Increment Y Register
    pub fn INY(&mut self) {
        todo!()
    }

    ///  Load Accumulator
    pub fn LDA(&mut self) {
        todo!()
    }

    ///  Load X Register
    pub fn LDX(&mut self) {
        todo!()
    }

    ///  Load Y Register
    pub fn LDY(&mut self) {
        todo!()
    }

    ///  Store Accumulator
    pub fn STA(&mut self) {
        todo!()
    }

    ///  Store X Register
    pub fn STX(&mut self) {
        todo!()
    }

    ///  Store Y Register
    pub fn STY(&mut self) {
        todo!()
    }

    ///  Transfer Accumulator to X
    pub fn TAX(&mut self) {
        todo!()
    }

    ///  Transfer Accumulator to Y
    pub fn TAY(&mut self) {
        todo!()
    }

    ///  Transfer Stack Pointer to X
    pub fn TSX(&mut self) {
        todo!()
    }

    ///  Transfer X to Accumulator
    pub fn TXA(&mut self) {
        todo!()
    }

    ///  Transfer X to Stack Pointer
    pub fn TXS(&mut self) {
        todo!()
    }

    ///  Transfer Y to Accumulator
    pub fn TYA(&mut self) {
        todo!()
    }
}
