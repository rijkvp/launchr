use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use tiny_skia::{PixmapMut, Transform};
use winit::window::Window;

use crate::component::Component;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}

impl Rect {
    pub fn new(x: u64, y: u64, width: u64, height: u64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl Into<tiny_skia::Rect> for Rect {
    fn into(self) -> tiny_skia::Rect {
        tiny_skia::Rect::from_xywh(
            self.x as f32,
            self.y as f32,
            self.width as f32,
            self.height as f32,
        )
        .unwrap()
    }
}

pub type Color = tiny_skia::Color;

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

    pub fn draw(&mut self, root: &impl Component) {
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

        let window_bounds = Rect::new(0, 0, width as u64, height as u64);
        root.render(window_bounds, &mut pixmap);

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
