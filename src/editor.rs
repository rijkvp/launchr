use crate::text::FONT_SYSTEM;
use cosmic_text::{Attrs, Buffer, Edit, Family, Metrics, SwashCache};
use tiny_skia::{Paint, PixmapMut, Rect, Transform};

pub struct Editor {
    editor: cosmic_text::Editor,
    attrs: cosmic_text::Attrs<'static>,
    swash_cache: cosmic_text::SwashCache,
}

impl Editor {
    pub fn new() -> Self {
        let mut editor = cosmic_text::Editor::new(Buffer::new_empty(Metrics::new(64.0, 74.0)));
        let attrs = Attrs::new().family(Family::Monospace);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        editor
            .buffer_mut()
            .set_text(&mut font_system, "", attrs, cosmic_text::Shaping::Advanced);
        let swash_cache = SwashCache::new();
        Self {
            editor,
            attrs,
            swash_cache,
        }
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut, width: u32, height: u32) {
        let mut paint = Paint::default();
        let transform = Transform::identity();
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.editor
            .buffer_mut()
            .set_size(&mut font_system, width as f32, height as f32);
        self.editor.draw(
            &mut font_system,
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

    pub fn perform_action(&mut self, action: cosmic_text::Action) {
        println!("Action: {:?}", action);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.editor.action(&mut font_system, action);
        self.editor.shape_as_needed(&mut font_system);
    }
}
