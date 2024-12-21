use std::env;
use std::io;

mod editor;
use editor::Editor;



use crossterm::{
    cursor,
    event::read,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};



fn main() -> io::Result<()> {


    let mut stdout = io::stdout();

    let args: Vec<String> = env::args().collect();
    let mut editor = Editor::new(&args[1]);

    let _ = enable_raw_mode();
    while !editor.exit {
        editor.render(&mut stdout)?;
        editor.update(read()?);
    }

    stdout
        .execute(Clear(ClearType::All))?
        .execute(cursor::Show)?
        .execute(cursor::MoveTo(0, 0))?;
    let _ = disable_raw_mode();

    Ok(())
}

