mod buffer;
mod cpu;

pub use buffer::*;
pub use cpu::CpuRenderer;

use crate::ui::{Color, Element, Rect};

pub trait Renderer {
    fn render(&mut self, root: &Element);
}

pub struct DrawHandle {
    inner: Box<dyn DrawHandleImpl>,
}

impl DrawHandle {
    pub fn from(inner: impl DrawHandleImpl + 'static) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }

    #[inline]
    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.inner.draw_rect(rect, color);
    }

    #[inline]
    pub fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        self.inner.draw_texture(x, y, texture)
    }
}

pub trait DrawHandleImpl {
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer);
}
