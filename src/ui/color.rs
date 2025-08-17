use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy)]
pub struct Color([u8; 4]);

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Remove the # prefix if present
        let hex = s.strip_prefix('#').unwrap_or(&s);

        match hex.len() {
            6 => {
                // RGB format (#rrggbb)
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid red component"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid green component"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid blue component"))?;
                Ok(Color::from_rgb(r, g, b))
            }
            8 => {
                // RGBA format (#rrggbbaa)
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid red component"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid green component"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid blue component"))?;
                let a = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| serde::de::Error::custom("Invalid alpha component"))?;
                Ok(Color::from_rgba(r, g, b, a))
            }
            _ => Err(serde::de::Error::custom(
                "Color hex string must be 6 (RGB) or 8 (RGBA) characters long",
            )),
        }
    }
}

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

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self([red, green, blue, 255])
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

/// Scales a color component by an alpha value
/// Optimized to use no floating point operations
#[inline]
pub fn scale_u8(c: u8, a: u8) -> u8 {
    let p = u32::from(c) * u32::from(a) + 128;
    ((p + (p >> 8)) >> 8) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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

    #[test]
    fn test_deserialize_rgb_hex() {
        let color: Color = serde_json::from_str("\"#192330\"").unwrap();
        assert_eq!(color.red(), 25);
        assert_eq!(color.green(), 35);
        assert_eq!(color.blue(), 48);
        assert_eq!(color.alpha(), 255);
    }

    #[test]
    fn test_deserialize_rgba_hex() {
        let color: Color = serde_json::from_str("\"#00ff00ff\"").unwrap();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 0);
        assert_eq!(color.alpha(), 255);
    }

    #[test]
    fn test_deserialize_rgba_hex_with_alpha() {
        let color: Color = serde_json::from_str("\"#0000ff80\"").unwrap();
        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 255);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    fn test_deserialize_without_hash() {
        let color: Color = serde_json::from_str("\"ffffff\"").unwrap();
        assert_eq!(color.red(), 255);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 255);
        assert_eq!(color.alpha(), 255);
    }

    #[test]
    fn test_deserialize_invalid_length() {
        let result: Result<Color, _> = serde_json::from_str("\"#fff\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_hex() {
        let result: Result<Color, _> = serde_json::from_str("\"#gggggg\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_toml() {
        #[derive(serde::Deserialize)]
        struct Config {
            color: Color,
        }

        let toml_str = "color = \"#ff8000\"";
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.color.red(), 255);
        assert_eq!(config.color.green(), 128);
        assert_eq!(config.color.blue(), 0);
        assert_eq!(config.color.alpha(), 255);
    }
}
