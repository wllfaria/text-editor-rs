use std::collections::HashMap;

use crossterm::style::Color;

pub mod loader;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub appearance: Style,
    pub statusline: StatuslineTheming,
    pub float: Style,
    pub gutter: Style,
    pub tokens: HashMap<String, Style>,
}

#[derive(Debug, Clone, Copy)]
pub struct StatuslineTheming {
    pub file_name: Style,
    pub mode: Style,
    pub cursor: Style,
    pub appearance: Style,
}

#[derive(Debug)]
pub struct Gutter {
    pub bg: Color,
    pub fg: Color,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub italic: Option<bool>,
    pub bold: Option<bool>,
}

impl Theme {
    pub fn dark() -> anyhow::Result<Self> {
        loader::ThemeLoader::default_dark()
    }

    pub fn light() -> anyhow::Result<Self> {
        loader::ThemeLoader::default_light()
    }
}
