use super::super::CPU;


#[allow(dead_code, non_snake_case)]
impl CPU {
    ///  Add with Carry
    pub fn ADC(&mut self, operand: u8) {
        let res_as_u16: u16 = self.register_a as u16 + operand as u16;
        // check for carry
        self.set_carry( (res_as_u16 & 0x0100) == 0x0100);

        // check for overflow
        let is_operand_negative: bool = operand & 0x80 == 0x80;
        let is_a_negative: bool = self.register_a & 0x80 == 0x80;
        let is_result_negative: bool = res_as_u16 & 0x0080 == 0x80;
        // overflow occurred iff the sign of the result does not match the expected sign
        let overflow_occurred: bool = (is_operand_negative == is_a_negative) & (is_operand_negative != is_result_negative);
        self.set_overflow(overflow_occurred);

        // now we can set register a as the result. Don't forget to convert it properly!
        self.register_a = (res_as_u16 & 0x00FF) as u8;
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Logical AND
    pub fn AND(&mut self) {
        todo!()
    }

    ///  Arithmetic Shift Left
    pub fn ASL(&mut self) {
        todo!()
    }

    ///  Bit Test
    pub fn BIT(&mut self) {
        todo!()
    }

    ///  Compare
    pub fn CMP(&mut self) {
        todo!()
    }

    ///  Decrement Memory
    pub fn DEC(&mut self) {
        todo!()
    }

    ///  Exclusive OR
    pub fn EOR(&mut self) {
        todo!()
    }

    ///  Logical Shift Right
    pub fn LSR(&mut self) {
        todo!()
    }

    ///  Logical Inclusive OR
    pub fn ORA(&mut self) {
        todo!()
    }

    ///  Rotate Left
    pub fn ROL(&mut self) {
        todo!()
    }

    ///  Rotate Right
    pub fn ROR(&mut self) {
        todo!()
    }

    ///  Subtract with Carry
    pub fn SBC(&mut self) {
        todo!()
    }
}
