use std::io::{self, Stdout, Write,Read};
use std::fs::File;
use std::cmp;

use crossterm::style::SetColors;
use crossterm::{
    cursor,
    event::{Event, KeyCode},
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

enum Mode {
    Normal,
    Insert,
    Selection,
    Jump
}

pub struct Editor {
    refresh: bool,
    pub exit: bool,
    edit_mode: bool,
    jump_mode: bool,
    selection_mode: bool,
    cursor_index: usize,
    cursor_start: usize,
    nibble_index:u8,
    page: usize,
    clipboard: Vec<u8>,
    buffer: Vec<u8>,
    jump_adress: u32,
    file_name: String,

}

impl Editor {
    pub fn new(file_name: &String) -> Editor {
        let mut f = File::open(file_name).unwrap();

        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        Editor{
            refresh: true,
            exit: false,
            edit_mode: false,
            jump_mode: false,
            selection_mode: false,
            cursor_index: 0,
            cursor_start: 0,
            nibble_index: 0,
            page: 0,
            clipboard: Vec::new(),
            buffer: buf,
            jump_adress: 0,
            file_name: file_name.clone(),
        }
    }

    pub fn update(&mut self, event: Event) {
        if self.edit_mode {
            match event {
                Event::Key(e) => {
                    if let KeyCode::Char(k) = e.code {
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
                }
                _ => {}
            }
        }
        if self.jump_mode {

            // Process commands
            match event {
                Event::Key(e) => match e.code {
                    KeyCode::Esc => {
                        self.jump_mode = false;
                        self.refresh = true;
                    }
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Enter => {
                       self.cursor_index = self.jump_adress as usize;
                       self.jump_adress = 0;
                       self.jump_mode = false;
                       self.refresh = true;
                    }
                    KeyCode::Backspace => {
                        self.jump_adress >>= 4;
                        self.refresh = true;
                    }
                    _ => {}
                }
                _ => {}
            }


            // Process the adress input 
            match event {
                Event::Key(e) => {
                    if let KeyCode::Char(k) = e.code {

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
                _ => {}
            }

        }
        else {
            match event {
                Event::Key(e) => match e.code {
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
                        let line = self.cursor_index / 16;
                        self.cursor_index = line * 16;
                        self.nibble_index = 0;
                        self.refresh = true;
                    }
                    KeyCode::Char(')') => {
                        let line = self.cursor_index / 16;
                        self.cursor_index = line * 16 + 15;
                        self.nibble_index = 0;
                        self.refresh = true;
                    }
                    KeyCode::Char('b') => {
                        if self.page > 0 {

                            self.page -= 1;
                            self.cursor_index -= PAGE_SIZE;
                            self.refresh = true;
                        }
                    }
                    KeyCode::Char('n') => {
                        if self.page < self.buffer.len() / PAGE_SIZE {
                            self.page += 1;
                            self.cursor_index += PAGE_SIZE;
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
                        self.edit_mode = true;
                        self.selection_mode = false;
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
                        if self.selection_mode {
                            let clipboard_range = self.cursor_index - self.cursor_start + 1;
                            for n in 0..clipboard_range {
                                self.clipboard.push(self.buffer[self.cursor_index - (clipboard_range - 1 - n)]);
                            }
                            self.selection_mode = false;
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
                        self.selection_mode = true;
                        self.refresh = true;
                    }
                    KeyCode::Char('w') => {
                        let mut f = File::create(&self.file_name).unwrap();
                        f.write(&self.buffer).expect("impossible to write file");
                    }
                    KeyCode::Char('J') => {
                        self.jump_mode = true;
                        self.refresh = true;
                    }
                    KeyCode::Esc => {
                        self.nibble_index = 0;
                        self.edit_mode = false;
                        self.selection_mode = false;
                        self.refresh = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if self.cursor_index >= self.buffer.len() {
            self.cursor_index = self.buffer.len() - 1;
        }



        self.cursor_index = cmp::max(0, self.cursor_index);
        self.cursor_index = cmp::min(self.cursor_index, self.buffer.len() - 1);

        if self.cursor_index >= (self.page + 1) * PAGE_SIZE || self.cursor_index < self.page * PAGE_SIZE {
            self.page = self.cursor_index / PAGE_SIZE;
            self.refresh = true;
        }

    }

    pub fn render(&mut self, stdout: &mut Stdout) -> io::Result<()> {

        stdout.queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0,0))?;

        // queue!(stdout, Clear(ClearType::UntilNewLine))?;

        let mut line: u16 = 0;
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
        if self.edit_mode {
            queue!(
                stdout,
                cursor::MoveToColumn(30),
                PrintStyledContent("-- EDIT --".magenta()),
                )?;
        }
        queue!(
            stdout,
            cursor::MoveToNextLine(1),
            PrintStyledContent("Size : ".green()),
            PrintStyledContent(format!("{} bytes", self.buffer.len()).magenta()),
            PrintStyledContent("  -  Page : ".green()),
            PrintStyledContent(format!("{} / {}", self.page, self.buffer.len() / PAGE_SIZE).magenta()),
            PrintStyledContent("  -  Address : ".green()),
            PrintStyledContent(format!("{:08x}", self.cursor_index).magenta()),
            cursor::MoveToNextLine(1)

            )?;

        let limit: usize = cmp::min(self.buffer.len(), (self.page + 1) * PAGE_SIZE);
        for i in (self.page * PAGE_SIZE)..limit {
            if i == 0 {
                stdout.queue(PrintStyledContent("00000000 : ".green()))?;
            }
            else if i % 16 == 0 && i != 0 {
                stdout.queue(PrintStyledContent(format!("{:08x} : ", i).green()))?;
            }

            if i == self.cursor_index && !self.edit_mode && !self.selection_mode {
                stdout.queue(SetColors(Colors::new(DarkGrey, Red)))?;
            } else if i == self.cursor_index && self.edit_mode && !self.selection_mode {
                stdout.queue(SetColors(Colors::new(DarkGrey, DarkYellow)))?;
            } else if !self.edit_mode && !self.selection_mode && is_printable_code(self.buffer[i]) {
                stdout.queue(SetColors(Colors::new(DarkYellow, Reset)))?;
            } else if self.selection_mode && i >= self.cursor_start && i <= self.cursor_index {
                stdout.queue(SetColors(Colors::new(White, Green)))?;
            } else if self.edit_mode && is_printable_code(self.buffer[i]) {
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
                            stdout.queue(SetColors(Colors::new(DarkGrey, Red)))?
                                .queue(Print(format!("{}", displayed_char)))?;
                        } else if is_printable_code(self.buffer[line_index * 16 + c]) {
                            stdout.queue(SetColors(Colors::new(DarkYellow, Reset)))?
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
        if self.jump_mode {
            stdout.queue(cursor::MoveToNextLine(1))?
                .queue(cursor::MoveToColumn(20))?
                .queue(PrintStyledContent("Jump to ".magenta()))?
                .queue(PrintStyledContent(format!("0x{:08x}", self.jump_adress).magenta()))?;
        }
        stdout.flush()?;
        Ok(())
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
