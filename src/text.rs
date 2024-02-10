use cosmic_text::{
    Action, Attrs, Buffer, Edit, Family, FontSystem, Metrics, Motion, Shaping, SwashCache,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use tiny_skia::{Color, PixmapMut};
use winit::keyboard::KeyCode;

static FONT_SYSTEM: Lazy<Mutex<FontSystem>> = Lazy::new(|| Mutex::new(FontSystem::new()));
static SWASH_CACHE: Lazy<Mutex<SwashCache>> = Lazy::new(|| Mutex::new(SwashCache::new()));

pub struct Text {
    buffer: cosmic_text::Buffer,
    attrs: cosmic_text::Attrs<'static>,
}

// TODO: Improve performance
fn fill_rect(pixmap: &mut PixmapMut, x: i32, y: i32, w: u32, h: u32, color: cosmic_text::Color) {
    let (x, y, w, h) = (x as usize, y as usize, w as usize, h as usize);
    let (width, height) = (pixmap.width() as usize, pixmap.height() as usize);
    if x >= width || y >= height {
        return;
    }
    let color = Color::from_rgba8(color.r(), color.g(), color.b(), color.a())
        .premultiply()
        .to_color_u8();
    let pixels = pixmap.pixels_mut();
    for j in y..(y + h).min(height as usize) {
        for i in x..(x + w).min(width as usize) {
            let index = j * width + i;
            pixels[index] = color;
        }
    }
}

impl Text {
    pub fn new() -> Self {
        let buffer = cosmic_text::Buffer::new_empty(Metrics::new(32.0, 32.0));
        let attrs = Attrs::new().family(Family::Monospace);
        Self { buffer, attrs }
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut, width: u32, height: u32) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        self.buffer
            .set_size(&mut font_system, width as f32, height as f32);
        self.buffer.draw(
            &mut font_system,
            &mut swash_cache,
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            |x, y, w, h, color| fill_rect(pixmap, x, y, w, h, color),
        );
    }

    pub fn set_text(&mut self, text: &str) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        self.buffer
            .set_text(&mut font_system, text, self.attrs, Shaping::Basic);
    }
}

pub struct Editor {
    editor: cosmic_text::Editor<'static>,
    attrs: cosmic_text::Attrs<'static>,
}

impl Editor {
    pub fn new() -> Self {
        let mut editor = cosmic_text::Editor::new(Buffer::new_empty(Metrics::new(32.0, 32.0)));
        let attrs = Attrs::new().family(Family::Monospace);
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        editor.with_buffer_mut(|buf| {
            buf.set_text(&mut font_system, "", attrs, Shaping::Basic);
        });
        Self { editor, attrs }
    }

    pub fn text(&self) -> String {
        self.editor
            .with_buffer(|buf| buf.lines[0].text().to_string())
    }

    pub fn render(&mut self, pixmap: &mut PixmapMut, width: u32, height: u32) {
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut swash_cache = SWASH_CACHE.lock().unwrap();
        self.editor.with_buffer_mut(|buf| {
            buf.set_size(&mut font_system, width as f32, height as f32);
        });
        self.editor.draw(
            &mut font_system,
            &mut swash_cache,
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            cosmic_text::Color::rgb(0xFF, 0xFF, 0xFF),
            |x, y, w, h, color| fill_rect(pixmap, x, y, w, h, color),
        );
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
        self.editor.shape_as_needed(&mut font_system, true);
    }
}
