use crossterm::event::KeyCode;
use crate::editor::{Editor, Mode};
impl Editor {
    pub fn help_inputs(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc | KeyCode::Char('?') => {
                self.mode = Mode::Normal;
                self.refresh = true;
            }
            _ => {}
        }

    }
}
