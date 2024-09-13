mod cpu_operations;
#[cfg(test)]
mod cpu_tests;
mod massive_switch;
pub mod mem_utils;

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
    pub register_y: u8,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            stack_pointer: 0,
            memory: [0; 0xFFFF],
        }
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
    pub fn set_zero_and_negative_flag(&mut self, result: u8) {
        if result != 0 {
            // change zero flag
            self.status &= 0b11111101
        } else {
            self.status |= 0b00000010
        }
        if result & 0x80 != 0 {
            // change negative flag
            self.status &= 0b11111110
        } else {
            self.status |= 0b00000001
        }
    }
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter];
            self.program_counter += 1;
            if !self.massive_switch(opcode, &program){
                return;
            }
        }
    }
}
