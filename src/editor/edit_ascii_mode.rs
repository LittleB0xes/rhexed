
use crate::editor::{Editor, Mode};
use crossterm::event::KeyCode;

impl Editor {
    pub fn edit_ascii_input(&mut self, key_code: KeyCode) {
        if let KeyCode::Char(k) = key_code {
            self.buffer[self.cursor_index] = k as u8;
            self.cursor_index += 1;
            self.refresh = true;
        }
        match key_code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.refresh = true;
            }
            KeyCode::Left => {
                if self.cursor_index > 0 {
                    self.nibble_index = 0;
                    self.cursor_index -= 1;
                    self.refresh = true;
                }
            }
            KeyCode::Down => {
                if self.cursor_index < self.buffer.len() - 16 {
                    self.cursor_index += 16;
                    self.nibble_index = 0;
                    self.refresh = true;
                }
            }
            KeyCode::Up => {
                if self.cursor_index >= 16 {
                    self.nibble_index = 0;
                    self.cursor_index -= 16;
                    self.refresh = true;
                }
            }
            KeyCode::Right => {
                if self.cursor_index < self.buffer.len() - 1 {
                    self.nibble_index = 0;
                    self.cursor_index += 1;
                    self.refresh = true;
                }
            }
            _=> {}
        }

    }

}
