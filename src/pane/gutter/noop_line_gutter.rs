use crate::config::Config;
use crate::pane::Frame;
use crate::pane::Gutter;
use crate::theme::Theme;
use crate::tui::rect::Rect;

#[derive(Debug)]
pub struct NoopLineDrawer<'a> {
    _config: &'a Config,
    _theme: &'a Theme,
    _area: Rect,
}

impl<'a> NoopLineDrawer<'a> {
    pub fn new(_config: &'a Config, _theme: &'a Theme, _area: Rect) -> Self {
        Self {
            _config,
            _theme,
            _area,
        }
    }
}

impl Gutter for NoopLineDrawer<'_> {
    fn draw(&self, _: &mut Frame, _: usize, _: usize, _: usize) {}
    fn width(&self) -> u16 {
        0
    }
}
