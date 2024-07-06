use crate::ui::{Rect, UVec2, Widget};

use softbuffer::{Context, Surface};
use std::{num::NonZeroU32, sync::Arc};
use winit::window::Window;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<cosmic_text::Color> for Color {
    fn from(color: cosmic_text::Color) -> Self {
        Self {
            red: color.r(),
            green: color.g(),
            blue: color.b(),
            alpha: color.a(),
        }
    }
}

impl Color {
    pub fn from_rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn to_array(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

pub struct RenderBuffer<'a> {
    buffer: &'a mut [u8],
    width: u32,
    height: u32,
}

// https://en.wikipedia.org/wiki/Alpha_compositing
fn blend_color(foreground: Color, background: Color) -> Color {
    let Color {
        red: r_a,
        green: g_a,
        blue: b_a,
        alpha: a_a,
    } = foreground;
    let Color {
        red: r_b,
        green: g_b,
        blue: b_b,
        alpha: a_b,
    } = background;

    let a_a = a_a as f32 / 255.0;
    let a_b = a_b as f32 / 255.0;
    let a_c = a_b * (1.0 - a_a);
    let a_o = a_a + a_c;

    Color {
        red: ((r_a as f32 * a_a + r_b as f32 * a_c) / a_o) as u8,
        green: ((g_a as f32 * a_a + g_b as f32 * a_c) / a_o) as u8,
        blue: ((b_a as f32 * a_a + b_b as f32 * a_c) / a_o) as u8,
        alpha: 255,
    }
}

impl<'a> RenderBuffer<'a> {
    pub fn from_bytes(buffer: &'a mut [u8], width: u32, height: u32) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        if color.alpha == 0 {
            return;
        }
        let Rect { pos, size } = rect;
        let (width, height) = (self.width as u64, self.height as u64);
        let (x, y) = (pos.x.min(width), pos.y.min(height));
        let (w, h) = (size.x.min(width - x), size.y.min(height - y));
        for y in y..y + h {
            for x in x..x + w {
                let idx = (y * width + x) as usize * 4;
                let bg_color = Color {
                    red: self.buffer[idx],
                    green: self.buffer[idx + 1],
                    blue: self.buffer[idx + 2],
                    alpha: self.buffer[idx + 3],
                };
                let blended_color = blend_color(color, bg_color);
                self.buffer[idx] = blended_color.red;
                self.buffer[idx + 1] = blended_color.green;
                self.buffer[idx + 2] = blended_color.blue;
                self.buffer[idx + 3] = blended_color.alpha;
            }
        }
    }
}

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
        let mut render_buffer = RenderBuffer::from_bytes(surface_buffer_u8, width, height);
        render_buffer.clear();

        root.render(UVec2::zero(), &mut render_buffer);

        self.window.pre_present_notify();
        surface_buffer.present().unwrap();
    }
}
