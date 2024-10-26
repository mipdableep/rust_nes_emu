#[derive(Debug, Eq, PartialEq)]
pub struct PPUDataReg {
    pub data_reg: u8,
}

impl PPUDataReg {
    pub fn new() -> Self {
        Self { data_reg: 0 }
    }

    pub fn read_current_value(&self) -> u8 {
        self.data_reg
    }

    pub fn update_current_value(&mut self, new_value: u8) {
        self.data_reg = new_value
    }
}
