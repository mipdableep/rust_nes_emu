#[derive(Debug, Eq, PartialEq)]
pub struct PPUMaskRegister {
    value: u8,
    // 7  bit  0
    // ---- ----
    // BGRs bMmG
    // |||| ||||
    // |||| |||+- Greyscale (0: normal color, 1: greyscale)
    // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
    // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
    // |||| +---- 1: Enable background rendering
    // |||+------ 1: Enable sprite rendering
    // ||+------- Emphasize red (green on PAL/Dendy)
    // |+-------- Emphasize green (red on PAL/Dendy)
    // +--------- Emphasize blue
}

impl PPUMaskRegister {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn read(&self) -> u8 {
        self.value
    }

    pub fn write_byte(&mut self, value: u8) {
        self.value = value;
    }
}
