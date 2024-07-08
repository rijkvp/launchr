#[derive(Debug, Clone, Copy)]
pub struct Color([u8; 4]);

impl From<cosmic_text::Color> for Color {
    fn from(color: cosmic_text::Color) -> Self {
        Self::from_rgba(color.r(), color.g(), color.b(), color.a())
    }
}

impl Color {
    #[inline]
    pub fn red(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn green(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn blue(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn alpha(&self) -> u8 {
        self.0[3]
    }

    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self([red, green, blue, alpha])
    }

    pub fn to_array(&self) -> [u8; 4] {
        self.0
    }

    #[inline]
    pub fn premultiply(&self) -> Self {
        self.premultiply_with(self.alpha())
    }

    #[inline]
    pub fn premultiply_with(&self, alpha: u8) -> Self {
        if alpha != 255 {
            Self::from_rgba(
                scale_u8(self.red(), alpha),
                scale_u8(self.green(), alpha),
                scale_u8(self.blue(), alpha),
                alpha,
            )
        } else {
            *self
        }
    }
}

#[inline]
pub fn scale_u8(c: u8, a: u8) -> u8 {
    let p = u32::from(c) * u32::from(a) + 128;
    ((p + (p >> 8)) >> 8) as u8
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multiply() {
        assert_eq!(super::scale_u8(0, 0), 0);
        assert_eq!(super::scale_u8(0, 255), 0);
        assert_eq!(super::scale_u8(255, 0), 0);
        assert_eq!(super::scale_u8(255, 255), 255);
        assert_eq!(super::scale_u8(255, 128), 128);
        assert_eq!(super::scale_u8(128, 255), 128);
        assert_eq!(super::scale_u8(128, 128), 64);
    }
}
