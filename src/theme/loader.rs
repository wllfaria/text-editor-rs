use crate::{
    config::Config,
    theme::{Gutter, StatuslineTheming, Style, Theme},
};

use crossterm::style::Color;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize, Debug, Clone)]
struct TokenStyle {
    fg: Option<String>,
    bg: Option<String>,
    italic: Option<bool>,
    bold: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct StatuslineStyle {
    file_name: TokenStyle,
    mode: TokenStyle,
    cursor: TokenStyle,
    appearance: TokenStyle,
}

#[derive(Deserialize, Debug)]
struct GutterStyle {
    bg: String,
    fg: String,
}

#[derive(Deserialize, Debug)]
pub struct ThemeLoader {
    name: String,
    appearance: TokenStyle,
    statusline: StatuslineStyle,
    float: TokenStyle,
    gutter: TokenStyle,
    tokens: HashMap<String, TokenStyle>,
}

impl ThemeLoader {
    pub fn default_dark() -> anyhow::Result<Theme> {
        let theme_path = Path::new("./config/themes").join("glyph-dark-default.toml");
        let toml = std::fs::read_to_string(theme_path)?;
        let theme: ThemeLoader = toml::from_str(&toml).unwrap();
        Ok(theme.into())
    }

    pub fn default_light() -> anyhow::Result<Theme> {
        let theme_path = Config::themes_path().join("glyph-light-default.toml");
        let toml = std::fs::read_to_string(theme_path)?;
        let theme: ThemeLoader = toml::from_str(&toml).unwrap();
        Ok(theme.into())
    }
}

pub fn hex_to_rgb(hex: Option<String>) -> Result<Option<Color>, &'static str> {
    match hex {
        Some(hex) => {
            if hex.starts_with('#') && hex.len() == 7 {
                let r = u8::from_str_radix(&hex[1..3], 16).expect("Invalid red component");
                let g = u8::from_str_radix(&hex[3..5], 16).expect("Invalid green component");
                let b = u8::from_str_radix(&hex[5..7], 16).expect("Invalid blue component");

                Ok(Some(Color::Rgb { r, g, b }))
            } else {
                Err("Invalid hex color format")
            }
        }
        None => Ok(None),
    }
}

impl From<ThemeLoader> for Theme {
    fn from(val: ThemeLoader) -> Self {
        let tokens = val.tokens.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k.clone(), (*v).clone().into());
            acc
        });
        Theme {
            name: val.name,
            statusline: val.statusline.into(),
            gutter: val.gutter.into(),
            float: val.float.into(),
            tokens,
            appearance: val.appearance.into(),
        }
    }
}

impl From<StatuslineStyle> for StatuslineTheming {
    fn from(val: StatuslineStyle) -> Self {
        StatuslineTheming {
            file_name: val.file_name.into(),
            mode: val.mode.into(),
            cursor: val.cursor.into(),
            appearance: val.appearance.into(),
        }
    }
}

impl From<GutterStyle> for Gutter {
    fn from(val: GutterStyle) -> Self {
        Gutter {
            bg: hex_to_rgb(Some(val.bg)).unwrap().unwrap(),
            fg: hex_to_rgb(Some(val.fg)).unwrap().unwrap(),
        }
    }
}

impl From<TokenStyle> for Style {
    fn from(val: TokenStyle) -> Self {
        Style {
            fg: hex_to_rgb(val.fg).unwrap(),
            bg: hex_to_rgb(val.bg).unwrap(),
            bold: val.bold,
            italic: val.italic,
        }
    }
}
