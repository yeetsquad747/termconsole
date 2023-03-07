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