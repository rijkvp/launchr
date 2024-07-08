mod cpu;
mod texture;

pub use cpu::CpuRenderer;
pub use texture::Texture;

use crate::ui::{Color, Element, Rect};

pub trait Renderer {
    fn render(&mut self, root: &Element);
}

pub trait DrawHandle {
    fn draw_rect(&mut self, rect: Rect, color: Color);
}
