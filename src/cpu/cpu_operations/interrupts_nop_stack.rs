use super::super::CPU;

const BRK_ADDRESS: u16 = 0xfffe;

#[allow(non_snake_case)]
impl<'a> CPU<'a> {
    /////////////////////////////
    /////// STACK RELATED ///////
    /////////////////////////////

    ///  Push Accumulator
    pub fn PHA(&mut self) {
        self.stack_push(self.register_a);
    }

    ///  Push Processor Status
    pub fn PHP(&mut self) {
        self.stack_push_status();
    }

    ///  Pull Accumulator
    pub fn PLA(&mut self) {
        self.register_a = self.stack_pull();
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Pull Processor Status
    pub fn PLP(&mut self) {
        self.stack_pull_status();
    }

    ////////////////////////////
    /////// NO OPERATION ///////
    ////////////////////////////

    ///  No Operation
    pub fn NOP(&mut self) {
        return;
    }

    //////////////////////////
    /////// INTERRUPTS ///////
    //////////////////////////

    ///  Return from Interrupt
    pub fn RTI(&mut self) {
        self.stack_pull_status(); // for some wierd reason, bits 4 and 5 are probably ignored
        self.program_counter = self.stack_pull_u16();
    }

    ///  Force Interrupt
    pub fn BRK(&mut self) {
        self.stack_push_u16(self.program_counter.wrapping_add(1));
        self.stack_push(self.status | 0b00010000);
        self.program_counter = self.read_memory_2_bytes(BRK_ADDRESS);
    }
}
