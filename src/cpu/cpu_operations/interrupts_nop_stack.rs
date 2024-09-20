use super::super::CPU;

#[allow(dead_code, non_snake_case)]
impl CPU {
    /////////////////////////////
    /////// STACK RELATED ///////
    /////////////////////////////

    ///  Push Accumulator
    pub fn PHA(&mut self) {
        todo!()
    }

    ///  Push Processor Status
    pub fn PHP(&mut self) {
        todo!()
    }

    ///  Pull Accumulator
    pub fn PLA(&mut self) {
        todo!()
    }

    ///  Pull Processor Status
    pub fn PLP(&mut self) {
        todo!()
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
