use std::{cell::RefCell, rc::Rc};

use crate::render::{Color, Rect};
use tiny_skia::{PixmapMut, Transform};

use super::Component;

pub struct Container {
    child: Rc<RefCell<dyn Component>>,
    bg_color: Option<Color>,
    padding: Option<u64>,
}

impl Container {
    pub fn new(child: impl Component + 'static) -> Self {
        Self {
            child: Rc::new(RefCell::new(child)),
            bg_color: None,
            padding: None,
        }
    }

    pub fn from(child: Rc<RefCell<dyn Component>>) -> Self {
        Self {
            child,
            bg_color: None,
            padding: None,
        }
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn with_padding(mut self, padding: u64) -> Self {
        self.padding = Some(padding);
        self
    }
}

impl Component for Container {
    fn layout(&mut self, width: u64, height: u64) {
        let padding = self.padding.unwrap_or(0);
        let mut child = self.child.borrow_mut();
        child.layout(width - 2 * padding, height - 2 * padding);
    }

    fn render(&self, bounds: Rect, pixmap: &mut PixmapMut) {
        let mut paint = tiny_skia::Paint::default();

        if let Some(bg_color) = self.bg_color {
            paint.set_color(bg_color);
            pixmap.fill_rect(bounds.into(), &paint, Transform::identity(), None);
        }
        let padding = self.padding.unwrap_or(0);
        let child_bounds = Rect::new(
            bounds.x + padding,
            bounds.y + padding,
            bounds.width - 2 * padding,
            bounds.height - 2 * padding,
        );
        self.child.borrow().render(child_bounds, pixmap);
    }
}
