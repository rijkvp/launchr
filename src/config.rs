use crate::ui::Color;
use anyhow::Result;
use serde::Deserialize;
use std::fs;

const CONFIG_DIR_NAME: &str = env!("CARGO_CRATE_NAME");

#[derive(Default, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub font: FontConfig,
    pub color: ColorConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = dirs::config_dir()
            .expect("no config dir")
            .join(CONFIG_DIR_NAME)
            .join("config.toml");
        if config_path.exists() {
            let text = fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&text)?)
        } else {
            log::info!("no config file found: using default config");
            Ok(Self::default())
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(default)]
pub struct FontConfig {
    pub normal_size: f32,
    pub large_size: f32,
    pub font_name: Option<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            normal_size: 18.0,
            large_size: 24.0,
            font_name: None,
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(default)]
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
