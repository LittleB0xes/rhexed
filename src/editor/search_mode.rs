
use crossterm::event::KeyCode;
use crate::editor::{Editor, Mode, write_nibble};

impl Editor {
    pub fn search_inputs(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Esc => {
                self.nibble_index = 0;
                self.mode = Mode::Normal;
                self.refresh = true;
            }
            KeyCode::Backspace => {
                self.nibble_index = 0;
                self.search_pattern.pop();
                self.refresh = true;
            }
            KeyCode::Enter => {
                self.nibble_index = 0;
                if !self.search_pattern.is_empty() {
                    self.search();
                }
                self.mode = Mode::Normal;
                if !self.search_result.is_empty() {
                    self.cursor_index = self.search_result[0] as usize;
                }
                
                self.refresh = true;
            }
            _ => {}
        }

        if let KeyCode::Char(k) = key_code {
            let mut valid_entry = false;
            let mut value = 0;
            if k as u8 >= 48 && k as u8 <= 57 {
                value = k as u8 - 48;
                valid_entry = true;

            } else if k as u8 >= 97 && k as u8 <= 102 {
                value = k as u8 - 87;
                valid_entry = true;
            }
            if valid_entry {
                if self.nibble_index == 0 {
                    self.search_pattern.push(0);
                }
                let pattern_length = self.search_pattern.len();
                write_nibble(
                    &mut self.search_pattern,
                    pattern_length - 1,
                    value,
                    self.nibble_index,
                );
                self.nibble_index += 1;
                if self.nibble_index > 1 {
                    self.nibble_index = 0;
                }
                self.refresh = true;
            }


        }
    }

    fn search(&mut self) {

        self.search_result.clear();
        for (i,_) in self.buffer.iter().enumerate() {
            let mut flag = true;
            'pat: for (j,_) in self.search_pattern.iter().enumerate() {
                if self.buffer[i + j] != self.search_pattern[j] { 
                    flag = false;
                    break 'pat;
                }
            }
            if flag == true {self.search_result.push(i as u32)}
        }

    }
}
