use super::Drawable;
use crate::render::{Color, Rect};
use tiny_skia::{PixmapMut, Transform};

pub struct Container {
    rect: Rect,
    background: Color,
}

impl Container {
    pub fn new(rect: Rect, color: Color) -> Self {
        Self {
            rect,
            background: color,
        }
    }
}

impl Drawable for Container {
    fn render(&self, pixmap: &mut PixmapMut) {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color(self.background);
        pixmap.fill_rect(
            self.rect.clone().into(),
            &paint,
            Transform::identity(),
            None,
        );
    }
}
