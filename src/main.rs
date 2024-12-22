use std::cmp;
use std::env;
use std::io;

mod editor;
use crossterm::terminal;
use editor::Editor;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};



fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut show_title = true;
    let args: Vec<String> = env::args().collect();
    let mut editors: Vec<Editor> = Vec::new();
    let mut current_editor = 0;
    for file_number in 1..args.len() {
        editors.push(Editor::new(&args[file_number], file_number - 1));
    }

    let _ = enable_raw_mode();
    editors[current_editor].render(&mut stdout, show_title)?;
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
                } 
                else if e.code == KeyCode::Tab {
                    show_title = !show_title;
                    editors[current_editor].render(&mut stdout, show_title)?;

                } else {
                    editors[current_editor].update(e);
                }
            },
            Event::Resize(_,_ ) => editors[current_editor].render(&mut stdout, show_title)?,
            _ => {
            
            }
        }
        if editors[current_editor].refresh {
            editors[current_editor].render(&mut stdout, show_title)?;
        }
    }

    stdout
        .execute(Clear(ClearType::All))?
        .execute(cursor::Show)?
        .execute(cursor::MoveTo(0, 0))?;
    let _ = disable_raw_mode();

    Ok(())
}



