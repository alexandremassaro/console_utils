use crossterm::{cursor, execute, terminal};
use std::io::{self, stdout, Write};

pub fn move_cursor_to(line: u16) -> io::Result<()> {
    execute!(stdout(), cursor::MoveTo(0, line))?;
    Ok(())
}

pub fn clear_current_line() -> io::Result<()> {
    execute!(stdout(), cursor::MoveToColumn(0), terminal::Clear(terminal::ClearType::CurrentLine))?;
    Ok(())
}

pub fn get_cursor_position() -> io::Result<(u16, u16)> {
    cursor::position()
}

pub fn print_formatted(message: &str) -> io::Result<()> {
    print!("{}", message);
    stdout().flush()?;
    Ok(())
}

pub fn print_newline() -> io::Result<()> {
    println!();
    Ok(())
}

pub fn print_message(message: &str) -> io::Result<()> {
    println!("{}", message);
    Ok(())
}

pub fn hide_cursor() -> io::Result<()> {
    execute!(stdout(), cursor::Hide)?;
    Ok(())
}

pub fn show_cursor() -> io::Result<()> {
    execute!(stdout(), cursor::Show)?;
    Ok(())
}
