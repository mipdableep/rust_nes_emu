use super::super::CPU;

pub fn convert_u8_to_i8_2sc(offset: u8) -> i8 {
    i8::from_ne_bytes(offset.to_ne_bytes())
}

fn change_pc_by_offset(cpu: &mut CPU, offset: u8) {
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
    pub fn BEQ(&mut self, offset: u8) {
        if self.get_status_z() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Minus
    pub fn BMI(&mut self, offset: u8) {
        if self.get_status_n() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Not Equal
    pub fn BNE(&mut self, offset: u8) {
        if !self.get_status_z() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Positive
    pub fn BPL(&mut self, offset: u8) {
        if !self.get_status_n() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Overflow Clear
    pub fn BVC(&mut self, offset: u8) {
        if !self.get_status_v() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Branch if Overflow Set
    pub fn BVS(&mut self, offset: u8) {
        if self.get_status_v() {
            change_pc_by_offset(self, offset);
        }
    }

    ///  Jump
    pub fn JMP(&mut self, address: u16) {
        self.program_counter = address;
    }

    ///  Jump to Subroutine
    pub fn JSR(&mut self, address: u16) {
        self.stack_push_u16(self.program_counter.wrapping_sub(1));
        self.program_counter = address;
    }

    ///  Return from Subroutine
    pub fn RTS(&mut self) {
        self.program_counter = self.stack_pull_u16();
    }
}
