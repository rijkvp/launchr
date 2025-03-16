use super::{Color, Rect, UVec2, Widget};
use crate::render::{BorrowedBuffer, DrawHandle};
use cosmic_text::{
    Action, Attrs, CacheKeyFlags, Edit, Family, FontSystem, Metrics, Motion, Shaping, Stretch,
    Style, SwashCache, Weight,
};
use once_cell::sync::Lazy;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};
use winit::keyboard::KeyCode;

static FONT_SYSTEM: Lazy<Mutex<FontSystem>> = Lazy::new(|| Mutex::new(FontSystem::new()));
static SWASH_CACHE: Lazy<Mutex<SwashCache>> = Lazy::new(|| Mutex::new(SwashCache::new()));
static GLYPH_CACHE: Lazy<Mutex<GlyphCache>> = Lazy::new(|| Mutex::new(GlyphCache::default()));

#[derive(Default)]
struct GlyphCache {
    cache: HashMap<cosmic_text::CacheKey, Option<CachcedGlyph>>,
}

struct CachcedGlyph {
    texture: Vec<u8>,
    placement: cosmic_text::Placement,
}

const DEFAULT_ATTRS: Attrs = Attrs {
    color_opt: None,
    family: Family::SansSerif,
    stretch: Stretch::Normal,
    style: Style::Normal,
    weight: Weight::NORMAL,
    metadata: 0,
    cache_key_flags: CacheKeyFlags::empty(),
    metrics_opt: None,
};

const DEFAULT_FONT_SIZE: f32 = 18.0;

pub struct TextBuilder {
    text: String,
    size: Option<f32>,
    line_height: Option<f32>,
    font_name: Option<String>,
    bold: bool,
}

impl TextBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            size: None,
            line_height: None,
            font_name: None,
            bold: false,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn line_height(mut self, line_height: f32) -> Self {
        self.line_height = Some(line_height);
        self
    }

    pub fn font(mut self, font_name: impl Into<String>) -> Self {
        self.font_name = Some(font_name.into());
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    pub fn build(self) -> Text {
        let size = self.size.unwrap_or(DEFAULT_FONT_SIZE);
        Text::new(
            &self.text,
            size,
            self.line_height.unwrap_or(size),
            self.bold,
            self.font_name,
        )
    }
}

pub struct Text {
    buffer: cosmic_text::Buffer,
    width: u32,
    height: u32,
}

impl Text {
    fn new(text: &str, size: f32, line_height: f32, bold: bool, font_name: Option<String>) -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();

        let mut attrs = DEFAULT_ATTRS;
        if let Some(font) = &font_name {
            attrs.family = Family::Name(font)
        }
        attrs.weight = if bold { Weight::BOLD } else { Weight::NORMAL };

        let mut buffer =
            cosmic_text::Buffer::new(&mut font_system, Metrics::new(size, line_height));
        // use advanced shaping to get all font features, like emojis and ligatures
        buffer.set_text(&mut font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut font_system, false);

        let (width, height) = buffer.size();
        let (width, height) = (
            width.unwrap_or(0.0).ceil() as u32,
            height.unwrap_or(0.0).ceil() as u32,
        );

        Self {
            buffer,
            width,
            height,
        }
    }
}

impl Widget for Text {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let (buf_width, buf_height) = (
            self.buffer.size().0.map(|f| f as u32),
            self.buffer.size().1.map(|f| f as u32),
        );
        // only relayout if the bounds have changed
        // the buffer is always resized to the bounds so we can determine the wrapped text size
        // afterwards
        if Some(bounds.x) != buf_width || Some(bounds.y) != buf_height {
            self.buffer.set_size(
                &mut FONT_SYSTEM.lock().unwrap(),
                Some(bounds.x as f32),
                Some(bounds.y as f32),
            );
            self.buffer
                .shape_until_scroll(&mut FONT_SYSTEM.lock().unwrap(), false);
            if self.buffer.layout_runs().count() == 0 {
                log::error!(
                    "no layout runs for text, bounds are: {}x{}",
                    bounds.x,
                    bounds.y
                );
                panic!("no layout runs for text");
            }
            // at the moment there is no build-in way to get the size the text will take in cosmic-text
            // so this computes it manually from the layout runs
            // see also: https://github.com/pop-os/cosmic-text/discussions/163
            for run in self.buffer.layout_runs() {
                self.height = self
                    .height
                    .max(run.line_top as u32 + run.line_height as u32);
                self.width = self.width.max(run.line_w as u32);
            }
            log::debug!("text layout: {}x{}", self.width, self.height);
        }
        UVec2::new(self.width, self.height)
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        let mut glyph_cache = GLYPH_CACHE.lock().unwrap();

        // Iterate all the glyphs
        for run in self.buffer.layout_runs() {
            for glyph in run.glyphs.iter() {
                let physical_glyph = glyph.physical((0., 0.), 1.0);

                if let Some(glyph) = glyph_cache
                    .cache
                    .entry(physical_glyph.cache_key)
                    .or_insert_with(|| {
                        // For now, draw the glyph with a white color
                        // TODO: transparency
                        let glyph_color = match glyph.color_opt {
                            Some(some) => some,
                            None => cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                        };
                        swash_cache
                            .get_image_uncached(&mut font_system, physical_glyph.cache_key)
                            .and_then(|image| {
                                convert_image(&image, Color::from(glyph_color)).map(|texture| {
                                    CachcedGlyph {
                                        texture,
                                        placement: image.placement,
                                    }
                                })
                            })
                    })
                {
                    let texture = BorrowedBuffer::from_bytes(
                        &glyph.texture,
                        glyph.placement.width,
                        glyph.placement.height,
                    );

                    draw_handle.draw_texture(
                        (pos.x as i32 + physical_glyph.x + glyph.placement.left) as u32,
                        (pos.y as i32 + run.line_y as i32 + physical_glyph.y - glyph.placement.top)
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
            // 8-bit alpha mask
            for i in 0..glyph_size {
                let j = i * 4;
                let pixel_color = color.premultiply_with(image.data[i]);
                buffer[j] = pixel_color.red();
                buffer[j + 1] = pixel_color.green();
                buffer[j + 2] = pixel_color.blue();
                buffer[j + 3] = pixel_color.alpha();
            }
        }
        cosmic_text::SwashContent::Color => {
            // 32-bit RGBA bitmap
            // This is used by characters that have color, like emojis
            for i in 0..glyph_size {
                let j = i * 4;
                let r = image.data[j];
                let g = image.data[j + 1];
                let b = image.data[j + 2];
                let a = image.data[j + 3];

                let pixel_color = Color::from_rgba(r, g, b, a).premultiply();

                buffer[j] = pixel_color.red();
                buffer[j + 1] = pixel_color.green();
                buffer[j + 2] = pixel_color.blue();
                buffer[j + 3] = pixel_color.alpha();
            }
        }
        cosmic_text::SwashContent::SubpixelMask => panic!("subpixel mask not supported"),
    }

    Some(buffer)
}

#[derive(Clone)]
pub struct Editor {
    inner: Rc<RefCell<cosmic_text::Editor<'static>>>,
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
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
            buf.set_text(&mut font_system, "", DEFAULT_ATTRS, Shaping::Advanced);
            // intial text must be set
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
            KeyCode::Delete => self.perform_action(Action::Delete),
            KeyCode::ArrowLeft => self.perform_action(Action::Motion(Motion::Left)),
            KeyCode::ArrowRight => self.perform_action(Action::Motion(Motion::Right)),
            KeyCode::Home => self.perform_action(Action::Motion(Motion::Home)),
            KeyCode::End => self.perform_action(Action::Motion(Motion::End)),
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
    width: u32,
    height: u32,
}

impl TextEditor {
    pub fn new(editor: Editor, font_size: f32) -> Self {
        editor.inner.borrow_mut().with_buffer_mut(|buf| {
            buf.set_metrics(
                &mut FONT_SYSTEM.lock().unwrap(),
                Metrics::new(font_size, font_size),
            );
        });
        Self {
            editor,
            width: 0,
            height: 0,
        }
    }
}

impl Widget for TextEditor {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.editor.inner.borrow_mut().with_buffer_mut(|buf| {
            let (buf_width, buf_height) = (
                buf.size().0.map(|f| f as u32),
                buf.size().1.map(|f| f as u32),
            );
            // only relayout if the bounds have changed
            if Some(bounds.x) != buf_width || Some(bounds.y) != buf_height {
                buf.set_size(
                    &mut FONT_SYSTEM.lock().unwrap(),
                    Some(bounds.x as f32),
                    Some(bounds.y as f32),
                );
                self.width = bounds.x;
                self.height = bounds.y.min(buf.metrics().line_height as u32);
                log::debug!("text editor layout: {}x{}", self.width, self.height);
            }
        });
        UVec2::new(self.width, self.height)
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
