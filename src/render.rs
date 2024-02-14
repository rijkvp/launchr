use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use tiny_skia::{PixmapMut, PremultipliedColorU8};
use winit::window::Window;

use crate::component::{Component, Drawable};

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
pub fn fill_rect(pixmap: &mut PixmapMut, rect: Rect, color: cosmic_text::Color) {
    let (x, y, w, h) = (
        rect.x as usize,
        rect.y as usize,
        rect.width as usize,
        rect.height as usize,
    );
    let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
    let max_x = x.saturating_add(w).min(width); // Prevent overflow & clamp to width
    let max_y = y.saturating_add(h).min(height);
    if max_x <= x || max_y <= y {
        // Don't render if the rect is out of bounds
        return;
    }

    println!("color: {:?}", color.as_rgba());
    let (r, g, b, a) = color.as_rgba_tuple();
    // if a == 0 {
    //     // Don't render transparent pixels
    //     return;
    // }
    let color = Color::from_rgba8(r, g, b, a);
    println!(
        "color2: {}, {}, {}, {}",
        color.red(),
        color.green(),
        color.blue(),
        color.alpha()
    );
    let pixels = pixmap.pixels_mut();
    for j in y..max_y {
        let row_start = j * width;
        for i in x..max_x {
            let c1 = pixels[row_start + i].demultiply();
            let a1 = c1.alpha();
            let a2 = color.alpha();
            let a = a1 + (1.0 - a1) * a2;
            let r = (c1.red() * a1 + color.red() * a2 * (255 - a1)) / a;
            let g = (c1.green() * a1 + color.green() * a2 * (255 - a1)) / a;
            let b = (c1.blue() * a1 + color.blue() * a2 * (255 - a1)) / a;
            pixels[row_start + i] = PremultipliedColorU8::from_rgba(r, g, b, a).unwrap();
        }
    }
}
