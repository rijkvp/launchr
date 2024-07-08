use crate::ui::{Color, Rect};

use super::DrawHandleImpl;

pub struct RenderBuffer<'a> {
    buffer: &'a mut [u8],
    width: u32,
    height: u32,
}

// https://en.wikipedia.org/wiki/Alpha_compositing
#[inline]
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
        debug_assert_eq!(buffer.len(), (width * height * 4) as usize);
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    #[inline]
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        if color.alpha == 0 {
            return;
        }
        let Rect { pos, size } = rect;
        let (x, y) = (pos.x.min(self.width), pos.y.min(self.height));
        let (w, h) = (size.x.min(self.width - x), size.y.min(self.height - y));
        for y in y..y + h {
            for x in x..x + w {
                let idx = (y * self.width + x) as usize * 4;
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

    // TODO: avoid duplication
    fn fill_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        // The position to start filling the texture
        let (start_x, start_y) = (x.min(self.width - 1), y.min(self.height - 1));
        let (w, h) = (
            texture.width.min(self.width - start_x),
            texture.height.min(self.height - start_y),
        ); // The width/height of the area to fill
        for y in 0..h {
            for x in 0..w {
                let (self_x, self_y) = (x + start_x, y + start_y);
                let idx = (self_y * self.width + self_x) as usize * 4;
                let other_idx = (y * texture.width + x) as usize * 4;
                let blended_color = blend_color(
                    Color {
                        red: texture.buffer[other_idx],
                        green: texture.buffer[other_idx + 1],
                        blue: texture.buffer[other_idx + 2],
                        alpha: texture.buffer[other_idx + 3],
                    },
                    Color {
                        red: self.buffer[idx],
                        green: self.buffer[idx + 1],
                        blue: self.buffer[idx + 2],
                        alpha: self.buffer[idx + 3],
                    },
                );
                self.buffer[idx] = blended_color.red;
                self.buffer[idx + 1] = blended_color.green;
                self.buffer[idx + 2] = blended_color.blue;
                self.buffer[idx + 3] = blended_color.alpha;
            }
        }
    }
}

impl DrawHandleImpl for RenderBuffer<'_> {
    fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.fill_rect(rect, color);
    }

    fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        self.fill_texture(x, y, texture);
    }
}

pub struct OnwedBuffer {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
}

impl OnwedBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: vec![0; (width * height * 4) as usize],
            width,
            height,
        }
    }

    // TODO: avoid duplication
    #[inline]
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        if color.alpha == 0 {
            return;
        }
        let Rect { pos, size } = rect;
        let (x, y) = (pos.x.min(self.width), pos.y.min(self.height));
        let (w, h) = (size.x.min(self.width - x), size.y.min(self.height - y));
        for y in y..y + h {
            for x in x..x + w {
                let idx = (y * self.width + x) as usize * 4;
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

    fn fill_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        // The position to start filling the texture
        let (start_x, start_y) = (x.min(self.width - 1), y.min(self.height - 1));
        let (w, h) = (
            texture.width.min(self.width - start_x),
            texture.height.min(self.height - start_y),
        ); // The width/height of the area to fill
        for y in 0..h {
            for x in 0..w {
                let (self_x, self_y) = (x + start_x, y + start_y);
                let idx = (self_y * self.width + self_x) as usize * 4;
                let other_idx = (y * texture.width + x) as usize * 4;
                let blended_color = blend_color(
                    Color {
                        red: texture.buffer[other_idx],
                        green: texture.buffer[other_idx + 1],
                        blue: texture.buffer[other_idx + 2],
                        alpha: texture.buffer[other_idx + 3],
                    },
                    Color {
                        red: self.buffer[idx],
                        green: self.buffer[idx + 1],
                        blue: self.buffer[idx + 2],
                        alpha: self.buffer[idx + 3],
                    },
                );
                self.buffer[idx] = blended_color.red;
                self.buffer[idx + 1] = blended_color.green;
                self.buffer[idx + 2] = blended_color.blue;
                self.buffer[idx + 3] = blended_color.alpha;
            }
        }
    }
}

impl DrawHandleImpl for OnwedBuffer {
    fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.fill_rect(rect, color);
    }

    fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        self.fill_texture(x, y, texture);
    }
}

pub struct BorrowedBuffer<'a> {
    buffer: &'a [u8],
    width: u32,
    height: u32,
}

impl<'a> BorrowedBuffer<'a> {
    pub fn from_bytes(buffer: &'a [u8], width: u32, height: u32) -> Self {
        debug_assert_eq!(buffer.len(), (width * height * 4) as usize);
        Self {
            buffer,
            width,
            height,
        }
    }
}
