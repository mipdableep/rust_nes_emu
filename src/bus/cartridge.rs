#[derive(Debug, PartialEq, Eq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
    Unloaded,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cartridge {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub screen_mirroring: Mirroring,
}

static NES_TAG: [u8; 4] = [b'N', b'E', b'S', 0x1A];

impl Cartridge {
    pub fn new() -> Self {
        Self {
            prg_rom: vec![],
            chr_rom: vec![],
            mapper: 0,
            screen_mirroring: Mirroring::Unloaded,
        }
    }
    pub fn load_from_dump(&mut self, raw_dump: &Vec<u8>) {
        let tag = &raw_dump[0..=3];
        let prg_rom_size_16kb = raw_dump[4];
        let chr_vrom_size_8kb = raw_dump[5];
        let control_byte_1 = raw_dump[6];
        let control_byte_2 = raw_dump[7];

        if tag != &NES_TAG {
            panic!("Wrong Nes tag used - {tag:?} should be {NES_TAG:?}");
        }
        if control_byte_2 & 0b1111 != 0 {
            panic!("control byte 2 bits 0..4 should be 0 for iNES_v1");
        }
        // ctrl b 1 has 4 lower bits, ctrl b 2 has 4 upper
        let mapper = (control_byte_2 & 0b11110000) | (control_byte_1 & 0b11110000) >> 4;
        // if ctrl b 1 bit 3 on - 4 screen. if not - match bit 0
        let screen_mirroring = if control_byte_1 & 0b1000 != 0 {
            Mirroring::FourScreen
        } else {
            match control_byte_1 & 0b1 {
                0 => Mirroring::Horizontal,
                1 => Mirroring::Vertical,
                _ => unreachable!(),
            }
        };
        let trainer_512_byte_exists = control_byte_1 & 0b100 != 0;

        let prg_rom_size_u8 = prg_rom_size_16kb as usize * 0x4000; // 0x4000 = 16k
        let chr_vrom_size_u8 = chr_vrom_size_8kb as usize * 0x2000; // 0x2000 = 16k

        let prg_rom_start = 16 + if trainer_512_byte_exists { 512 } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size_u8;

        assert!(prg_rom_size_u8 == 0x4000 || prg_rom_size_u8 == 0x8000); // assert the prg rom size is plausible

        self.prg_rom = raw_dump[prg_rom_start..(prg_rom_start + prg_rom_size_u8)].to_vec();
        self.chr_rom = raw_dump[chr_rom_start..(chr_rom_start + chr_vrom_size_u8)].to_vec();
        self.mapper = mapper;
        self.screen_mirroring = screen_mirroring;
    }
    pub fn raw_load(&mut self, program: Vec<u8>) {
        self.prg_rom = program;
    }

    pub fn read_prg_rom(&self, mut addr: u16) -> u8 {
        // get address mapped 0x0000 to 0x7FFF, and returns the corresponding ROM value
        if self.prg_rom.len() == 0x4000 {
            addr -= 0x4000;
        }
        self.prg_rom[addr as usize]
    }
}
