use std::io::{self, Stdout, Write,Read};
use std::fs::File;
use std::{cmp, usize};

use crossterm::event::KeyEvent;
use crossterm::style::{Color, SetColors};
use crossterm::terminal;
use crossterm::{
    cursor,
    event::KeyCode,
    queue,
    style::{
        Color::{DarkGrey, DarkYellow, Green, Red, Reset, White, DarkGreen},
        Colors, Print, PrintStyledContent, Stylize,
    },
    terminal::{Clear, ClearType},
    QueueableCommand,
};

const RHEXED: [&str; 6] = [
    "d8888b. db   db d88888b db    db d88888b d8888b.",
    "88  `8D 88   88 88'     `8b  d8' 88'     88  `8D",
    "88oobY' 88ooo88 88ooooo  `8bd8'  88ooooo 88   88",
    "88`8b   88~~~88 88~~~~~  .dPYb.  88~~~~~ 88   88",
    "88 `88. 88   88 88.     .8P  Y8. 88.     88  .8D",
    "88   YD YP   YP Y88888P YP    YP Y88888P Y8888D'",
];

const PAGE_SIZE: usize = 0x100;

#[derive(PartialEq)]
enum Mode {
    Normal,
    Edit,
    Selection,
    Jump
}

struct ColorProfile {
    ascii_fg: Color,
    cursor_fg: Color,
    cursor_bg: Color,
    selection_fg: Color,
    selection_bg: Color
}

pub struct Editor {
    pub id: usize,
    pub refresh: bool,
    pub exit: bool,
    mode: Mode,
    cursor_index: usize,
    cursor_start: usize,
    nibble_index:u8,
    page: usize,
    clipboard: Vec<u8>,
    buffer: Vec<u8>,
    jump_adress: u32,
    file_name: String,
    terminal_height: usize,
    page_size: usize

}

impl Editor {
    pub fn new(file_name: &String, id: usize) -> Editor {
        let mut f = File::open(file_name).unwrap();
        let terminal_height = terminal::size().unwrap().1 as usize;

        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        Editor{
            id,
            refresh: true,
            exit: false,
            mode: Mode::Normal,
            cursor_index: 0,
            cursor_start: 0,
            nibble_index: 0,
            page: 0,
            clipboard: Vec::new(),
            buffer: buf,
            jump_adress: 0,
            file_name: file_name.clone(),
            terminal_height,
            page_size: (terminal_height - 12) * 16,
        }
    }

    pub fn update(&mut self, key_event: KeyEvent) {
        match self.mode {
            Mode::Normal =>{
                    match key_event.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Char('h') | KeyCode::Left => {
                            if self.cursor_index > 0 {
                                self.nibble_index = 0;
                                self.cursor_index -= 1;
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if self.cursor_index < self.buffer.len() - 16 {
                                self.cursor_index += 16;
                                self.nibble_index = 0;
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if self.cursor_index >= 16 {
                                self.nibble_index = 0;
                                self.cursor_index -= 16;
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            if self.cursor_index < self.buffer.len() - 1 {
                                self.nibble_index = 0;
                                self.cursor_index += 1;
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('(') => {
                            self.cursor_index = self.cursor_index / 16 * 16;
                            self.nibble_index = 0;
                            self.refresh = true;
                        }
                        KeyCode::Char(')') => {
                            self.cursor_index = self.cursor_index / 16 * 16 + 15;
                            self.nibble_index = 0;
                            self.refresh = true;
                        }
                        KeyCode::Char('[') => {
                            self.cursor_index = self.page * self.page_size;
                            self.nibble_index = 0;
                            self.refresh = true;
                        }
                        KeyCode::Char(']') => {
                            self.cursor_index = self.page * self.page_size + self.page_size - 1;
                            self.nibble_index = 0;
                            self.refresh = true;
                        }
                        KeyCode::Char('b') => {
                            if self.page > 0 {

                                self.page -= 1;
                                self.cursor_index -= self.page_size;
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('n') => {
                            if self.page < self.buffer.len() / self.page_size {
                                self.page += 1;
                                self.cursor_index += self.page_size;
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('g') => {
                            self.cursor_index = 0;
                            self.refresh = true;
                            self.nibble_index = 0;
                        }
                        KeyCode::Char('G') => {
                            self.cursor_index = self.buffer.len() - 1;
                            self.nibble_index = 0;
                            self.refresh = true;
                        }
                        KeyCode::Char('i') => {
                            self.mode = Mode::Edit;
                            self.refresh = true;
                        }
                        KeyCode::Char('a') => {
                            self.buffer.insert(self.cursor_index + 1, 0);
                            self.refresh = true;
                        }
                        KeyCode::Char('x') => {
                            if self.buffer.len() > 0 {
                                self.clipboard.clear();
                                self.clipboard.push(self.buffer[self.cursor_index]);
                                self.buffer.remove(self.cursor_index);
                                self.refresh = true;
                            }
                        }
                        KeyCode::Char('y') => {
                            self.clipboard.clear();
                            if self.mode == Mode::Selection {
                                let clipboard_range = self.cursor_index - self.cursor_start + 1;
                                for n in 0..clipboard_range {
                                    self.clipboard.push(self.buffer[self.cursor_index - (clipboard_range - 1 - n)]);
                                }
                                self.mode = Mode::Normal;
                                self.refresh = true;
                            } else {
                                self.clipboard.push(self.buffer[self.cursor_index]);
                            }
                        }
                        KeyCode::Char('p') => {
                            for n in 0..self.clipboard.len() {
                                if self.cursor_index + n < self.buffer.len() {

                                    self.buffer[self.cursor_index + n] = self.clipboard[n];
                                }
                            }
                            self.refresh = true;
                        }
                        KeyCode::Char('v') => {
                            self.cursor_start = self.cursor_index;
                            self.mode = Mode::Selection;
                            self.refresh = true;
                        }
                        KeyCode::Char('w') => {
                            let mut f = File::create(&self.file_name).unwrap();
                            f.write(&self.buffer).expect("impossible to write file");
                        }
                        KeyCode::Char('J') => {
                            self.mode = Mode::Jump;
                            self.refresh = true;
                        }
                        KeyCode::Char('r') => {
                            self.reload();
                            self.refresh = true;
                        }
                        KeyCode::Esc => {
                            self.nibble_index = 0;
                            self.mode = Mode::Normal;
                            self.refresh = true;
                        }
                        _ => {}
                    }
            },
            Mode::Edit => {
                if let KeyCode::Char(k) = key_event.code {
                    if k as u8 >= 48 && k as u8 <= 57 {
                        let value = k as u8 - 48;

                        write_nibble(
                            &mut self.buffer,
                            self.cursor_index % 16 + 16 * (self.cursor_index / 16),
                            value,
                            self.nibble_index,
                        );
                        self.nibble_index += 1;
                        if self.nibble_index > 1 {
                            self.nibble_index = 0;
                            if self.cursor_index < self.buffer.len() - 1 {
                                self.cursor_index += 1;
                            }
                        }
                        self.refresh = true;
                    } else if k as u8 >= 97 && k as u8 <= 102 {
                        let value = k as u8 - 87;
                        write_nibble(
                            &mut self.buffer,
                            self.cursor_index % 16 + 16 * (self.cursor_index / 16),
                            value,
                            self.nibble_index,
                        );
                        self.nibble_index += 1;
                        if self.nibble_index > 1 {
                            self.nibble_index = 0;
                            if self.cursor_index < self.buffer.len() - 1 {
                                self.cursor_index += 1;
                            }
                        }
                        self.refresh = true;
                    }
                }
                match key_event.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.refresh = true;
                    }
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                }

            },
            Mode::Selection => {
                match key_event.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.refresh = true;
                    }
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('h') | KeyCode::Left => {
                        if self.cursor_index > 0 {
                            self.cursor_index -= 1;
                            self.refresh = true;
                        }
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        if self.cursor_index < self.buffer.len() - 16 {
                            self.cursor_index += 16;
                            self.refresh = true;
                        }
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        if self.cursor_index >= 16 {
                            self.cursor_index -= 16;
                            self.refresh = true;
                        }
                    }
                    KeyCode::Char('l') | KeyCode::Right => {
                        if self.cursor_index < self.buffer.len() - 1 {
                            self.cursor_index += 1;
                            self.refresh = true;
                        }
                    }
                    _ => {}
                }

            },
            Mode::Jump => {
                // Process commands
                match key_event.code {
                    KeyCode::Esc => {
                        self.mode = Mode::Normal;
                        self.refresh = true;
                    }
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Enter => {
                       self.cursor_index = self.jump_adress as usize;
                       self.jump_adress = 0;
                       self.mode = Mode::Normal;
                       self.refresh = true;
                    }
                    KeyCode::Backspace => {
                        self.jump_adress >>= 4;
                        self.refresh = true;
                    }
                    _ => {}
                }


                // Process the adress input 
                if let KeyCode::Char(k) = key_event.code {

                    // For digit 0 to 9 
                    if k as u32 >= 48 && k as u32 <= 57 {
                        let value = k as u32 - 48;
                        self.jump_adress <<= 4;
                        self.jump_adress += value;
                        
                    }
                    // For digit a to f
                    else if k as u32 >= 97 && k as u32 <= 102 {
                        let value = k as u32 - 87;
                        self.jump_adress <<= 4;
                        self.jump_adress += value;
                    }
                    self.refresh = true;
                }
            }
        }

        self.cursor_index = cmp::max(0, self.cursor_index);
        self.cursor_index = cmp::min(self.cursor_index, self.buffer.len() - 1);

    }

    pub fn render(&mut self, stdout: &mut Stdout, show_title: bool) -> io::Result<()> {
        self.terminal_height = terminal::size()?.1 as usize;
        if show_title {
            self.page_size = (self.terminal_height - 12) * 16;
        } else {
            self.page_size = (self.terminal_height - 6) * 16;

        }
        if self.cursor_index >= (self.page + 1) * self.page_size || self.cursor_index < self.page * self.page_size {
            self.page = self.cursor_index / self.page_size;
        }
        self.refresh = false;
        let color_profile: ColorProfile;
        match self.mode {
            Mode::Normal => {
                color_profile = ColorProfile {
                    ascii_fg: Color::DarkYellow,
                    cursor_fg: Color::DarkGrey,
                    cursor_bg: Color::Red,
                    selection_fg: Color::DarkGrey,
                    selection_bg: Color::DarkYellow

                }
            },
            Mode::Edit => {

                color_profile = ColorProfile {
                    ascii_fg: Color::DarkYellow,
                    cursor_fg: Color::DarkGrey,
                    cursor_bg: Color::Magenta,
                    selection_fg: Color::DarkGrey,
                    selection_bg: Color::DarkYellow

                }
            },
            Mode::Jump => {

                color_profile = ColorProfile {
                    ascii_fg: Color::DarkYellow,
                    cursor_fg: Color::DarkGrey,
                    cursor_bg: Color::Magenta,
                    selection_fg: Color::DarkGrey,
                    selection_bg: Color::DarkYellow

                }
            },
            Mode::Selection => { 

                color_profile = ColorProfile {
                    ascii_fg: Color::DarkYellow,
                    cursor_fg: Color::DarkGrey,
                    cursor_bg: Color::Magenta,
                    selection_fg: Color::DarkGrey,
                    selection_bg: Color::DarkYellow

                }
            }
        }
        stdout.queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0,0))?;

        // queue!(stdout, Clear(ClearType::UntilNewLine))?;

        let mut line: u16 = 0;
        if show_title {
            for line_text in RHEXED.iter() {
                queue!(
                    stdout,
                    cursor::MoveTo(0, line),
                    PrintStyledContent(line_text.magenta()),
                )?;
                line += 1;
            }
            queue!(
                stdout,
                cursor::MoveToNextLine(2))?;
        }

        if self.mode == Mode::Edit {
            queue!(
                stdout,
                cursor::MoveToColumn(30),
                PrintStyledContent("-- EDIT --".magenta()),
                )?;
        }
        queue!(
            stdout,
            cursor::MoveToNextLine(1),
            PrintStyledContent(format!("File {}: ", self.id).green()),
            PrintStyledContent(format!("{}", self.file_name).magenta()),
            cursor::MoveToNextLine(1),
            PrintStyledContent("Size : ".green()),
            PrintStyledContent(format!("{} bytes", self.buffer.len()).magenta()),
            PrintStyledContent("  -  Page : ".green()),
            PrintStyledContent(format!("{} / {}", self.page + 1 , self.buffer.len() / self.page_size + 1).magenta()),
            PrintStyledContent("  -  Address : ".green()),
            PrintStyledContent(format!("{:08x}", self.cursor_index).magenta()),
            PrintStyledContent(" / ".green()),
            PrintStyledContent(format!("{:08x}", self.buffer.len()).magenta()),
            cursor::MoveToNextLine(1)

            )?;

        let limit: usize = cmp::min(self.buffer.len(), (self.page + 1) * self.page_size);
        for i in (self.page * self.page_size)..limit {
            if i == 0 {
                stdout.queue(PrintStyledContent("00000000 : ".green()))?;
            }
            else if i % 16 == 0 && i != 0 {
                stdout.queue(PrintStyledContent(format!("{:08x} : ", i).green()))?;
            }

            if i == self.cursor_index {
                stdout.queue(SetColors(Colors::new(color_profile.cursor_fg, color_profile.cursor_bg)))?;
            } else if is_printable_code(self.buffer[i]) {
                stdout.queue(SetColors(Colors::new(color_profile.ascii_fg, Reset)))?;
            } else if self.mode == Mode::Selection && i >= self.cursor_start && i <= self.cursor_index {
                stdout.queue(SetColors(Colors::new(
                            color_profile.selection_fg,
                            color_profile.selection_bg)))?;
            } else if self.mode == Mode::Edit && is_printable_code(self.buffer[i]) {
                stdout.queue(SetColors(Colors::new(DarkGreen, Reset)))?;
            } else {
                stdout.queue(SetColors(Colors::new(Reset, Reset)))?;
            }
            stdout.queue(Print(format!("{:02x}", self.buffer[i])))?
                .queue(SetColors(Colors::new(Reset, Reset)))?
                .queue(Print(" "))?;


            // Ascii  Side bar
            if i % 16 == 15 || i == self.buffer.len() - 1 {
                stdout.queue(cursor::MoveToColumn(60))?
                    .queue(PrintStyledContent("|  ".green()))?;
                
                let line_index = i / 16;
                for c in 0..16 {
                    if line_index * 16 + c < self.buffer.len() {
                        let displayed_char = if is_printable_code(self.buffer[line_index * 16 + c]) {
                            self.buffer[line_index * 16 + c] as char
                        } else {
                            '.'
                        };

                        if line_index * 16 + c == self.cursor_index {
                            stdout.queue(SetColors(Colors::new( Reset, Reset)))?
                                .queue(Print(format!("{}", displayed_char)))?;
                        } else if is_printable_code(self.buffer[line_index * 16 + c]) {
                            stdout.queue(SetColors(Colors::new(
                                        Color::DarkYellow,
                                        Reset)))?
                                .queue(Print(format!("{}", displayed_char)))?;
                        } else {
                            stdout.queue(SetColors(Colors::new(Reset, Reset)))?
                                .queue(Print(format!("{}", displayed_char)))?;
                        }
                        // Reset Colors
                        stdout.queue(SetColors(Colors::new(Reset, Reset)))?;
                    }
                }
                stdout.queue(cursor::MoveToNextLine(1))?;
            }
        }
        if self.mode == Mode::Jump {
            stdout.queue(cursor::MoveToNextLine(1))?
                .queue(cursor::MoveToColumn(20))?
                .queue(PrintStyledContent("Jump to ".magenta()))?
                .queue(PrintStyledContent(format!("0x{:08x}", self.jump_adress).magenta()))?;
        }
        stdout.flush()?;
        Ok(())
    }

    fn reload(&mut self) {
        let mut f = File::open(&self.file_name).unwrap();
        self.buffer.clear();
        f.read_to_end(&mut self.buffer).unwrap();
    }

}

fn write_nibble(buffer: &mut Vec<u8>, position: usize, value: u8, nibble_hl: u8) {
    let nibble_bits: u8 = value << 4 * (1 - nibble_hl);
    let mask: u8 = 0x0F << 4 * nibble_hl;
    buffer[position] &= mask;
    buffer[position] |= nibble_bits;
}

fn is_printable_code(c: u8) -> bool {
    c >= 32 && c <= 126
}
