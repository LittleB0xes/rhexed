use crossterm::event::KeyCode;
use crate::editor::{Editor, Mode};
impl Editor {
    pub fn jump_inputs(&mut self, key_code: KeyCode) {
        match key_code {
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
        if let KeyCode::Char(k) = key_code {

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
