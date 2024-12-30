
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
            _=> {}
        }

    }

}
