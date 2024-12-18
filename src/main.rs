use std::env;
use std::io::{self, Stdout, Write};

use crossterm::style::SetColors;
use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue,
    style::{
        Color::{Red, Reset, White, DarkYellow},
        Colors, Print, PrintStyledContent, Stylize,
    },
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
        ExecutableCommand, QueueableCommand,
};

use std::fs::File;
use std::io::Read;

const RHEXED: [&str; 6] = [
    "d8888b. db   db d88888b db    db d88888b d8888b.",
    "88  `8D 88   88 88'     `8b  d8' 88'     88  `8D",
    "88oobY' 88ooo88 88ooooo  `8bd8'  88ooooo 88   88",
    "88`8b   88~~~88 88~~~~~  .dPYb.  88~~~~~ 88   88",
    "88 `88. 88   88 88.     .8P  Y8. 88.     88  .8D",
    "88   YD YP   YP Y88888P YP    YP Y88888P Y8888D'",
];

fn main() -> io::Result<()> {
    let mut refresh: bool = true;
    let mut exit: bool = false;
    let mut insert_mode: bool = false;
    let mut cursor_index = 0;
    let mut nibble_index = 0;
    let mut stdout = io::stdout();

    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1]).unwrap();

    let mut buffer = vec![];
    f.read_to_end(&mut buffer).unwrap();

    stdout.execute(Clear(ClearType::All))?;

    let _ = enable_raw_mode();
    while !exit {
        if refresh {
            render_screen(&mut stdout, &buffer, cursor_index, insert_mode)?;
            refresh = false;
        }
        let event = read()?;
        if insert_mode {
            match event {
                Event::Key(e) => {
                    if let KeyCode::Char(k) = e.code {
                        if k as u8 >= 48 && k as u8 <= 57 {
                            let value = k as u8 - 48; 

                            let nibble_bits: u8 = value << 4 * (1 - nibble_index);
                            let mask: u8 = 0x0F << 4 * nibble_index;
                            buffer[cursor_index % 16 + 16 * (cursor_index / 16)] &= mask;
                            buffer[cursor_index % 16 + 16 * (cursor_index / 16)] |= nibble_bits;
                            nibble_index += 1;
                            if nibble_index > 1 {
                                nibble_index = 0;
                                if cursor_index < buffer.len() - 1 {
                                    cursor_index += 1;
                                }
                            }
                            refresh = true;
                        }
                        else if k as u8 >= 97 && k as u8 <= 102 {
                            let value = k as u8 - 87; 
                            let nibble_bits: u8 = value << 4 * (1 - nibble_index);
                            let mask: u8 = 0x0F << 4 * nibble_index;
                            buffer[cursor_index % 16 + 16 * (cursor_index / 16)] &= mask;
                            buffer[cursor_index % 16 + 16 * (cursor_index / 16)] |= nibble_bits;
                            nibble_index += 1;
                            if nibble_index > 1 {
                                nibble_index = 0;
                                if cursor_index < buffer.len() - 1 {
                                    cursor_index += 1;
                                }
                            }
                            refresh = true;

                        }
                    }
                }
                _ => {}
                
            }
            
        }
        match event {
            Event::Key(e) => match e.code {
                KeyCode::Char('q') => exit = true,
                KeyCode::Char('h') | KeyCode::Left => {
                    if cursor_index > 0 {
                        cursor_index -= 1;
                        refresh = true;
                    }
                }
                KeyCode::Char('j') | KeyCode::Down=> { 
                    if cursor_index < buffer.len() - 17 {
                        cursor_index += 16;
                        refresh = true;
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if cursor_index >= 16 {
                        cursor_index -= 16;
                        refresh = true;
                    }
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    if cursor_index < buffer.len() - 1 {
                        cursor_index += 1;
                        refresh = true;
                    }
                }
                KeyCode::Char('0') => {
                    let line = cursor_index / 16;
                    cursor_index = line * 16;
                    refresh = true;
                }
                KeyCode::Char('$') => {
                    let line = cursor_index / 16;
                    cursor_index = line * 16 + 15;
                    refresh = true;
                }
                KeyCode::Char('g') => {
                    cursor_index = 0;
                    refresh = true;
                }
                KeyCode::Char('G') => {
                    cursor_index = buffer.len() - 1;
                    refresh = true;
                }
                KeyCode::Char('i') => {
                    insert_mode = true;
                    refresh = true;
                }
                KeyCode::Char('w') => {
                    f = File::create(&args[1]).unwrap();
                    f.write(&buffer).expect("impossible to write file");

                }
                KeyCode::Esc => {
                    insert_mode = false;
                    refresh = true;
                }
                _ => {}
            },
            _ => {}
        }
    }

    stdout
        .execute(Clear(ClearType::All))?
        .execute(cursor::Show)?
        .execute(cursor::MoveTo(0, 0))?;
    let _ = disable_raw_mode();


    Ok(())
}

fn render_screen(stdout: &mut Stdout, buffer: &Vec<u8>, cursor_index: usize, edit_mode: bool) -> io::Result<()> {
    queue!(stdout, Clear(ClearType::UntilNewLine), cursor::Hide)?;

    let mut line: u16 = 0;
    for line_text in RHEXED.iter() {
        queue!(
            stdout,
            cursor::MoveTo(0, line),
            PrintStyledContent(line_text.magenta()),
            cursor::MoveToNextLine(1)
        )?;
        line += 1;
    }
    queue!(stdout, cursor::MoveToNextLine(1))?;
    let mut char_index = 0;

    let mut char_line = 0;
    stdout.queue(PrintStyledContent("00000000 : ".green()));
    for (i, byte) in buffer.iter().enumerate() {
        if i % 16 == 0 && i != 0 {
            // Jump to the next line
            char_line += 1;
            char_index = 0;
            queue!(
                stdout,
                cursor::MoveToNextLine(1),
                PrintStyledContent(format!("{:08x} : ", i).green())
            )?;
        }

        if i == cursor_index && !edit_mode {
            queue!(stdout, SetColors(Colors::new(White, Red)))?;
        } else if i == cursor_index && edit_mode {
            queue!(stdout, SetColors(Colors::new(White, DarkYellow)))?;

        } else {
            queue!(stdout, SetColors(Colors::new(Reset, Reset)))?;
        }
        queue!(stdout, Print(format!("{:02x}", byte)))?;
        queue!(stdout, SetColors(Colors::new(Reset, Reset)), Print(" "))?;

        if i % 16 == 15 {
            queue!(stdout, PrintStyledContent("  |  ".green()))?;
            for c in 0..16 {
                if buffer[char_line * 16 + c] >= 32 && buffer[char_line * 16 + c] <= 126 {
                    queue!(
                        stdout,
                        Print(format!("{}", buffer[char_line * 16 + c] as char))
                    )?;
                } else {
                    queue!(stdout, Print("."))?;
                }
            }
        }

        char_index += 1;
    }
    stdout.flush()?;
    Ok(())
}
