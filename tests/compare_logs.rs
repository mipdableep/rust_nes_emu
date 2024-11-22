#[cfg(test)]
mod generate_cpu_logs {

    use nes_emulator::bus::Bus;
    use nes_emulator::cpu::CPU;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::u16;

    trait GenLogs {
        fn get_status_string(&mut self, opcode: u8, cycles: u64) -> String;
        fn run_and_write_to_file(&mut self, file_path: &Path, last_line: u16);
        fn load_test(&mut self, path: &Path);
    }

    impl<'a> GenLogs for CPU<'a> {
        fn get_status_string(&mut self, opcode: u8, cycles: u64) -> String {
            let pc_string = format!(" pc: {:#04x}", self.program_counter);
            let opcode_string = format!(" opcode: {:#02x}", opcode);
            let registers_string = format!(
                " A: {:#02x}, X: {:#02x}, Y: {:#02x}, SP: {:#02x}, Status: {:#02x}, Cycles: {:}",
                self.register_a,
                self.register_x,
                self.register_y,
                self.stack_pointer,
                self.status,
                cycles
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
            let mut cycles = 7;
            loop {
                let opcode = self.read_memory(self.program_counter);
                if self.bus.as_mut().unwrap().cpu_idle_cycles == 0 {
                    writeln!(file, "{}", self.get_status_string(opcode, cycles)).unwrap();
                }

                self.run_one_cycle();

                if self.program_counter == last_line {
                    // run only up to a specific line
                    return;
                }
                cycles += 1;
            }
        }

        fn load_test(&mut self, path: &Path) {
            let bytes = std::fs::read(path).unwrap();
            self.bus.as_mut().unwrap().cartridge.load_from_dump(&bytes);
            self.program_counter = 0xc000;
            // I don't know, but the tests start this way. Not sure if this is part of proper start up or not
            self.stack_pointer = 0xfd;
            self.status = 0x24;
        }
    }

    pub fn run_and_log_all(args: Vec<String>) {
        // let path = std::env::current_dir().unwrap();
        // println!("The current directory is {}", path.display());
        let mut bus = Bus::new();
        let mut cpu = CPU::new(&mut bus);
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
}

#[cfg(test)]
mod compare {

    use crate::generate_cpu_logs;
    use anyhow::Result;
    use once_cell::sync::Lazy;
    use regex::Regex;

    #[derive(PartialEq, Debug)]
    struct LogLine {
        pc: u16,
        opcode: u16,
        a: u16,
        x: u16,
        y: u16,
        sp: u16,
        p: u16,
        cycle_num: u32,
    }

    static PARSE_OURS_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
        r#"pc: (0x[a-f\d]+) opcode: (0x[a-f\d]+) A: (0x[a-f\d]+), X: (0x[a-f\d]+), Y: (0x[a-f\d]+), SP: (0x[a-f\d]+), Status: (0x[a-f\d]+), Cycles: (\d+)"#,
    ).unwrap()
    });

    static PARSE_THEIRS_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(
        r#"([A-F\d]{4})  ([A-F\d]{2}) (.{2}) (.{2})  ([A-Z]{3}) (.{27}) A:([A-F\d]{2}) X:([A-F\d]{2}) Y:([A-F\d]{2}) P:([A-F\d]{2}) SP:([A-F\d]{2}) PPU:(.{7}) CYC:(\d*)"#,
    ).unwrap()
    });

    fn read_ours(line: &str) -> Result<LogLine> {
        let line_args = PARSE_OURS_REGEX.captures(line).unwrap();

        Ok(LogLine {
            pc: u16::from_str_radix(&line_args[1][2..], 16).unwrap(),
            opcode: u16::from_str_radix(&line_args[2][2..], 16).unwrap(),
            a: u16::from_str_radix(&line_args[3][2..], 16).unwrap(),
            x: u16::from_str_radix(&line_args[4][2..], 16).unwrap(),
            y: u16::from_str_radix(&line_args[5][2..], 16).unwrap(),
            p: u16::from_str_radix(&line_args[7][2..], 16).unwrap(),
            sp: u16::from_str_radix(&line_args[6][2..], 16).unwrap(),
            cycle_num: u32::from_str_radix(&line_args[8], 10).unwrap(),
        })
    }

    fn read_theirs(line: &str) -> Result<LogLine> {
        let line_args = PARSE_THEIRS_REGEX.captures(line).unwrap();

        Ok(LogLine {
            pc: u16::from_str_radix(&line_args[1], 16).unwrap(),
            opcode: u16::from_str_radix(&line_args[2], 16).unwrap(),
            a: u16::from_str_radix(&line_args[7], 16).unwrap(),
            x: u16::from_str_radix(&line_args[8], 16).unwrap(),
            y: u16::from_str_radix(&line_args[9], 16).unwrap(),
            p: u16::from_str_radix(&line_args[10], 16).unwrap(),
            sp: u16::from_str_radix(&line_args[11], 16).unwrap(),
            cycle_num: u32::from_str_radix(&line_args[13], 10).unwrap(),
        })
    }
    static RESULTS_FILE_PATH: &str = "./tests/our_result.txt";
    static TEST_FILE_PATH: &str = "./tests/nestest.nes";
    static NESTEST_RESULT_GOOD: &str = "./tests/nestest_result_good.log";

    #[test]
    pub fn create_and_compare_logs() {
        generate_cpu_logs::run_and_log_all(vec![
            "".to_string(),
            TEST_FILE_PATH.to_string(),
            RESULTS_FILE_PATH.to_string(),
            "0xc6bd".to_string(),
        ]);

        let binding = std::fs::read_to_string(RESULTS_FILE_PATH.to_string()).unwrap();
        let mut our_lines = binding.lines();

        let binding = std::fs::read_to_string(NESTEST_RESULT_GOOD).unwrap();
        let mut their_lines = binding.lines();

        for i in 0..5003 {
            let our = read_ours(&our_lines.next().unwrap()).unwrap();
            let their = read_theirs(&their_lines.next().unwrap()).unwrap();
            if our == their {
                if i % 256 == 0 {
                    println!("passed line {i}");
                }
            } else {
                panic!("line failed: {i}\nour - {our:?}\ntheir - {their:?}");
            }
        }
    }
}
