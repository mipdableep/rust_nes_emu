use nes_emulator::cpu::CPU;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::u16;
trait GenLogs {
    fn get_status_string(&mut self, opcode: u8) -> String;
    fn run_and_write_to_file(&mut self, file_path: &Path, last_line: u16);
    fn load_test(&mut self, path: &Path);
}

impl GenLogs for CPU {
    fn get_status_string(&mut self, opcode: u8) -> String {
        let pc_string = format!(" pc: {:#04x}", self.program_counter);
        let opcode_string = format!(" opcode: {:#02x}", opcode);
        let registers_string = format!(
            " A: {:#02x}, X: {:#02x}, Y: {:#02x}, SP: {:#02x}, Status: {:#02x}",
            self.register_a, self.register_x, self.register_y, self.stack_pointer, self.status
        );
        let mut final_string: String = "".to_owned();
        final_string.push_str(&pc_string);
        final_string.push_str(&opcode_string);
        final_string.push_str(&registers_string);

        final_string
    }

    fn run_and_write_to_file(&mut self, file_path: &Path, last_line: u16) {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(file_path)
            .unwrap();
        loop {
            let opcode = self.read_memory(self.program_counter);
            writeln!(file, "{}", self.get_status_string(opcode)).unwrap();

            if self.program_counter == last_line {
                // run only up to a specific line
                return;
            }
            if !self.massive_switch(opcode) {
                return;
            }
        }
    }

    fn load_test(&mut self, path: &Path) {
        let bytes = std::fs::read(path).unwrap();
        self.bus.cartridge.load_from_dump(&bytes);
        self.program_counter = 0xc000;
        // I don't know, but the tests start this way. Not sure if this is part of proper start up or not
        self.stack_pointer = 0xfd;
        self.status = 0x24;
    }
}

fn main() {
    // let path = std::env::current_dir().unwrap();
    // println!("The current directory is {}", path.display());
    let mut cpu = CPU::new();
    let args: Vec<String> = std::env::args().collect();
    let (test_file_path, result_file_path, line_upto) = match args.len() {
        4 => {
            let last_pc = args[3].as_str();
            let last_pc_without_fmt =  last_pc.strip_prefix("0x").unwrap_or(last_pc.strip_prefix("0X").unwrap_or(last_pc));
            let last_pc_u16 =  u16::from_str_radix(last_pc_without_fmt, 16).unwrap();
            (args[1].as_str(), args[2].as_str(), last_pc_u16)
        }
        _ => panic!("You must give exactly 3 additional arguments - the path to the test, the result file path and at which PC to abort (in base 16!), respectively"),
    };
    File::create(result_file_path).unwrap(); // create the file
    cpu.load_test(test_file_path.as_ref());
    cpu.run_and_write_to_file(result_file_path.as_ref(), line_upto);
}
