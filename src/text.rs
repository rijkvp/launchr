use cosmic_text::{Attrs, Family, FontSystem, Metrics, Shaping, SwashCache};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tiny_skia::{Paint, PixmapMut, Rect, Transform};

pub static FONT_SYSTEM: Lazy<Mutex<FontSystem>> = Lazy::new(|| Mutex::new(FontSystem::new()));
pub static SWASH_CACHE: Lazy<Mutex<SwashCache>> = Lazy::new(|| Mutex::new(SwashCache::new()));

pub struct Text {
    buffer: cosmic_text::Buffer,
    attrs: cosmic_text::Attrs<'static>,
}

impl Text {
    pub fn new() -> Self {
        let buffer = cosmic_text::Buffer::new_empty(Metrics::new(32.0, 32.0));
        let attrs = Attrs::new().family(Family::Monospace);
        Self { buffer, attrs }
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut, width: u32, height: u32) {
        let mut paint = Paint::default();
        let transform = Transform::identity();
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        self.buffer
            .set_size(&mut font_system, width as f32, height as f32);
        // Note: for performance, use SwashCache directly
        // https://github.com/notgull/piet-tiny-skia/blob/main/src/lib.rs#L382
        // https://github.com/notgull/piet-tiny-skia/blob/main/src/lib.rs#L641
        self.buffer.draw(
            &mut font_system,
            &mut swash_cache,
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            |x, y, w, h, color| {
                paint.set_color_rgba8(color.r(), color.g(), color.b(), color.a());
                pixmap.fill_rect(
                    Rect::from_xywh(x as f32, y as f32, w as f32, h as f32).unwrap(),
                    &paint,
                    transform,
                    None,
                );
            },
        );
    }

    pub fn set_text(&mut self, text: &str) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.buffer
            .set_text(&mut font_system, text, self.attrs, Shaping::Basic);
    }
}
