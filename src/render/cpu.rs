use super::{texture::Texture, DrawHandle, Renderer};
use crate::ui::{Color, Element, Rect, UVec2, Widget};
use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use winit::window::Window;

pub struct CpuRenderer {
    window: Arc<Window>,
    #[allow(dead_code)] // TODO: Remove if not needed
    context: Context<Arc<Window>>,
    surface: Surface<Arc<Window>, Arc<Window>>,
}

impl CpuRenderer {
    pub fn new(window: Arc<Window>) -> Self {
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        Self {
            window,
            context,
            surface,
        }
    }
}

impl Renderer for CpuRenderer {
    fn render(&mut self, root: &Element) {
        let (width, height) = {
            let size = self.window.inner_size();
            (size.width, size.height)
        };
        self.surface
            .resize(
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            )
            .unwrap();
        let mut surface_buffer = self.surface.buffer_mut().unwrap();
        let surface_buffer_u8 = unsafe {
            std::slice::from_raw_parts_mut(
                surface_buffer.as_mut_ptr() as *mut u8,
                surface_buffer.len() * 4,
            )
        };
        let mut texture = Texture::from_bytes(surface_buffer_u8, width as u64, height as u64);
        texture.clear();
        let mut render_buffer: Box<dyn DrawHandle> = Box::new(texture);

        root.render(UVec2::zero(), &mut render_buffer);

        self.window.pre_present_notify();
        surface_buffer.present().unwrap();
    }
}

impl DrawHandle for Texture<'_> {
    fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.fill_rect(rect, color);
    }
}
