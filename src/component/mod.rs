use crate::render::Rect;

pub mod container;
pub mod text;

pub enum Length {
    Audo,
    Fill,
}

pub trait Component {
    /// Layout the component and its children
    fn layout(&mut self, width: u64, height: u64);
    /// Renders the component to the pixmap.
    fn render(&self, bounds: Rect, pixmap: &mut tiny_skia::PixmapMut);
}

// pub struct ComponentMut<C: Component>(RefCell<C>);
//
// impl<C: Component> ComponentMut<C> {
//     pub fn new(inner: C) -> Self {
//         Self(RefCell::new(inner))
//     }
// }
//
// impl<C: Component> Component for ComponentMut<C> {
//     fn layout(&mut self, width: u64, height: u64) {
//         self.0.borrow_mut().layout(width, height);
//     }
//
//     fn render(&self, bounds: Rect, pixmap: &mut tiny_skia::PixmapMut) {
//         self.0.borrow().render(bounds, pixmap);
//     }
// }
//
// impl<'a, C: Component> Deref for ComponentMut<C> {
//     type Target = RefCell<C>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<C: Component> DerefMut for ComponentMut<C> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0.borrow_mut()
//     }
// }
