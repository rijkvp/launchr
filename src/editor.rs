use crate::text::FONT_SYSTEM;
use cosmic_text::{Action, Attrs, Buffer, Edit, Family, Metrics, SwashCache};
use tiny_skia::{Paint, PixmapMut, Rect, Transform};
use winit::keyboard::KeyCode;

pub struct Editor {
    editor: cosmic_text::Editor,
    attrs: cosmic_text::Attrs<'static>,
    swash_cache: cosmic_text::SwashCache,
}

impl Editor {
    pub fn new() -> Self {
        let mut editor = cosmic_text::Editor::new(Buffer::new_empty(Metrics::new(32.0, 32.0)));
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

    pub fn text(&self) -> &str {
        self.editor.buffer().lines[0].text()
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

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Backspace => self.perform_action(Action::Backspace),
            KeyCode::ArrowLeft => self.perform_action(Action::Left),
            KeyCode::ArrowRight => self.perform_action(Action::Right),
            KeyCode::ArrowUp => self.perform_action(Action::Up),
            KeyCode::ArrowDown => self.perform_action(Action::Down),
            _ => return false,
        }
        true
    }

    pub fn perform_action(&mut self, action: cosmic_text::Action) {
        println!("Action: {:?}", action);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.editor.action(&mut font_system, action);
        self.editor.shape_as_needed(&mut font_system);
    }
}
