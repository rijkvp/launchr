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

    pub fn premultiply(&self) -> Self {
        if self.alpha != 255 {
            Self {
                red: premultiply_u8(self.red, self.alpha),
                green: premultiply_u8(self.green, self.alpha),
                blue: premultiply_u8(self.blue, self.alpha),
                alpha: self.alpha,
            }
        } else {
            *self
        }
    }
}

fn premultiply_u8(c: u8, a: u8) -> u8 {
    let prod = u32::from(c) * u32::from(a) + 128;
    ((prod + (prod >> 8)) >> 8) as u8
}
