use crate::render::Color;

pub struct Config {
    pub font_size: f32,
    pub colors: ColorConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_size: 22.0,
            colors: ColorConfig::default(),
        }
    }
}

pub struct ColorConfig {
    pub background: Color,
    pub background_second: Color,
    pub foreground: Color,
    pub foreground_second: Color,
    pub primary: Color,
    pub secondary: Color,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: Color::from_rgba8(50, 50, 50, 255),
            background_second: Color::from_rgba8(80, 80, 80, 255),
            foreground: Color::from_rgba8(200, 200, 200, 255),
            foreground_second: Color::from_rgba8(150, 150, 150, 255),
            primary: Color::from_rgba8(50, 50, 200, 255),
            secondary: Color::from_rgba8(50, 200, 50, 255),
        }
    }
}
