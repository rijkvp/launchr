use crate::app::App;
use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use tiny_skia::{Color, PixmapMut};
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

    pub fn draw(&mut self, app: &mut App) {
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
        pixmap.fill(Color::from_rgba8(0, 0, 0, 0xFF));

        app.editor.render(&mut pixmap);
        app.text.render(&mut pixmap);

        surface_buffer.present().unwrap();
    }
}
