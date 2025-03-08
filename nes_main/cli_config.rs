use std::path::PathBuf;

use nes_emulator::bus::Config;
use sdl2::keyboard::Keycode;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

macro_rules! write_and_flash {
    ($dst:expr, $($arg:tt)*) => {{
        let res = write!($dst, $($arg)*);
        $dst.flush().unwrap();
        res
    }};
}

const CONTROLLER: &str = r#"
╔════════════════════════════════════════╗
║     .                       .-.   .-.  ║
║   _| |_     select start   (   ) (   ) ║
║ -[_   _]-     ___   ___     '-'   '-'  ║
║    |_|       (___) (___)     B     A   ║
║     '                                  ║
╚════════════════════════════════════════╝
"#;

#[derive(Debug)]
enum Buttons {
    Up,
    Down,
    Left,
    Right,
    A,
    B,
    Select,
    Start,
}

fn configure_key(
    stdin: &mut termion::input::Keys<std::io::StdinLock>,
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    conf: &mut Config,
    button: Buttons,
) {
    write_and_flash!(
        stdout,
        "{}{}{}configuring button ( {button:?} ) - press a char or arrow to bind",
        termion::cursor::Goto(1, 12),
        termion::clear::AfterCursor,
        termion::cursor::Goto(1, 13),
    )
    .unwrap();

    // Read the next key press
    if let Some(Ok(key_event)) = stdin.next() {
        match key_event {
            Key::Char(c) => {
                if let Some(kc) = Keycode::from_name(&c.to_ascii_uppercase().to_string()) {
                    match button {
                        Buttons::Up => conf.up = kc,
                        Buttons::Down => conf.down = kc,
                        Buttons::Left => conf.left = kc,
                        Buttons::Right => conf.right = kc,
                        Buttons::A => conf.a = kc,
                        Buttons::B => conf.b = kc,
                        Buttons::Select => conf.select = kc,
                        Buttons::Start => conf.start = kc,
                    }
                    write_and_flash!(
                        stdout,
                        "{}{}{}mapped key ({}) to button {button:?}",
                        termion::cursor::Goto(1, 12),
                        termion::clear::AfterCursor,
                        termion::cursor::Goto(1, 13),
                        kc.to_string(),
                    )
                    .unwrap();
                }
            }
            Key::Up => {
                match button {
                    Buttons::Up => conf.up = Keycode::Up,
                    Buttons::Down => conf.down = Keycode::Up,
                    Buttons::Left => conf.left = Keycode::Up,
                    Buttons::Right => conf.right = Keycode::Up,
                    Buttons::A => conf.a = Keycode::Up,
                    Buttons::B => conf.b = Keycode::Up,
                    Buttons::Select => conf.select = Keycode::Up,
                    Buttons::Start => conf.start = Keycode::Up,
                }
                write_and_flash!(
                    stdout,
                    "{}{}{}mapped key ({}) to button {button:?}",
                    termion::cursor::Goto(1, 12),
                    termion::clear::AfterCursor,
                    termion::cursor::Goto(1, 13),
                    Keycode::Up.to_string(),
                )
                .unwrap();
            }
            Key::Down => {
                match button {
                    Buttons::Up => conf.up = Keycode::Up,
                    Buttons::Down => conf.down = Keycode::Down,
                    Buttons::Left => conf.left = Keycode::Down,
                    Buttons::Right => conf.right = Keycode::Down,
                    Buttons::A => conf.a = Keycode::Down,
                    Buttons::B => conf.b = Keycode::Down,
                    Buttons::Select => conf.select = Keycode::Down,
                    Buttons::Start => conf.start = Keycode::Down,
                }
                write_and_flash!(
                    stdout,
                    "{}{}{}mapped key ({}) to button {button:?}",
                    termion::cursor::Goto(1, 12),
                    termion::clear::AfterCursor,
                    termion::cursor::Goto(1, 13),
                    Keycode::Down.to_string(),
                )
                .unwrap();
            }
            Key::Left => {
                match button {
                    Buttons::Up => conf.up = Keycode::Up,
                    Buttons::Down => conf.down = Keycode::Left,
                    Buttons::Left => conf.left = Keycode::Left,
                    Buttons::Right => conf.right = Keycode::Left,
                    Buttons::A => conf.a = Keycode::Left,
                    Buttons::B => conf.b = Keycode::Left,
                    Buttons::Select => conf.select = Keycode::Left,
                    Buttons::Start => conf.start = Keycode::Left,
                }
                write_and_flash!(
                    stdout,
                    "{}{}{}mapped key ({}) to button {button:?}",
                    termion::cursor::Goto(1, 12),
                    termion::clear::AfterCursor,
                    termion::cursor::Goto(1, 13),
                    Keycode::Left.to_string(),
                )
                .unwrap();
            }
            Key::Right => {
                match button {
                    Buttons::Up => conf.up = Keycode::Up,
                    Buttons::Down => conf.down = Keycode::Right,
                    Buttons::Left => conf.left = Keycode::Right,
                    Buttons::Right => conf.right = Keycode::Right,
                    Buttons::A => conf.a = Keycode::Right,
                    Buttons::B => conf.b = Keycode::Right,
                    Buttons::Select => conf.select = Keycode::Right,
                    Buttons::Start => conf.start = Keycode::Right,
                }
                write_and_flash!(
                    stdout,
                    "{}{}{}mapped key ({}) to button {button:?}",
                    termion::cursor::Goto(1, 12),
                    termion::clear::AfterCursor,
                    termion::cursor::Goto(1, 13),
                    Keycode::Right.to_string(),
                )
                .unwrap();
            }
            _ => {
                write_and_flash!(
                    stdout,
                    "{}{}{}invalid key!",
                    termion::cursor::Goto(1, 12),
                    termion::clear::AfterCursor,
                    termion::cursor::Goto(1, 13),
                )
                .unwrap();
            }
        }
    }
}
fn save_config_to_file(output_path: PathBuf, config: Config) {
    let t_s = toml::to_string(&config).expect("should always work - a serde operation");
    match std::fs::write(&output_path, t_s) {
        Ok(_) => println!("writen default config to {output_path:?}"),
        Err(e) => println!("failed to write to file {output_path:?} - {e}"),
    }
}

pub fn create_config(output_path: PathBuf) {
    let mut conf = Config::default();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write_and_flash!(stdout, "{}", termion::clear::All).unwrap();

    for (i, s) in CONTROLLER.lines().enumerate() {
        write_and_flash!(stdout, "{}{}", termion::cursor::Goto(1, i as u16), s).unwrap();
    }

    write_and_flash!(
        stdout,
        "{}key bind configurator!{}",
        termion::cursor::Goto(1, 8),
        termion::cursor::Hide,
    )
    .unwrap();
    write_and_flash!(
        stdout,
        "{}press a key to bind: (arrows) | (a) | (b) | (p) for start | (o) for select{}press (q) to quit, (Ctrl-q) to quit without saving, press (s) to save, (v) to view config{}",
        termion::cursor::Goto(1, 10),
        termion::cursor::Goto(1, 11),
        termion::cursor::Goto(1, 13),
    )
    .unwrap();

    let mut keys = stdin.lock().keys(); // Lock stdin **only once**

    'outer: while let Some(Ok(k)) = keys.next() {
        match k {
            Key::Char('q') => break 'outer,
            Key::Char('a') => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::A),
            Key::Char('b') => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::B),
            Key::Up => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::Up),
            Key::Down => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::Down),
            Key::Left => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::Left),
            Key::Right => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::Right),
            Key::Char('o') => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::Select),
            Key::Char('p') => configure_key(&mut keys, &mut stdout, &mut conf, Buttons::Start),
            Key::Char('v') => {
                for (i, l) in toml::to_string(&conf).unwrap().lines().enumerate() {
                    write_and_flash!(stdout, "{}{}", termion::cursor::Goto(1, 15 + i as u16), l)
                        .unwrap();
                }
            }
            Key::Char('s') => {
                save_config_to_file(output_path.clone(), conf.clone());
            }
            Key::Ctrl('q') => {
                println!("quitting without saving!");
                write_and_flash!(stdout, "{}", termion::cursor::Show).unwrap();
                return;
            }
            _ => {
                write_and_flash!(
                    stdout,
                    "{}{}{}invalid key!",
                    termion::cursor::Goto(1, 12),
                    termion::clear::AfterCursor,
                    termion::cursor::Goto(1, 13),
                )
                .unwrap();
            }
        }
    }
    save_config_to_file(output_path, conf);

    write_and_flash!(stdout, "{}", termion::cursor::Show).unwrap();
}
