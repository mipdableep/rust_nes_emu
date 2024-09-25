use super::super::CPU;

#[allow(non_snake_case)]
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
    pub fn CPX(&mut self, operand: u8) {
        self.compare(self.register_x, operand);
    }

    ///  Compare Y Register
    pub fn CPY(&mut self, operand: u8) {
        self.compare(self.register_y, operand);
    }

    ///  Decrement Memory
    pub fn DEC(&mut self, address: u16) {
        let new_mem_value: u8 = self.read_memory(address).wrapping_sub(1);
        self.write_memory(address, new_mem_value);
        self.set_zero_and_negative_flag(new_mem_value);
    }

    ///  Decrement X Register
    pub fn DEX(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.set_zero_and_negative_flag(self.register_x);
    }

    ///  Decrement Y Register
    pub fn DEY(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.set_zero_and_negative_flag(self.register_y);
    }

    ///  Increment Memory
    pub fn INC(&mut self, address: u16) {
        let new_mem_value: u8 = self.read_memory(address).wrapping_add(1);
        self.write_memory(address, new_mem_value);
        self.set_zero_and_negative_flag(new_mem_value);
    }

    ///  Increment X Register
    pub fn INX(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.set_zero_and_negative_flag(self.register_x);
    }

    ///  Increment Y Register
    pub fn INY(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.set_zero_and_negative_flag(self.register_y);
    }

    ///  Load Accumulator
    pub fn LDA(&mut self, operand: u8) {
        self.register_a = operand;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Load X Register
    pub fn LDX(&mut self, operand: u8) {
        self.register_x = operand;
        self.set_zero_and_negative_flag(self.register_x);
    }

    ///  Load Y Register
    pub fn LDY(&mut self, operand: u8) {
        self.register_y = operand;
        self.set_zero_and_negative_flag(self.register_y);
    }

    ///  Store Accumulator
    pub fn STA(&mut self, address: u16) {
        self.write_memory(address, self.register_a);
    }

    ///  Store X Register
    pub fn STX(&mut self, address: u16) {
        self.write_memory(address, self.register_x);
    }

    ///  Store Y Register
    pub fn STY(&mut self, address: u16) {
        self.write_memory(address, self.register_y)
    }

    ///  Transfer Accumulator to X
    pub fn TAX(&mut self) {
        self.register_x = self.register_a;
        self.set_zero_and_negative_flag(self.register_x);
    }

    ///  Transfer Accumulator to Y
    pub fn TAY(&mut self) {
        self.register_y = self.register_a;
        self.set_zero_and_negative_flag(self.register_y);
    }

    ///  Transfer Stack Pointer to X
    pub fn TSX(&mut self) {
        self.register_x = self.stack_pointer;
        self.set_zero_and_negative_flag(self.register_x);
    }

    ///  Transfer X to Accumulator
    pub fn TXA(&mut self) {
        self.register_a = self.register_x;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Transfer X to Stack Pointer
    pub fn TXS(&mut self) {
        self.stack_pointer = self.register_x;
    }

    ///  Transfer Y to Accumulator
    pub fn TYA(&mut self) {
        self.register_a = self.register_y;
        self.set_zero_and_negative_flag(self.register_a);
    }
}
