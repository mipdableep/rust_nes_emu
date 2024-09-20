use super::super::CPU;

pub fn convert_u8_to_i8_2sc(offset: u8) -> i8 {
    i8::from_ne_bytes(offset.to_ne_bytes())
}

fn change_pc_by_offset(cpu : &mut CPU, offset: u8) {
    cpu.program_counter = (cpu.program_counter as i16 + convert_u8_to_i8_2sc(offset) as i16) as u16;
}

#[allow(dead_code, non_snake_case)]
impl CPU {
    ///  Branch if Carry Clear
    pub fn BCC(&mut self, offset: u8) {
        if !self.get_status_c() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Carry Set
    pub fn BCS(&mut self, offset: u8) {
        if self.get_status_c() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Equal
    pub fn BEQ(&mut self) {
        todo!()
    }

    ///  Branch if Minus
    pub fn BMI(&mut self) {
        todo!()
    }

    ///  Branch if Not Equal
    pub fn BNE(&mut self) {
        todo!()
    }

    ///  Branch if Positive
    pub fn BPL(&mut self) {
        todo!()
    }

    ///  Branch if Overflow Clear
    pub fn BVC(&mut self) {
        todo!()
    }

    ///  Branch if Overflow Set
    pub fn BVS(&mut self) {
        todo!()
    }

    ///  Jump
    pub fn JMP(&mut self) {
        todo!()
    }

    ///  Jump to Subroutine
    pub fn JSR(&mut self) {
        todo!()
    }

    ///  Return from Subroutine
    pub fn RTS(&mut self) {
        todo!()
    }
}
