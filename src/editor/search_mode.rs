use crossterm::event::KeyCode;
use crate::editor::{Editor, Mode};

impl Editor {
    pub fn search_inputs(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc => {
                self.nibble_index = 0;
                self.mode = Mode::Normal;
                self.refresh = true;
            }
            KeyCode::Char('?') => {
                self.mode = Mode::Help;
                self.refresh = true;
            }
            _ => {}
        }
    }
}
