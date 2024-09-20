use super::super::CPU;

const STACK_END: u16 = 0x100;

#[allow(dead_code, non_snake_case)]
impl CPU {
    /////////////////////////////
    /////// STACK RELATED ///////
    /////////////////////////////

    ///  Push Accumulator
    pub fn PHA(&mut self) {
        self.write_memory(STACK_END + self.stack_pointer as u16, self.register_a);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    ///  Push Processor Status
    pub fn PHP(&mut self) {
        self.write_memory(STACK_END + self.stack_pointer as u16, self.status);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    ///  Pull Accumulator
    pub fn PLA(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.register_a = self.read_memory(STACK_END + self.stack_pointer as u16);
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Pull Processor Status
    pub fn PLP(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.status = self.read_memory(STACK_END + self.stack_pointer as u16);
    }

    ////////////////////////////
    /////// NO OPERATION ///////
    ////////////////////////////

    ///  No Operation
    pub fn NOP(&mut self) {
        return
    }

    //////////////////////////
    /////// INTERRUPTS ///////
    //////////////////////////

    ///  Return from Interrupt
    pub fn RTI(&mut self) {
        todo!()
    }

    ///  Force Interrupt
    pub fn BRK(&mut self) {
        return;
    }
}
