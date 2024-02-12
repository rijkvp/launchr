pub mod container;
pub mod text;

pub trait Drawable {
    fn render(&self, pixmap: &mut tiny_skia::PixmapMut);
}

pub enum Component<'a> {
    Container(container::Container),
    Text(text::Text),
    Editor(&'a text::TextEditor),
}

impl Drawable for Component<'_> {
    fn render(&self, pixmap: &mut tiny_skia::PixmapMut) {
        match self {
            Component::Container(c) => c.render(pixmap),
            Component::Text(t) => t.render(pixmap),
            Component::Editor(e) => e.render(pixmap),
        }
    }
}
