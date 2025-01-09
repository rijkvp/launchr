use crate::render::DrawHandle;

use super::{DynWidget, UVec2, Widget};
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
struct ListItems {
    items: Vec<DynWidget>,
    dirty: bool,
}

#[derive(Clone)]
pub struct ListContent {
    inner: Rc<RefCell<ListItems>>,
}

impl ListContent {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(ListItems::default())),
        }
    }

    pub fn update<I, E>(&mut self, new_items: I)
    where
        I: IntoIterator<Item = E>,
        E: Into<DynWidget>,
    {
        let mut new_items: Vec<DynWidget> = new_items.into_iter().map(|c| c.into()).collect();
        let mut inner = self.inner.borrow_mut();
        inner.items.clear();
        inner.items.append(&mut new_items);
        inner.dirty = true;
    }
}

pub struct DynamicList {
    content: ListContent,
    item_height: u32,
    spacing: u32,
    current_width: u32,
    max_items: usize,
}

impl DynamicList {
    pub fn new(content: ListContent, item_height: u32) -> Self {
        Self {
            content,
            item_height,
            spacing: 0,
            current_width: 0,
            max_items: 0,
        }
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for DynamicList {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let mut inner = self.content.inner.borrow_mut();
        let max_items = (bounds.y / (self.item_height + self.spacing)) as usize;
        // only relayout childs if width  has changed or items have changed
        if bounds.x != self.current_width || max_items > self.max_items || inner.dirty {
            log::debug!(
                "relayout list (max. {} items): {}x{}",
                max_items,
                bounds.x,
                bounds.y
            );

            // Bounds for each child is constant
            let child_bounds = UVec2::new(bounds.x, self.item_height);
            for child in inner.items.iter_mut().take(max_items as usize) {
                child.layout(child_bounds);
            }

            self.current_width = bounds.x;
            inner.dirty = false;
        }
        self.max_items = max_items;

        bounds // always fill whole area
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        for (i, child) in self
            .content
            .inner
            .borrow()
            .items
            .iter()
            .take(self.max_items)
            .enumerate()
        {
            let offset = UVec2::new(0, i as u32 * (self.item_height + self.spacing));
            child.render(pos + offset, draw_handle);
        }
    }
}
