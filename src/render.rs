use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use tiny_skia::PixmapMut;
use winit::window::Window;

use crate::component::{Drawable, Component};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
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

    pub fn draw<'a>(&mut self, drawables: impl Iterator<Item = Component<'a>>) {
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

        for drawable in drawables {
            drawable.render(&mut pixmap);
        }

        surface_buffer.present().unwrap();
    }
}

// TODO: Improve performance
pub fn fill_rect(
    pixmap: &mut PixmapMut,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: cosmic_text::Color,
) {
    let (x, y, w, h) = (x as usize, y as usize, w as usize, h as usize);
    let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
    let max_x = x.saturating_add(w).min(width); // Prevent overflow & clamp to width
    let max_y = y.saturating_add(h).min(height);
    if max_x <= x || max_y <= y {
        // Don't render if the rect is out of bounds
        return;
    }
    let (r, g, b, a) = (color.r(), color.g(), color.b(), color.a());
    if a == 0 {
        // Don't render transparent pixels
        return;
    }
    let color = Color::from_rgba8(r, g, b, a).premultiply().to_color_u8();
    let pixels = pixmap.pixels_mut();
    for j in y..max_y {
        let row_start = j * width;
        for i in x..max_x {
            pixels[row_start + i] = color;
        }
    }
}
