use std::io::{self, stdout, Read, Stdout, Write};
use std::fs::File;
use std::{cmp, usize};

use crossterm::event::{KeyCode, KeyEvent};
use crossterm::style::{Color, SetColors};
use crossterm::terminal;
use crossterm::{
    cursor,
    queue,
    style::{
        Color::{DarkGrey, DarkYellow, Magenta, Red, Reset, DarkGreen},
        Colors, Print, PrintStyledContent, Stylize,
    },
    terminal::{Clear, ClearType},
    QueueableCommand,
};


mod normal_mode;
mod edit_mode;
mod jump_mode;
mod selection_mode;
mod search_mode;
mod edit_ascii_mode;
mod help_mode;

const RHEXED: [&str; 6] = [
    "d8888b. db   db d88888b db    db d88888b d8888b.",
    "88  `8D 88   88 88'     `8b  d8' 88'     88  `8D",
    "88oobY' 88ooo88 88ooooo  `8bd8'  88ooooo 88   88",
    "88`8b   88~~~88 88~~~~~  .dPYb.  88~~~~~ 88   88",
    "88 `88. 88   88 88.     .8P  Y8. 88.     88  .8D",
    "88   YD YP   YP Y88888P YP    YP Y88888P Y8888D'",
];

const HELP: [&str; 26] = [
"      - hjkl or arrow     move                                            ",
"      - g                 move to the beginning of the file               ",
"      - G                 move to the end of the file                     ",
"      - (                 move to the beginning of the line               ",
"      - )                 move to the end of the line                     ",
"      - [                 move to the beginning of the page               ",
"      - ]                 move to the end of the page                     ",
"      - n                 go to the next page                             ",
"      - b                 go to the previous page                         ",
"      - N                 go to the next file                             ",
"      - B                 go to the previous file                         ",
"      - J                 go to a specified address                       ",
"      - a                 insert a byte at cursor position                ",
"      - x                 cut a byte                                      ",
"      - y                 copy a byte or a range of selected bytes        ",
"      - p                 paste a byte or a range of selected bytes       ",
"      - i                 insert mode                                     ",
"      - I                 insert mode (in ascii)                          ",
"      - <ESC>             quit insert mode                                ",
"      - <TAB>             show / hide title                               ",
"      - r                 reload file                                     ",
"      - w                 write file                                      ",
"      - q                 quit                                            ",
"      - ?                 help                                            ",
"                                                                          ",
"                        <ESC> or '?' to quit help                         "
];

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Search,
    Edit,
    AsciiEdit,
    Selection,
    Jump,
    Help
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
    search_pattern: Vec<u8>,
    search_result: Vec<u32>,
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
            search_pattern: Vec::new(),
            search_result: Vec::new(),
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
                self.normal_inputs(key_event.code);
            },
            Mode::Edit => {
                self.edit_inputs(key_event.code);
            },
            Mode::Selection => {
                self.selection_inputs(key_event.code);

            },
            Mode::Jump => {
                self.jump_inputs(key_event.code);
            }
            Mode::Search => {
                self.search_inputs(key_event.code);
            }
            Mode::AsciiEdit => {
                self.edit_ascii_input(key_event.code);
            }
            Mode::Help => {
                self.help_inputs(key_event.code);
            }
        }

        self.cursor_index = cmp::max(0, self.cursor_index);
        self.cursor_index = cmp::min(self.cursor_index, self.buffer.len() - 1);

    }

    pub fn render(&mut self, stdout: &mut Stdout, show_title: bool) -> io::Result<()> {
        self.terminal_height = terminal::size()?.1 as usize;
        if show_title && self.terminal_height > 20 {
            self.page_size = (self.terminal_height - 12) * 16;
        } else {
            self.page_size = (self.terminal_height - 6) * 16;

        }
        if self.cursor_index >= (self.page + 1) * self.page_size || self.cursor_index < self.page * self.page_size {
            self.page = self.cursor_index / self.page_size;
        }
        self.refresh = false;
        let color_profile = match self.mode {
            Mode::Normal | Mode::Search | Mode::Help=> {
                ColorProfile {
                    ascii_fg: DarkYellow,
                    cursor_fg: DarkGrey,
                    cursor_bg: Red,
                    selection_fg: DarkGrey,
                    selection_bg: DarkYellow
                }
            },
            Mode::Edit | Mode::AsciiEdit => {
                ColorProfile {
                    ascii_fg: DarkYellow,
                    cursor_fg: DarkGrey,
                    cursor_bg: Magenta,
                    selection_fg: DarkGrey,
                    selection_bg: DarkYellow
                }
            },
            Mode::Jump => {
                ColorProfile {
                    ascii_fg: DarkYellow,
                    cursor_fg: DarkGrey,
                    cursor_bg: Magenta,
                    selection_fg: DarkGrey,
                    selection_bg: DarkYellow
                }
            },
            Mode::Selection => { 
                ColorProfile {
                    ascii_fg: DarkYellow,
                    cursor_fg: DarkGrey,
                    cursor_bg: Magenta,
                    selection_fg: DarkGrey,
                    selection_bg: DarkYellow
                }
            }
        };
        stdout.queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0,0))?;

        let mut line: u16 = 0;
        if show_title && self.terminal_height > 20 {
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

        match self.mode {
            Mode::Edit => {
                queue!(
                    stdout,
                    cursor::MoveToColumn(30),
                    PrintStyledContent("-- EDIT --".magenta()),
                )?;
            }
            Mode::AsciiEdit => {
                queue!(
                    stdout,
                    cursor::MoveToColumn(27),
                    PrintStyledContent("-- ASCII EDIT --".magenta()),
                )?;
            }
            Mode::Jump => {
                queue!(
                    stdout,
                    cursor::MoveToColumn(20),
                    PrintStyledContent("Jump to ".magenta()),
                    PrintStyledContent(format!("0x{:08x}", self.jump_adress).magenta())
                    )?;
                }
            Mode::Search => {
                let mut searching_pattern: String = "".to_string();
                for s in self.search_pattern.iter_mut() {
                    searching_pattern = format!("{} {:02x}", searching_pattern, s);
                }
                queue!(
                    stdout,
                    cursor::MoveToColumn(20),
                    PrintStyledContent(format!("Search {}", searching_pattern).magenta())
                )?;

            }
            _ => {}
        }

        // Header info data
        queue!(
            stdout,
            cursor::MoveToNextLine(1),
            PrintStyledContent(format!("File {}: ", self.id).green()),
            PrintStyledContent(format!("{}", self.file_name).magenta()))?;

        if !self.search_pattern.is_empty() {
            queue!(
                stdout,
                PrintStyledContent("    Search result : ".green()),
                PrintStyledContent(format!("{}", self.search_result.len()).magenta()))?;
        }
        queue!(
            stdout,
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

            // Start address display
            if i % 16 == 0 {
                stdout.queue(SetColors(Colors::new(Reset, Reset)))?;
                stdout.queue(PrintStyledContent(format!("{:08x} : ", i).green()))?;
            }

            // Line of hex data display
            
            // First, color setting with context
            let mut fg_color = Reset;
            let mut bg_color = Reset;


            if i == self.cursor_index {
                fg_color = color_profile.cursor_fg;
                bg_color = color_profile.cursor_bg;
            } else if is_printable_code(self.buffer[i]) {
                fg_color = color_profile.ascii_fg;
            } else if self.mode == Mode::Selection && i >= self.cursor_start && i <= self.cursor_index {
                fg_color = color_profile.selection_fg;
                bg_color = color_profile.selection_bg;
            } else if self.mode == Mode::Edit && is_printable_code(self.buffer[i]) {
                fg_color = DarkGreen;
            }

            // Show seaarch result with background highlight
            // All the pattenr will be highlighted
            if self.search_pattern.len() != 0  && self.search_result.len() != 0 {
                // Affichage des résultat de recherche
                for s in 0..self.search_pattern.len() {
                    match self.search_result.iter().find(|res| **res + s as u32 == i as u32) {
                        Some(_) => {
                            fg_color = color_profile.selection_fg;
                            bg_color = color_profile.selection_bg;
                            if self.cursor_index == i {bg_color = color_profile.cursor_bg;}
                        }
                        None => {}
                    }
                }
            }

            stdout.queue(SetColors(Colors::new(fg_color, bg_color)))?;

            // match self.search_result.iter().find(|c| (**c) as usize == i) {
            //     Some(_) => {
            //         stdout.queue(SetColors(Colors::new(
            //                 color_profile.selection_fg,
            //                 color_profile.selection_bg)))?;
            //     }
            //     None => {}
            // }

            // Then, hex code display
            stdout.queue(Print(format!("{:02x}", self.buffer[i])))?
                .queue(SetColors(Colors::new(Reset, Reset)))?
                .queue(Print(" "))?;


            //  And, at the end of th 16 bytes line,  Char Side bar display
            if i % 16 == 15 || i == self.buffer.len() - 1 {


                // Separator
                stdout.queue(cursor::MoveToColumn(60))?
                    .queue(PrintStyledContent("|  ".green()))?;
                
                for c in 0..16 {

                    // Index of char to display
                    let char_index = (i / 16) * 16 + c;

                    //Set char if printable or '.' dot if not
                    if char_index < self.buffer.len() {
                        let displayed_char = if is_printable_code(self.buffer[char_index]) {
                            self.buffer[char_index] as char
                        } else {
                            '.'
                        };

                        // Set Char color
                        if char_index == self.cursor_index {
                            stdout.queue(SetColors(Colors::new(
                                        color_profile.cursor_fg,
                                        color_profile.cursor_bg)))?;
                        } else if is_printable_code(self.buffer[char_index]) {
                            stdout.queue(SetColors(Colors::new(
                                        DarkYellow,
                                        Reset)))?;
                        } else {
                            stdout.queue(SetColors(Colors::new(Reset, Reset)))?;
                        }

                        // Queue the char

                        stdout.queue(Print(format!("{}", displayed_char)))?;
                        // Reset Colors
                        // stdout.queue(SetColors(Colors::new(Reset, Reset)))?;
                    }
                }
                stdout.queue(cursor::MoveToNextLine(1))?;
            }
        } 

        if self.mode == Mode::Help {
            let mut line: u16 = 0;
            if show_title && self.terminal_height > 20 {
                for line_text in HELP.iter() {
                    queue!(
                        stdout,
                        cursor::MoveTo(5, line + 5),
                        PrintStyledContent(line_text.white()),
                    )?;
                    line += 1;
                }
                queue!(
                    stdout,
                    cursor::MoveToNextLine(2))?;
            }

        }

        stdout.queue(SetColors(Colors::new(Reset, Reset)))?;
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
    (c >= 32 && c <= 126) || (c > 127 && c < 255)
}
