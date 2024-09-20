use super::super::CPU;


#[allow(dead_code, non_snake_case)]
impl CPU {
    /////////////////////////////
    /////// STACK RELATED ///////
    /////////////////////////////

    ///  Push Accumulator
    pub fn PHA(&mut self) {
        self.stack_push(self.register_a);
    }

    ///  Push Processor Status
    pub fn PHP(&mut self) {
        self.stack_push(self.status);
    }

    ///  Pull Accumulator
    pub fn PLA(&mut self) {
        self.register_a = self.stack_pull();
        self.set_zero_and_negative_flag(self.register_a);
    }

    ///  Pull Processor Status
    pub fn PLP(&mut self) {
        self.status = self.stack_pull();
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
