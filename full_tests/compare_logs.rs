mod gen_cpu_logs_utils;

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
}

static PARSE_OURS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r#"pc: (0x[a-f\d]+) opcode: (0x[a-f\d]+) A: (0x[a-f\d]+), X: (0x[a-f\d]+), Y: (0x[a-f\d]+), SP: (0x[a-f\d]+), Status: (0x[a-f\d]+)"#,
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
    })
}
static RESULTS_FILE_PATH: &str = "./full_tests/our_result.txt";
static TEST_FILE_PATH: &str = "./full_tests/nestest.nes";

fn main() -> Result<()> {
    gen_cpu_logs_utils::run_and_log_all(vec![
        TEST_FILE_PATH.to_string(),
        RESULTS_FILE_PATH.to_string(),
        "0xc6bd".to_string(),
    ]);

    let binding = std::fs::read_to_string(RESULTS_FILE_PATH.to_string()).unwrap();
    let mut our_lines = binding.lines();

    let binding = std::fs::read_to_string(
        "/home/ido/personal/rust/rust_nes_emu/full_tests/nestest_result_good.log",
    )
    .unwrap();
    let mut their_lines = binding.lines();

    for i in 0..5003 {
        let our = read_ours(&our_lines.next().unwrap())?;
        let their = read_theirs(&their_lines.next().unwrap())?;
        if our == their {
            if i % 256 == 0 {
                println!("passed line {i}");
            }
        } else {
            panic!("line failed: {i}\nour - {our:?}\ntheir - {their:?}");
        }
    }
    Ok(())
}
