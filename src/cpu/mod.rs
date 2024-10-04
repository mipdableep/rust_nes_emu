mod cpu_operations;
#[cfg(test)]
mod cpu_operations_tests;
#[cfg(test)]
mod cpu_tests;
mod massive_switch;
pub mod mem_utils;
use crate::bus::memory_mapping_constants::PRG_ROM_START;
use crate::bus::Bus;

const STACK_END: u16 = 0x100;

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub bus: Bus,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0xff,
            bus: Bus::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.bus.cartridge.raw_load(program);
        self.program_counter = PRG_ROM_START;
    }

    pub fn get_status_n(&self) -> bool {
        self.status & 0b10000000 != 0
    }
    pub fn get_status_v(&self) -> bool {
        self.status & 0b01000000 != 0
    }
    pub fn get_status_b(&self) -> bool {
        self.status & 0b00010000 != 0
    }
    pub fn get_status_d(&self) -> bool {
        self.status & 0b00001000 != 0
    }
    pub fn get_status_i(&self) -> bool {
        self.status & 0b00000100 != 0
    }
    pub fn get_status_z(&self) -> bool {
        self.status & 0b00000010 != 0
    }
    pub fn get_status_c(&self) -> bool {
        self.status & 0b00000001 != 0
    }

    pub fn set_negative(&mut self, negative: bool) {
        if negative {
            self.status |= 0b10000000;
        } else {
            self.status &= !0b10000000;
        }
    }
    pub fn set_overflow(&mut self, overflow: bool) {
        if overflow {
            self.status |= 0b01000000;
        } else {
            self.status &= !0b01000000;
        }
    }
    pub fn set_decimal(&mut self, decimal: bool) {
        // the decimal flag is not used in operation (such as ADC), but still exists
        if decimal {
            self.status |= 0b00001000;
        } else {
            self.status &= !0b00001000;
        }
    }
    pub fn set_interrupt(&mut self, interrupt: bool) {
        if interrupt {
            self.status |= 0b00000100;
        } else {
            self.status &= !0b00000100;
        }
    }
    pub fn set_zero(&mut self, zero: bool) {
        if zero {
            self.status |= 0b00000010;
        } else {
            self.status &= !0b00000010;
        }
    }
    pub fn set_carry(&mut self, carry: bool) {
        if carry {
            self.status |= 0b00000001;
        } else {
            self.status &= !0b00000001;
        }
    }

    pub fn set_zero_and_negative_flag(&mut self, result: u8) {
        self.set_zero(result == 0);
        self.set_negative(result & 0x80 == 0x80);
    }

    pub fn stack_push(&mut self, value: u8) {
        self.write_memory(STACK_END + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    pub fn stack_pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.read_memory(STACK_END + self.stack_pointer as u16)
    }
    pub fn stack_push_u16(&mut self, value: u16) {
        let low = (value & 0x00ff) as u8;
        let high = ((value & 0xff00) >> 8) as u8;
        self.stack_push(high);
        self.stack_push(low);
    }
    pub fn stack_pull_u16(&mut self) -> u16 {
        let low = self.stack_pull() as u16;
        let high = self.stack_pull() as u16;
        high << 8 | low
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            if !self.massive_switch(opcode) {
                return;
            }
        }
    }
}
