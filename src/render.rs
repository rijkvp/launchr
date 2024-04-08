use crate::ui::{Color, Rect, UVec2, Widget};

use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use tiny_skia::{PixmapMut, Transform};
use winit::window::Window;

pub struct Renderer {
    window: Arc<Window>,
    #[allow(dead_code)] // TODO: Remove if not needed
    context: Context<Arc<Window>>,
    surface: Surface<Arc<Window>, Arc<Window>>,
}

impl Renderer {
    pub fn from_window(window: Arc<Window>) -> Self {
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        Self {
            window,
            context,
            surface,
        }
    }

    pub fn draw(&mut self, root: &impl Widget) {
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
        let mut pixmap = PixmapMut::from_bytes(surface_buffer_u8, width, height).unwrap();
        pixmap.fill(Color::from_rgba8(0, 0, 0, 0));

        root.render(UVec2::zero(), &mut pixmap);

        surface_buffer.present().unwrap();
    }
}

// TODO: Improve performance
pub fn fill_rect(pixmap: &mut PixmapMut, rect: Rect, color: cosmic_text::Color) {
    let mut paint = tiny_skia::Paint::default();
    let (r, g, b, a) = color.as_rgba_tuple();
    paint.set_color(Color::from_rgba8(r, g, b, a));
    pixmap.fill_rect(rect.into(), &paint, Transform::identity(), None);
}
