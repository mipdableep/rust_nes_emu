use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    tty::IsTty,
};
use nes_emulator::bus::Config;
use sdl2::keyboard::Keycode as sdl2_kc;

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

fn configure_key(stdout: &mut std::io::Stdout, conf: &mut Config, button: Buttons) {
    execute!(
        stdout,
        cursor::MoveTo(1, 12),
        Clear(ClearType::FromCursorDown),
        cursor::MoveTo(1, 13)
    )
    .unwrap();

    write_and_flash!(
        stdout,
        "Configuring button ({:?}) - press a key to bind",
        button
    )
    .unwrap();

    while let Ok(true) = event::poll(std::time::Duration::from_secs(10)) {
        if let Ok(event::Event::Key(KeyEvent {
            code, modifiers: _, ..
        })) = event::read()
        {
            let keycode = match code {
                KeyCode::Char(c) => sdl2_kc::from_name(&c.to_ascii_uppercase().to_string()),
                KeyCode::Up => Some(sdl2_kc::Up),
                KeyCode::Down => Some(sdl2_kc::Down),
                KeyCode::Left => Some(sdl2_kc::Left),
                KeyCode::Right => Some(sdl2_kc::Right),
                _ => None,
            };

            if let Some(kc) = keycode {
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

                execute!(
                    stdout,
                    cursor::MoveTo(1, 13),
                    Clear(ClearType::FromCursorDown)
                )
                .unwrap();
                write_and_flash!(
                    stdout,
                    "Mapped key ({}) to button {:?}",
                    kc.to_string(),
                    button
                )
                .unwrap();
                break;
            }
        }
    }
}

fn save_config_to_file(output_path: PathBuf, config: Config) {
    let t_s = toml::to_string(&config).expect("should always work - a serde operation");
    match std::fs::write(&output_path, t_s) {
        Ok(_) => println!("Written default config to {output_path:?}"),
        Err(e) => println!("Failed to write to file {output_path:?} - {e}"),
    }
}

pub fn create_config(output_path: PathBuf) {
    if !stdin().is_tty() {
        panic!("not a tty - calling config-cli should be done from terminal");
    }

    let mut conf = Config::default();
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    execute!(stdout, Clear(ClearType::All)).unwrap();

    for (i, s) in CONTROLLER.lines().enumerate() {
        execute!(stdout, cursor::MoveTo(1, i as u16)).unwrap();
        write_and_flash!(stdout, "{}", s).unwrap();
    }

    write_and_flash!(
        stdout,
        "{}Key bind configurator!{}",
        cursor::MoveTo(1, 8),
        cursor::Hide,
    )
    .unwrap();

    write_and_flash!(
        stdout,
        "{}Press a key to bind: (arrows) | (a) | (b) | (p) for start | (o) for select
        {}Press (q) to quit, (Ctrl-q) to quit without saving, press (s) to save, (v) to view config",
        cursor::MoveTo(1, 10),
        cursor::MoveTo(1, 11),
    )
    .unwrap();

    loop {
        if let Ok(event::Event::Key(KeyEvent {
            code, modifiers, ..
        })) = event::read()
        {
            match code {
                KeyCode::Char('q') if modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(
                        stdout,
                        cursor::MoveTo(1, 12),
                        Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();

                    println!("Quitting without saving!");
                    disable_raw_mode().unwrap();
                    execute!(stdout, cursor::Show).unwrap();
                    return;
                }
                KeyCode::Char('q') => break,
                KeyCode::Char('a') => configure_key(&mut stdout, &mut conf, Buttons::A),
                KeyCode::Char('b') => configure_key(&mut stdout, &mut conf, Buttons::B),
                KeyCode::Up => configure_key(&mut stdout, &mut conf, Buttons::Up),
                KeyCode::Down => configure_key(&mut stdout, &mut conf, Buttons::Down),
                KeyCode::Left => configure_key(&mut stdout, &mut conf, Buttons::Left),
                KeyCode::Right => configure_key(&mut stdout, &mut conf, Buttons::Right),
                KeyCode::Char('o') => configure_key(&mut stdout, &mut conf, Buttons::Select),
                KeyCode::Char('p') => configure_key(&mut stdout, &mut conf, Buttons::Start),
                KeyCode::Char('v') => {
                    for (i, l) in toml::to_string(&conf).unwrap().lines().enumerate() {
                        write_and_flash!(stdout, "{}{}", cursor::MoveTo(1, 15 + i as u16), l)
                            .unwrap();
                    }
                }
                KeyCode::Char('s') => {
                    execute!(
                        stdout,
                        cursor::MoveTo(1, 12),
                        Clear(ClearType::FromCursorDown),
                        cursor::MoveTo(1, 13),
                    )
                    .unwrap();
                    save_config_to_file(output_path.clone(), conf.clone())
                }
                _ => {
                    execute!(
                        stdout,
                        cursor::MoveTo(1, 12),
                        Clear(ClearType::FromCursorDown)
                    )
                    .unwrap();
                    write_and_flash!(stdout, "Invalid key!").unwrap();
                }
            }
        }
    }

    save_config_to_file(output_path, conf);
    disable_raw_mode().unwrap();
    execute!(stdout, cursor::Show).unwrap();
}
