#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<cosmic_text::Color> for Color {
    fn from(color: cosmic_text::Color) -> Self {
        Self {
            red: color.r(),
            green: color.g(),
            blue: color.b(),
            alpha: color.a(),
        }
    }
}

impl Color {
    pub fn from_rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn to_array(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}
