use super::super::CPU;
use crate::cpu::mem_utils::check_if_on_different_pages;

#[allow(non_snake_case)]
impl CPU {
    ///  Branch if Carry Clear
    pub fn BCC(&mut self, new_address: u16) {
        if !self.get_status_c() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Carry Set
    pub fn BCS(&mut self, new_address: u16) {
        if self.get_status_c() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Equal
    pub fn BEQ(&mut self, new_address: u16) {
        if self.get_status_z() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Minus
    pub fn BMI(&mut self, new_address: u16) {
        if self.get_status_n() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Not Equal
    pub fn BNE(&mut self, new_address: u16) {
        if !self.get_status_z() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Positive
    pub fn BPL(&mut self, new_address: u16) {
        if !self.get_status_n() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Overflow Clear
    pub fn BVC(&mut self, new_address: u16) {
        if !self.get_status_v() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
        }
    }

    ///  Branch if Overflow Set
    pub fn BVS(&mut self, new_address: u16) {
        if self.get_status_v() {
            self.increase_cpu_idle_cycles(
                match check_if_on_different_pages(self.program_counter, new_address) {
                    true => 2,
                    false => 1,
                },
            );
            self.program_counter = new_address;
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
        self.program_counter = self.stack_pull_u16().wrapping_add(1);
    }
}
