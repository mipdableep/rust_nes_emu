mod cpu_operations;
#[cfg(test)]
mod cpu_operations_tests;
#[cfg(test)]
mod cpu_tests;
mod massive_switch;
pub mod mem_utils;
mod opcodes;

use crate::bus::memory_mapping_constants::PRG_ROM_START;
use crate::bus::Bus;
use crate::bus_mut;

const STACK_END: u16 = 0x100;
const NMI_ADDRESS: u16 = 0xFFFA;

#[derive(Debug)]
pub struct CPU<'a> {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    pub bus: Option<&'a mut Bus>,
}

impl<'a> CPU<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0xff,
            bus: Some(bus),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        bus_mut!(self).cartridge.raw_load(program);
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
    pub fn stack_push_status(&mut self) {
        // this function push the status register to the stack
        // it mostly regular push, but it ignores the B flag (always set it)
        // this also always set bit 6
        // this is because those are not actual flags - and created only when pushing status to stack (via brk, php or interrupt)
        self.stack_push(self.status | 0x30);
    }

    pub fn stack_push_status_nmi(&mut self) {
        // this function push the status register to the stack when triggered by NMI or IRQ
        // it mostly regular push, but it resets the B flag when pushing it
        // this also always set bit 6
        // this is because those are not actual flags - and created only when pushing status to stack (via NMI/IRQ in this case)
        self.stack_push((self.status | 0x20) & !0x10);
    }

    pub fn stack_pull_status(&mut self) {
        // this function pulls the status register from the stack
        // it mostly regular pull, but it ignores the B flag and bit 6 (just use the old values)
        self.status = (self.stack_pull() & !0x30) | (self.status & 0x30);
    }

    fn attend_nmi_interrupt(&mut self) {
        // attends to nmi interrupt
        // this loads the address from 0xFFFA, and attends this interrupt

        // push the pc and the status to the stack
        self.stack_push_u16(self.program_counter);
        self.stack_push_status_nmi();
        self.set_interrupt(false);

        // takes two(?) cycles
        self.increase_cpu_idle_cycles(2);
        self.program_counter = self.read_memory_2_bytes(NMI_ADDRESS);
    }

    pub fn increase_cpu_idle_cycles(&mut self, inc: u16) {
        // if we want to say certain action took x cycles, we just tell the cpu to rest in the next x cycles
        bus_mut!(self).cpu_idle_cycles += inc;
    }

    pub fn decrease_cpu_idle_cycles(&mut self, dec: u16) {
        // same thing, every cycle we decrease the number of cycles we need to wait
        bus_mut!(self).cpu_idle_cycles -= dec;
    }

    pub fn run_one_cycle(&mut self) -> bool {
        let mut return_value: bool = true;

        if bus_mut!(self).nmi_generated {
            // attend to nmi if needed
            self.attend_nmi_interrupt();
            bus_mut!(self).nmi_generated = false;
        }

        if bus_mut!(self).cpu_idle_cycles == 0 {
            let opcode = self.read_memory(self.program_counter);
            return_value = self.massive_switch(opcode);
        }
        self.decrease_cpu_idle_cycles(1);
        return_value
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            if bus_mut!(self).cpu_idle_cycles == 0 {
                let opcode = program[self.program_counter as usize];
                if !self.massive_switch(opcode) {
                    return;
                }
            } else {
                self.decrease_cpu_idle_cycles(1);
            }
        }
    }
}
