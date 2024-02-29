use tiny_skia::Color;

#[derive(Default)]
pub struct Config {
    pub colors: ColorConfig,
}

pub struct ColorConfig {
    pub background: Color,
    pub background_second: Color,
    pub foreground: Color,
    pub foreground_second: Color,
    pub primary: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: Color::from_rgba8(50, 50, 50, 255),
            background_second: Color::from_rgba8(80, 80, 80, 255),
            foreground: Color::from_rgba8(200, 200, 200, 255),
            foreground_second: Color::from_rgba8(150, 150, 150, 255),
            primary: Color::from_rgba8(50, 50, 200, 255),
        }
    }
}
