use super::{container, Element, Length, Rect, UVec2, Widget};
use crate::render::fill_rect;

use cosmic_text::{
    Action, Attrs, CacheKeyFlags, Edit, Family, FontSystem, Metrics, Motion, Shaping, Stretch,
    Style, SwashCache, Weight,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, rc::Rc, sync::Mutex};
use tiny_skia::PixmapMut;
use winit::keyboard::KeyCode;

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

const DEFAULT_FONT_SIZE: f32 = 18.0;

pub struct Text {
    buffer: cosmic_text::Buffer,
}

pub fn text(text: &str) -> Text {
    Text::new(DEFAULT_FONT_SIZE).with_text(text)
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

impl Widget for Text {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.buffer.set_size(
            &mut FONT_SYSTEM.lock().unwrap(),
            bounds.x as f32,
            bounds.y as f32,
        );
        bounds
    }

    fn render(&self, pos: UVec2, pixmap: &mut PixmapMut) {
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
                        pos.x + x.max(0) as u64, // use max(0) to prevent underflow
                        pos.y + y.max(0) as u64,
                        w as u64,
                        h as u64,
                    ),
                    color,
                )
            },
        );
    }
}

pub fn text_box(text: &str, font_size: f32) -> Element {
    container(Text::new(font_size).with_text(text))
        .width(Length::Fill)
        .height(Length::Fixed(font_size as u64))
        .padding(4) // TODO: Configurable text box builder
        .into_element()
}

#[derive(Clone)]
pub struct Editor {
    inner: Rc<RefCell<cosmic_text::Editor<'static>>>,
}

impl Editor {
    pub fn new() -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let buffer = cosmic_text::Buffer::new(
            &mut font_system,
            Metrics::new(DEFAULT_FONT_SIZE, DEFAULT_FONT_SIZE),
        );
        let mut editor = cosmic_text::Editor::new(buffer);
        editor.with_buffer_mut(|buf| {
            buf.set_text(&mut font_system, "", DEFAULT_ATTRS, Shaping::Basic); // Intial text must be set
        });
        Self {
            inner: Rc::new(RefCell::new(editor)),
        }
    }

    pub fn text(&self) -> String {
        self.inner
            .borrow()
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
        log::debug!("edit action: {:?}", action);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut editor = self.inner.borrow_mut();
        editor.action(&mut font_system, action);
        editor.shape_as_needed(&mut font_system, false);
    }
}

pub struct TextEditor {
    editor: Editor,
}

impl TextEditor {
    pub fn new(editor: Editor, font_size: f32) -> Self {
        editor.inner.borrow_mut().with_buffer_mut(|buf| {
            buf.set_metrics(
                &mut FONT_SYSTEM.lock().unwrap(),
                Metrics::new(font_size, font_size),
            );
        });
        Self { editor }
    }
}

impl Widget for TextEditor {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.editor.inner.borrow_mut().with_buffer_mut(|buf| {
            buf.set_size(
                &mut FONT_SYSTEM.lock().unwrap(),
                bounds.x as f32,
                bounds.y as f32,
            );
        });
        bounds
    }

    fn render(&self, pos: UVec2, pixmap: &mut PixmapMut) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        let editor = self.editor.inner.borrow_mut();
        if editor.redraw() {
            editor.draw(
                &mut font_system,
                &mut swash_cache,
                cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                cosmic_text::Color::rgb(0xAA, 0xAA, 0xFF),
                |x, y, w, h, color| {
                    fill_rect(
                        pixmap,
                        Rect::new(
                            pos.x + x.max(0) as u64,
                            pos.y + y.max(0) as u64,
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
