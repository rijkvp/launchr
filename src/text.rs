use cosmic_text::{FontSystem, Metrics, SwashCache, Attrs, Family, Shaping};
use tiny_skia::{PixmapMut, Paint, Transform, Rect};

pub struct Text {
    text: String,
    buffer: cosmic_text::Buffer,
    attrs: cosmic_text::Attrs<'static>,
    font_system: cosmic_text::FontSystem,
    swash_cache: cosmic_text::SwashCache,
}

impl Text {
    pub fn new() -> Self {
        let font_system = FontSystem::new();
        let buffer = cosmic_text::Buffer::new_empty(Metrics::new(64.0, 74.0));
        let swash_cache = SwashCache::new();
        let attrs = Attrs::new().family(Family::Monospace);
        Self {
            text: String::new(),
            font_system,
            buffer,
            attrs,
            swash_cache,
        }
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut, width: u32, height: u32) {
        let mut paint = Paint::default();
        let transform = Transform::identity();
        self.buffer
            .set_size(&mut self.font_system, width as f32, height as f32);
        self.buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
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

    pub fn add_text(&mut self, text: &str) {
        self.text.push_str(text);
        println!("{}", self.text);
        self.buffer.set_text(
            &mut self.font_system,
            &self.text,
            self.attrs,
            Shaping::Advanced,
        );
    }
}
