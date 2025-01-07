mod color;
mod container;
mod flex;
mod list;
mod text;

pub use color::*;
pub use container::*;
pub use flex::*;
pub use list::*;
pub use text::*;

use crate::render::DrawHandle;

#[derive(Clone, Copy, Debug)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub fn new(x: u32, y: u32) -> Self {
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
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            pos: UVec2::new(x, y),
            size: UVec2::new(width, height),
        }
    }

    pub fn from_pos_size(pos: UVec2, size: UVec2) -> Self {
        Self { pos, size }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    Auto,
    Fixed(u32),
    Fill,
}

impl Into<Length> for u32 {
    fn into(self) -> Length {
        Length::Fixed(self)
    }
}

/// A graphical component in the UI
pub trait Widget {
    /// Layout the component and its children, returning the size of the component
    fn layout(&mut self, bounds: UVec2) -> UVec2;
    /// Renders the component to the buffer
    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle);
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

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        self.widget.render(pos, draw_handle)
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

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        if let Some(color) = self.color {
            draw_handle.draw_rect(Rect::from_pos_size(pos, self.layout_size), color);
        }
    }
}
