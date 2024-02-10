use cosmic_text::{
    Action, Attrs, CacheKeyFlags, Edit, Family, FontSystem, Metrics, Motion, Shaping, Stretch,
    Style, SwashCache, Weight,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tiny_skia::{Color, PixmapMut};
use winit::keyboard::KeyCode;

static FONT_SYSTEM: Lazy<Mutex<FontSystem>> = Lazy::new(|| Mutex::new(FontSystem::new()));
static SWASH_CACHE: Lazy<Mutex<SwashCache>> = Lazy::new(|| Mutex::new(SwashCache::new()));

pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct Text {
    rect: Rect,
    buffer: cosmic_text::Buffer,
}

const DEFAULT_ATTRS: Attrs = Attrs {
    color_opt: None,
    family: Family::SansSerif,
    stretch: Stretch::Normal,
    style: Style::Normal,
    weight: Weight::NORMAL,
    metadata: 0,
    cache_key_flags: CacheKeyFlags::empty(),
};

impl Text {
    pub fn new(rect: Rect, font_size: f32) -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut buffer =
            cosmic_text::Buffer::new(&mut font_system, Metrics::new(font_size, font_size));
        buffer.set_size(&mut font_system, rect.width as f32, rect.height as f32);
        Self { rect, buffer }
    }

    pub fn set_text(&mut self, text: &str) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.buffer
            .set_text(&mut font_system, text, DEFAULT_ATTRS, Shaping::Basic);
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        self.buffer.draw(
            &mut font_system,
            &mut swash_cache,
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            |x, y, w, h, color| fill_rect(pixmap, self.rect.x + x, self.rect.y + y, w, h, color),
        );
    }
}

pub struct Editor {
    rect: Rect,
    editor: cosmic_text::Editor<'static>,
}

impl Editor {
    pub fn new(rect: Rect, font_size: f32) -> Self {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut buffer =
            cosmic_text::Buffer::new(&mut font_system, Metrics::new(font_size, font_size));
        buffer.set_size(&mut font_system, rect.width as f32, rect.height as f32);
        let mut editor = cosmic_text::Editor::new(buffer);
        editor.with_buffer_mut(|buf| {
            // Intial text must be set
            buf.set_text(&mut font_system, "", DEFAULT_ATTRS, Shaping::Basic);
        });
        Self { rect, editor }
    }

    pub fn text(&self) -> String {
        self.editor
            .with_buffer(|buf| buf.lines[0].text().to_string())
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        self.editor.shape_as_needed(&mut font_system, true);
        if self.editor.redraw() {
            self.editor.draw(
                &mut font_system,
                &mut swash_cache,
                cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
                cosmic_text::Color::rgb(0xAA, 0xFF, 0xFF),
                cosmic_text::Color::rgb(0xAA, 0xAA, 0xFF),
                |x, y, w, h, color| {
                    fill_rect(pixmap, self.rect.x + x, self.rect.y + y, w, h, color)
                },
            );
            self.editor.set_redraw(false);
        }
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
        println!("Action: {:?}", action);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.editor.action(&mut font_system, action);
    }
}

// TODO: Improve performance
fn fill_rect(pixmap: &mut PixmapMut, x: i32, y: i32, w: u32, h: u32, color: cosmic_text::Color) {
    let (x, y, w, h) = (x as usize, y as usize, w as usize, h as usize);
    let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
    let max_x = x.saturating_add(w).min(width); // Prevent overflow & clamp to width
    let max_y = y.saturating_add(h).min(height);
    if max_x <= x || max_y <= y {
        // Don't render if the rect is out of bounds
        return;
    }
    let (r, g, b, a) = (color.r(), color.g(), color.b(), color.a());
    if a == 0 {
        // Don't render transparent pixels
        return;
    }
    let color = Color::from_rgba8(r, g, b, a).premultiply().to_color_u8();
    let pixels = pixmap.pixels_mut();
    for j in y..max_y {
        let row_start = j * width;
        for i in x..max_x {
            pixels[row_start + i] = color;
        }
    }
}
