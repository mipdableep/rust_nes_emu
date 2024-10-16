use super::super::CPU;

#[allow(non_snake_case)]
impl<'a> CPU<'a> {
    ///  Clear Carry Flag
    pub fn CLC(&mut self) {
        self.set_carry(false);
    }

    ///  Clear Decimal Mode
    pub fn CLD(&mut self) {
        self.set_decimal(false);
    }

    ///  Clear Interrupt Disable
    pub fn CLI(&mut self) {
        self.set_interrupt(false);
    }

    ///  Clear Overflow Flag
    pub fn CLV(&mut self) {
        self.set_overflow(false);
    }

    ///  Set Carry Flag
    pub fn SEC(&mut self) {
        self.set_carry(true);
    }

    ///  Set Decimal Flag
    pub fn SED(&mut self) {
        self.set_decimal(true);
    }

    ///  Set Interrupt Disable
    pub fn SEI(&mut self) {
        self.set_interrupt(true);
    }
}
