use crate::ui::Color;

#[derive(Default)]
pub struct Config {
    pub font: FontConfig,
    pub colors: ColorConfig,
}

pub struct FontConfig {
    pub normal_size: f32,
    pub large_size: f32,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            normal_size: 18.0,
            large_size: 24.0,
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
            background: Color::from_rgba(50, 50, 50, 255),
            background_second: Color::from_rgba(80, 80, 80, 255),
            foreground: Color::from_rgba(200, 200, 200, 255),
            foreground_second: Color::from_rgba(150, 150, 150, 255),
            primary: Color::from_rgba(50, 50, 200, 255),
            secondary: Color::from_rgba(50, 200, 50, 255),
        }
    }
}
