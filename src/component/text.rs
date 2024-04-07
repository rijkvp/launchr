use crate::render::{fill_rect, Rect};
use cosmic_text::{
    Action, Attrs, CacheKeyFlags, Edit, Family, FontSystem, Metrics, Motion, Shaping, Stretch,
    Style, SwashCache, Weight,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tiny_skia::PixmapMut;
use winit::keyboard::KeyCode;

use super::Component;

static FONT_SYSTEM: Lazy<Mutex<FontSystem>> = Lazy::new(|| Mutex::new(FontSystem::new()));
static SWASH_CACHE: Lazy<Mutex<SwashCache>> = Lazy::new(|| Mutex::new(SwashCache::new()));

const DEFAULT_ATTRS: Attrs = Attrs {
    color_opt: None,
    family: Family::Name("Iosevka Nerd Font"),
    stretch: Stretch::Normal,
    style: Style::Normal,
    weight: Weight::NORMAL,
    metadata: 0,
    cache_key_flags: CacheKeyFlags::empty(),
};

pub struct Text {
    buffer: cosmic_text::Buffer,
}

impl Text {
    pub fn new(font_size: f32) -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut buffer =
            cosmic_text::Buffer::new(&mut font_system, Metrics::new(font_size, font_size));
        // TODO: dynamic
        buffer.set_size(&mut font_system, 200.0, font_size);
        Self { buffer }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.set_text(text);
        self
    }

    pub fn set_text(&mut self, text: &str) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.buffer
            .set_text(&mut font_system, text, DEFAULT_ATTRS, Shaping::Basic);
    }
}

impl Component for Text {
    fn layout(&mut self, width: u64, height: u64) {
        self.buffer.set_size(
            &mut FONT_SYSTEM.lock().unwrap(),
            width as f32,
            height as f32,
        );
    }

    fn render(&self, bounds: Rect, pixmap: &mut PixmapMut) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        self.buffer.draw(
            &mut font_system,
            &mut swash_cache,
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            |x, y, w, h, color| {
                fill_rect(
                    pixmap,
                    Rect::new(
                        bounds.x + x.max(0) as u64, // use max(0) to prevent underflow
                        bounds.y + y.max(0) as u64,
                        w as u64,
                        h as u64,
                    ),
                    color,
                )
            },
        );
    }
}

pub struct TextEditor {
    editor: cosmic_text::Editor<'static>,
}

impl TextEditor {
    pub fn new(font_size: f32) -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut buffer =
            cosmic_text::Buffer::new(&mut font_system, Metrics::new(font_size, font_size));
        // TODO: dynamic
        buffer.set_size(&mut font_system, 200.0, font_size);
        let mut editor = cosmic_text::Editor::new(buffer);
        editor.with_buffer_mut(|buf| {
            // Intial text must be set
            buf.set_text(&mut font_system, "", DEFAULT_ATTRS, Shaping::Basic);
        });
        Self { editor }
    }

    pub fn text(&self) -> String {
        self.editor
            .with_buffer(|buf| buf.lines[0].text().to_string())
    }

    pub fn handle_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Backspace => self.perform_action(Action::Backspace),
            KeyCode::ArrowLeft => self.perform_action(Action::Motion(Motion::Left)),
            KeyCode::ArrowRight => self.perform_action(Action::Motion(Motion::Right)),
            KeyCode::ArrowUp => self.perform_action(Action::Motion(Motion::Up)),
            KeyCode::ArrowDown => self.perform_action(Action::Motion(Motion::Down)),
            _ => return false,
        }
        true
    }

    pub fn perform_action(&mut self, action: cosmic_text::Action) {
        log::info!("Action: {:?}", action);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.editor.action(&mut font_system, action);
        self.editor.shape_as_needed(&mut font_system, false);
    }
}

impl Component for TextEditor {
    fn layout(&mut self, width: u64, height: u64) {
        self.editor.with_buffer_mut(|buf| {
            buf.set_size(
                &mut FONT_SYSTEM.lock().unwrap(),
                width as f32,
                height as f32,
            );
        });
    }

    fn render(&self, bounds: Rect, pixmap: &mut PixmapMut) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        if self.editor.redraw() {
            self.editor.draw(
                &mut font_system,
                &mut swash_cache,
                cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                cosmic_text::Color::rgb(0xAA, 0xAA, 0xFF),
                |x, y, w, h, color| {
                    fill_rect(
                        pixmap,
                        Rect::new(
                            bounds.x + x.max(0) as u64,
                            bounds.y + y.max(0) as u64,
                            w as u64,
                            h as u64,
                        ),
                        color,
                    )
                },
            );
        }
    }
}
