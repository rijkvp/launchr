use super::{container, Color, Element, Length, Rect, UVec2, Widget};
use crate::render::{BorrowedBuffer, DrawHandle};
use cosmic_text::{
    Action, Attrs, CacheKeyFlags, Edit, Family, FontSystem, Metrics, Motion, Shaping, Stretch,
    Style, SwashCache, Weight,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, rc::Rc, sync::Mutex};
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
    metrics_opt: None,
};

const DEFAULT_FONT_SIZE: f32 = 18.0;

pub struct Text {
    buffer: cosmic_text::Buffer,
    texture_buf: Vec<u8>,
    width: u64,
    height: u64,
}

pub fn text(text: &str) -> Text {
    Text::new(DEFAULT_FONT_SIZE).with_text(text)
}

impl Text {
    pub fn new(font_size: f32) -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let buffer = cosmic_text::Buffer::new(&mut font_system, Metrics::new(font_size, font_size));
        Self {
            buffer,
            texture_buf: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.set_text(text);
        self
    }

    pub fn set_text(&mut self, text: &str) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.buffer
            .set_text(&mut font_system, text, DEFAULT_ATTRS, Shaping::Basic);

        let (width, height) = self.buffer.size();
        let (width, height) = (width.unwrap_or(0.0), height.unwrap_or(0.0));
        self.width = width.ceil() as u64;
        self.height = height.ceil() as u64;
        self.texture_buf = vec![0; (self.width * self.height * 4) as usize];
    }
}

impl Widget for Text {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.buffer.set_size(
            &mut FONT_SYSTEM.lock().unwrap(),
            Some(bounds.x as f32),
            Some(bounds.y as f32),
        );
        bounds
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();

        // Iterate all the glyphs
        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0., 0.), 1.0);

                // For now, draw the glyph with a white color
                // TODO: transparency
                let glyph_color = match glyph.color_opt {
                    Some(some) => some,
                    None => cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                };

                let Some(image) = swash_cache.get_image(&mut font_system, physical_glyph.cache_key)
                else {
                    continue;
                };

                let placement = image.placement;
                if let Some(data) = convert_image(image, Color::from(glyph_color)) {
                    let texture =
                        BorrowedBuffer::from_bytes(&data, placement.width, placement.height);

                    draw_handle.draw_texture(
                        (pos.x as i32 + physical_glyph.x + placement.left) as u32,
                        (pos.y as i32 + run.line_y as i32 + physical_glyph.y - placement.top)
                            as u32,
                        texture,
                    );
                }
            }
        }
    }
}

fn convert_image(image: &cosmic_text::SwashImage, color: Color) -> Option<Vec<u8>> {
    let glyph_size = image.placement.width as usize * image.placement.height as usize;

    if glyph_size == 0 {
        return None;
    }

    debug_assert_eq!(image.data.len(), glyph_size);
    let mut buffer = vec![0u8; glyph_size * 4];

    match image.content {
        cosmic_text::SwashContent::Mask => {
            for i in 0..glyph_size {
                let j = i << 2;
                let pixel_color = color.premultiply_with(image.data[i]);
                buffer[j] = pixel_color.red();
                buffer[j + 1] = pixel_color.green();
                buffer[j + 2] = pixel_color.blue();
                buffer[j + 3] = pixel_color.alpha();
            }
        }
        _ => panic!("Unsupported image content"),
    }

    Some(buffer)
}

pub fn text_box(text: &str, font_size: f32) -> Element {
    container(Text::new(font_size).with_text(text))
        .width(Length::Fill)
        .height(Length::Fixed(font_size as u32))
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
                Some(bounds.x as f32),
                Some(bounds.y as f32),
            );
        });
        bounds
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
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
                cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                |x, y, w, h, color| {
                    draw_handle.draw_rect(
                        Rect::new(pos.x + x.max(0) as u32, pos.y + y.max(0) as u32, w, h),
                        Color::from(color).premultiply(),
                    )
                },
            );
        }
    }
}
