use std::io::{stdout};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
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

fn color_print_line(background_color: Color, foreground_color: Color, text: &str) -> Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(foreground_color),
        SetBackgroundColor(background_color),
        Print(text),
        Print("\n"),
        ResetColor,
    )?;
    Ok(())
}

struct Pixel {
    background_color: Color,
    foreground_color: Color,
    text: String,
}

fn main() {
    let result = color_print(Color::Black, Color::Cyan, "Hi there!");
    if result.is_err() {
        println!("Error: {:?}", result);
    }
}