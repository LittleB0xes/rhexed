use crate::editor::{Editor, Mode, write_nibble};
use crossterm::event::KeyCode;

impl Editor {
    pub fn  edit_inputs(&mut self, key_code: KeyCode) {
        if let KeyCode::Char(k) = key_code {
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
        match key_code {
            KeyCode::Esc => {
                self.mode = Mode::Normal;
                self.refresh = true;
            }
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }

}
