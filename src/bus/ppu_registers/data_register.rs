use std::ops::Deref;

#[derive(Debug, Eq, PartialEq)]
pub struct PPUDataReg(u8);

impl Deref for PPUDataReg {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PPUDataReg {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read_current_value(&self) -> u8 {
        self.0
    }

    pub fn update_current_value(&mut self, new_value: u8) {
        self.0 = new_value
    }
}
