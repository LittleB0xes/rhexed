use crossterm::event::KeyCode;
use std::io::Write;
use std::fs::File;
use crate::editor::{Editor, Mode};

impl Editor {
    pub fn normal_inputs(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('h') | KeyCode::Left => {
                if self.cursor_index > 0 {
                    self.nibble_index = 0;
                    self.cursor_index -= 1;
                    self.refresh = true;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.buffer.len() > 16 && self.cursor_index < self.buffer.len() - 16 {
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
            KeyCode::Char('I') => {
                self.mode = Mode::AsciiEdit;
                self.refresh = true;
            }
            KeyCode::Char('a') => {
                self.buffer.insert(self.cursor_index, 0);
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
            KeyCode::Char('s') => {
                self.search_result = Vec::new();
                self.mode = Mode::Search;
                self.refresh = true;
            }
            KeyCode::Char('r') => {
                self.reload();
                self.refresh = true;
            }
            KeyCode::Char('?') => {
                self.mode = Mode::Help;
                self.refresh = true;
            }
            KeyCode::Esc => {
                self.nibble_index = 0;
                self.mode = Mode::Normal;
                self.refresh = true;
            }
            _ => {}
        }

    }

}
