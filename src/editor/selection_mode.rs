use crate::editor::{Editor, Mode};
use crossterm::event::KeyCode;

impl Editor {
    pub fn selection_inputs(&mut self, key_code: KeyCode) {
        match key_code {
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

    }

}
