mod buffer;
mod cpu;

pub use buffer::*;
pub use cpu::CpuRenderer;

use crate::ui::{Color, DynWidget, Rect};

pub trait Renderer {
    fn render(&mut self, root: &DynWidget);
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

    pub fn get_bytes(&self) -> &[u8] {
        self.inner.get_bytes()
    }
}

pub trait DrawHandleImpl {
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer);
    fn get_bytes(&self) -> &[u8];
}
