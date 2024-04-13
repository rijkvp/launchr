mod container;
mod flex;
mod list;
mod text;

pub use container::*;
pub use flex::*;
pub use text::*;
pub use list::*;

#[derive(Clone, Copy, Debug)]
pub struct UVec2 {
    pub x: u64,
    pub y: u64,
}

impl UVec2 {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add for UVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for UVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub pos: UVec2,
    pub size: UVec2,
}

impl Rect {
    pub fn new(x: u64, y: u64, width: u64, height: u64) -> Self {
        Self {
            pos: UVec2::new(x, y),
            size: UVec2::new(width, height),
        }
    }

    pub fn from_pos_size(pos: UVec2, size: UVec2) -> Self {
        Self { pos, size }
    }
}

impl Into<tiny_skia::Rect> for Rect {
    fn into(self) -> tiny_skia::Rect {
        tiny_skia::Rect::from_xywh(
            self.pos.x as f32,
            self.pos.y as f32,
            self.size.x as f32,
            self.size.y as f32,
        )
        .unwrap()
    }
}

pub type Color = tiny_skia::Color;

#[derive(Debug, Clone, Copy)]
pub enum Length {
    Auto,
    Fixed(u64),
    Fill,
}

/// A graphical component in the UI
pub trait Widget {
    /// Layout the component and its children, returning the size of the component
    fn layout(&mut self, bounds: UVec2) -> UVec2;
    /// Renders the component to the pixmap.
    fn render(&self, pos: UVec2, pixmap: &mut tiny_skia::PixmapMut);
    /// Converts the widget into an element
    fn into_element(self) -> Element
    where
        Self: Sized + 'static,
    {
        Element {
            widget: Box::new(self),
        }
    }
}

/// A generic widget
pub struct Element {
    widget: Box<dyn Widget>,
}

impl From<Box<dyn Widget>> for Element {
    fn from(widget: Box<dyn Widget>) -> Self {
        Self { widget }
    }
}

impl Widget for Element {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.widget.layout(bounds)
    }

    fn render(&self, pos: UVec2, pixmap: &mut tiny_skia::PixmapMut) {
        self.widget.render(pos, pixmap);
    }

    fn into_element(self) -> Element {
        self
    }
}

pub struct SizedBox {
    color: Option<Color>,
    width: Length,
    height: Length,
    layout_size: UVec2,
}

impl SizedBox {
    pub fn new() -> Self {
        Self {
            color: None,
            width: Length::Auto,
            height: Length::Auto,
            layout_size: UVec2::zero(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

impl Widget for SizedBox {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let layout_width = match self.width {
            Length::Auto => 0,
            Length::Fixed(width) => width,
            Length::Fill => bounds.x,
        };
        let layout_height = match self.height {
            Length::Auto => 0,
            Length::Fixed(height) => height,
            Length::Fill => bounds.y,
        };
        self.layout_size = UVec2::new(layout_width, layout_height);
        log::debug!("sized box layout: {}x{}", layout_width, layout_height);
        self.layout_size
    }

    fn render(&self, pos: UVec2, pixmap: &mut tiny_skia::PixmapMut) {
        if let Some(color) = self.color {
            let mut paint = tiny_skia::Paint::default();
            paint.set_color(color);
            pixmap.fill_rect(
                Rect::from_pos_size(pos, self.layout_size).into(),
                &paint,
                tiny_skia::Transform::identity(),
                None,
            );
        }
    }
}
