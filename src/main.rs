use std::io::{stdout};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result, ExecutableCommand, cursor,
};

fn color_print(background_color: Color, foreground_color: Color, text: &str) -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(foreground_color),
        SetBackgroundColor(background_color),
        Print(text),
        ResetColor,
    )?;
    Ok(())
}

fn main() {
    let mut stdout = stdout();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    let result = color_print(Color::Black, Color::Cyan, "Hi there!");
    if result.is_err() {
        println!("Error: {:?}", result);
    }
}
