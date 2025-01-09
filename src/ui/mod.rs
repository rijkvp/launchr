mod color;
mod container;
mod flex;
mod list;
mod math;
mod sized_box;
mod text;

pub use color::*;
pub use container::*;
pub use flex::*;
pub use list::*;
pub use math::*;
pub use sized_box::*;
pub use text::*;

use crate::render::DrawHandle;

/// A graphical user interface component that can be laid out and rendered
pub trait Widget {
    /// Layout the widget and its children (recursively), returns the size the widget takes up
    /// Any implementations should not take up more space than the bounds given
    fn layout(&mut self, bounds: UVec2) -> UVec2;

    /// Renders the widget and its children (recursively) at the given position
    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle);

    /// Converts the widget into a [`DynWidget`]
    fn into_dyn(self) -> DynWidget
    where
        Self: Sized + 'static,
    {
        DynWidget(Box::new(self))
    }
}

/// A dynamically typed [`Widget`]
/// Any [`Widget`] can be converted into a [`DynWidget`] using the [`Widget::into_dyn`] method
pub struct DynWidget(Box<dyn Widget>);

impl Widget for DynWidget {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.0.layout(bounds)
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        self.0.render(pos, draw_handle)
    }

    #[inline]
    fn into_dyn(self) -> DynWidget {
        // an already dynamic widget should not be double-boxed
        self
    }
}
