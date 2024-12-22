use std::cmp;
use std::env;
use std::io;


mod editor;
use editor::Editor;



use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};



fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let args: Vec<String> = env::args().collect();
    let mut editors: Vec<Editor> = Vec::new();
    let mut current_editor = 0;
    for file_number in 1..args.len() {
        editors.push(Editor::new(&args[file_number], file_number - 1));
    }

    let _ = enable_raw_mode();
    editors[current_editor].render(&mut stdout)?;
    while !editors[current_editor].exit {
        let event = read()?;
        match event {
            Event::Key(e) => {
                // editors[current_editor].update(e);
                if e.code == KeyCode::Char('B') {

                    current_editor = cmp::max(current_editor - 1, 0);
                    editors[current_editor].refresh = true;
                } 
                else if e.code == KeyCode::Char('N') {
                    current_editor = cmp::min(current_editor + 1, editors.len() - 1);
                    editors[current_editor].refresh = true;

                } else {
                    editors[current_editor].update(e);
                }
            },
            _ => {
            
            }
        }
        if editors[current_editor].refresh {
            editors[current_editor].render(&mut stdout)?;
        }
    }

    stdout
        .execute(Clear(ClearType::All))?
        .execute(cursor::Show)?
        .execute(cursor::MoveTo(0, 0))?;
    let _ = disable_raw_mode();

    Ok(())
}



