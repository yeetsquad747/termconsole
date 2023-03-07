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