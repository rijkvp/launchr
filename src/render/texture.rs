use crate::ui::{Color, Rect, UVec2};

pub struct Texture<'a> {
    buffer: &'a mut [u8],
    width: u64,
    height: u64,
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

impl<'a> Texture<'a> {
    pub fn from_bytes(buffer: &'a mut [u8], width: u64, height: u64) -> Self {
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

    pub fn overlay(&mut self, pos: UVec2, other: Texture) {
        let (x, y) = (pos.x.min(self.width), pos.y.min(self.height));
        let (w, h) = (
            other.width.min(self.width - x),
            other.height.min(self.height - y),
        );
        for y in y..y + h {
            for x in x..x + w {
                let idx = (y * self.width + x) as usize * 4;
                let other_idx = ((y - pos.y) * other.width + (x - pos.x)) as usize * 4;
                self.buffer[idx] = other.buffer[other_idx];
                self.buffer[idx + 1] = other.buffer[other_idx + 1];
                self.buffer[idx + 2] = other.buffer[other_idx + 2];
                self.buffer[idx + 3] = other.buffer[other_idx + 3];
            }
        }
    }
}
