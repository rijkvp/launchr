use super::DrawHandleImpl;
use crate::ui::{Color, Rect, scale_u8};

fn fill_rect(buf: &mut [u8], buf_width: u32, buf_height: u32, rect: Rect, color: Color) {
    if color.alpha() == 0 {
        return;
    }
    let Rect { pos, size } = rect;
    let (x, y) = (pos.x.min(buf_width), pos.y.min(buf_height));
    let (w, h) = (size.x.min(buf_width - x), size.y.min(buf_height - y));

    if color.alpha() == 255 {
        // fast overwrite method if color is opaque
        let [r, g, b, a] = color.to_array();
        let pixel = [b, g, r, a]; // BGRA format

        if w == 1 {
            // if width is 1, fill pixel-by-pixel
            for y in y..y + h {
                let idx = (y * buf_width + x) as usize * 4;
                buf[idx..idx + 4].copy_from_slice(&pixel);
            }
        } else {
            // fill first row
            let first_row_start = (y * buf_width + x) as usize * 4;
            for i in 0..w as usize {
                let idx = first_row_start + i * 4;
                buf[idx..idx + 4].copy_from_slice(&pixel);
            }

            // copy first row to subsequent rows
            let row_bytes = (w * 4) as usize;
            for row in 1..h {
                let src_start = first_row_start;
                let dst_start = ((y + row) * buf_width + x) as usize * 4;
                buf.copy_within(src_start..src_start + row_bytes, dst_start);
            }
        }
    } else {
        for y in y..y + h {
            for x in x..x + w {
                let idx = (y * buf_width + x) as usize * 4;
                blend_color(&mut buf[idx..], color);
            }
        }
    }
}

fn fill_texture(
    buf: &mut [u8],
    buf_width: u32,
    buf_height: u32,
    x: u32,
    y: u32,
    texture: BorrowedBuffer,
) {
    // the position to start filling the texture
    let (start_x, start_y) = (x.min(buf_width - 1), y.min(buf_height - 1));
    let (w, h) = (
        texture.width.min(buf_width - start_x),
        texture.height.min(buf_height - start_y),
    ); // the width/height of the area to fill

    let is_opaque = texture.buffer.chunks_exact(4).all(|pixel| pixel[3] == 255);

    if is_opaque {
        // fast overwrite method if texture is fully opaque
        for y in 0..h {
            let buf_row_start = ((y + start_y) * buf_width + start_x) as usize * 4;
            let texture_row_start = (y * texture.width) as usize * 4;

            for x in 0..w {
                let buf_idx = buf_row_start + (x as usize * 4);
                let texture_idx = texture_row_start + (x as usize * 4);
                buf[buf_idx] = texture.buffer[texture_idx + 2]; // B
                buf[buf_idx + 1] = texture.buffer[texture_idx + 1]; // G
                buf[buf_idx + 2] = texture.buffer[texture_idx]; // R
                buf[buf_idx + 3] = texture.buffer[texture_idx + 3]; // A
            }
        }
    } else {
        // alpha blend pixel-by-pixel
        for y in 0..h {
            let buf_row_start = ((y + start_y) * buf_width + start_x) as usize * 4;
            let texture_row_start = (y * texture.width) as usize * 4;

            for x in 0..w {
                let buf_idx = buf_row_start + (x as usize * 4);
                let texture_idx = texture_row_start + (x as usize * 4);
                blend_bufs(&mut buf[buf_idx..], &texture.buffer[texture_idx..]);
            }
        }
    }
}

// buf: BGRA
// color: RGBA
#[inline]
fn blend_color(buf: &mut [u8], color: Color) {
    let a = 255 - color.alpha();
    buf[0] = color.blue() + scale_u8(buf[2], a);
    buf[1] = color.green() + scale_u8(buf[1], a);
    buf[2] = color.red() + scale_u8(buf[0], a);
    buf[3] = color.alpha() + scale_u8(buf[3], a);
}

// buf1: BGRA
// buf2: RGBA
#[inline]
fn blend_bufs(buf1: &mut [u8], buf2: &[u8]) {
    let a = 255 - buf2[3];
    buf1[0] = buf2[2] + scale_u8(buf1[0], a);
    buf1[1] = buf2[1] + scale_u8(buf1[1], a);
    buf1[2] = buf2[0] + scale_u8(buf1[2], a);
    buf1[3] = buf2[3] + scale_u8(buf1[3], a);
}

pub struct RenderBuffer<'a> {
    pub buffer: &'a mut [u8],
    pub width: u32,
    pub height: u32,
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
}

impl DrawHandleImpl for RenderBuffer<'_> {
    #[inline]
    fn draw_rect(&mut self, rect: Rect, color: Color) {
        fill_rect(self.buffer, self.width, self.height, rect, color);
    }

    #[inline]
    fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        fill_texture(self.buffer, self.width, self.height, x, y, texture);
    }

    fn get_bytes(&self) -> &[u8] {
        self.buffer
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

    pub fn bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl DrawHandleImpl for OnwedBuffer {
    #[inline]
    fn draw_rect(&mut self, rect: Rect, color: Color) {
        fill_rect(&mut self.buffer, self.width, self.height, rect, color);
    }

    #[inline]
    fn draw_texture(&mut self, x: u32, y: u32, texture: BorrowedBuffer) {
        fill_texture(&mut self.buffer, self.width, self.height, x, y, texture);
    }

    fn get_bytes(&self) -> &[u8] {
        &self.buffer
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
