use super::super::CPU;

#[allow(dead_code, non_snake_case)]
impl CPU {
    ///  Add with Carry
    pub fn ADC(&mut self, operand: u8) {
        let res_as_u16: u16 = self.register_a as u16 + operand as u16;
        // check for carry
        self.set_carry((res_as_u16 & 0x0100) == 0x0100);

        // check for overflow
        let is_operand_negative: bool = operand & 0x80 == 0x80;
        let is_a_negative: bool = self.register_a & 0x80 == 0x80;
        let is_result_negative: bool = res_as_u16 & 0x0080 == 0x80;
        // overflow occurred iff the sign of the result does not match the expected sign
        let overflow_occurred: bool =
            (is_operand_negative == is_a_negative) & (is_operand_negative != is_result_negative);
        self.set_overflow(overflow_occurred);

        // now we can set register a as the result. Don't forget to convert it properly!
        self.register_a = (res_as_u16 & 0x00FF) as u8;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Logical AND
    pub fn AND(&mut self, operand: u8) {
        self.register_a &= operand;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Arithmetic Shift Left
    pub fn ASL_accumulator(&mut self) {
        let should_carry: bool = self.register_a & 0x80 == 0x80;
        self.register_a <<= 1;
        self.set_carry(should_carry);
        self.set_zero_and_negative_flag(self.register_a)
    }

    pub fn ASL_memory(&mut self, address: u16) {
        let address_value = self.read_memory(address);
        let should_carry: bool = address_value & 0x80 == 0x80;
        self.write_memory(address, address_value << 1);
        self.set_carry(should_carry);
        self.set_zero_and_negative_flag(self.read_memory(address));
    }

    ///  Bit Test
    pub fn BIT(&mut self, operand: u8) {
        // this command "performs" and between the operand and reg_a, but does not store the values
        // (only set flags)
        let operand_bit_7 = (operand & 0x80) == 0x80;
        let operand_bit_6 = (operand & 0x40) == 0x40;
        let is_result_zero = (operand & self.register_a) == 0;
        self.set_zero(is_result_zero);
        self.set_overflow(operand_bit_6);
        self.set_negative(operand_bit_7);
    }

    ///  Compare
    pub fn CMP(&mut self, operand: u8) {
        // compare reg_a to operand and set flags
        if self.register_a == operand {
            self.set_negative(false);
            self.set_zero(true);
            self.set_carry(true);
            return;
        } else {
            self.set_zero(false);
            // set negative as bit 7 of reg_a - operand
            self.set_negative(self.register_a.wrapping_sub(operand) >> 7 & 1 == 1);
            if self.register_a < operand {
                self.set_carry(false);
            } else {
                self.set_carry(true);
            }
        }
    }

    ///  Decrement Memory
    pub fn DEC(&mut self, address: u16) {
        let new_mem_value: u8 = self.read_memory(address).wrapping_sub(1);
        self.write_memory(address, new_mem_value);
        self.set_zero_and_negative_flag(new_mem_value);
    }

    ///  Exclusive OR
    pub fn EOR(&mut self, operand: u8) {
        self.register_a ^= operand;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Logical Shift Right
    pub fn LSR_accumulator(&mut self) {
        self.set_carry(self.register_a & 0b1 == 1);
        self.register_a = self.register_a >> 1;
        self.set_zero_and_negative_flag(self.register_a);
    }

    pub fn LSR_memory(&mut self, address: u16) {
        let address_value = self.read_memory(address);
        self.set_carry(address_value & 0b1 == 1);
        let res = address_value >> 1;
        self.write_memory(address, res);
        self.set_zero_and_negative_flag(res);
    }

    ///  Logical Inclusive OR
    pub fn ORA(&mut self, operand: u8) {
        self.register_a |= operand;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Rotate Left
    pub fn ROL(&mut self) {
        todo!()
    }
    ///  Rotate Right accumulator
    pub fn ROR_accumulator(&mut self) {
        let bit7: u8 = match self.get_status_c() {
            true => 1 << 7,
            false => 0,
        };
        self.set_carry(match self.register_a & 0b1 {
            0 => false,
            1 => true,
            _ => unreachable!(),
        });
        self.register_a = (self.register_a >> 1) | bit7;
        self.set_zero_and_negative_flag(self.register_a);
    }

    // Rotate right memory
    pub fn ROR_memory(&mut self, address: u16) {
        let initial = self.read_memory(address);

        let bit7: u8 = match self.get_status_c() {
            true => 1 << 7,
            false => 0,
        };
        self.set_carry(match initial & 0b1 {
            0 => false,
            1 => true,
            _ => unreachable!(),
        });
        let res = (initial >> 1) | bit7;
        self.write_memory(address, res);
        self.set_zero_and_negative_flag(res);
    }

    ///  Subtract with Carry
    pub fn SBC(&mut self) {
        todo!()
    }
}
