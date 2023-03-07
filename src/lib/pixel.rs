use crossterm::style::Color;

struct Pixel {
    foreground_color: Color,
    background_color: Color,
    text: &str
}