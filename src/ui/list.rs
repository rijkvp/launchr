use std::{cell::RefCell, rc::Rc};

use super::{Element, UVec2, Widget};

use tiny_skia::PixmapMut;

#[derive(Clone)]
pub struct ListContent {
    items: Rc<RefCell<Vec<Element>>>,
}

impl ListContent {
    pub fn new() -> Self {
        Self {
            items: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn update<I, E>(&mut self, new_items: I)
    where
        I: IntoIterator<Item = E>,
        E: Into<Element>,
    {
        let mut new_items: Vec<Element> = new_items.into_iter().map(|c| c.into()).collect();
        let mut items = self.items.borrow_mut();
        items.clear();
        items.append(&mut new_items);
    }
}

pub struct DynamicList {
    content: ListContent,
    item_height: u64,
    spacing: u64,
    current_width: u64,
    max_items: usize,
}

impl DynamicList {
    pub fn new(content: ListContent, item_height: u64) -> Self {
        Self {
            content,
            item_height,
            spacing: 0,
            current_width: 0,
            max_items: 0,
        }
    }

    pub fn spacing(mut self, spacing: u64) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Widget for DynamicList {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let mut items = self.content.items.borrow_mut();
        let max_items = (bounds.y / (self.item_height + self.spacing)) as usize;
        // only relayout childs if width  has changed
        if bounds.x != self.current_width || max_items > self.max_items {
            log::debug!(
                "relayout list (max. {} items): {}x{}",
                max_items,
                bounds.x,
                bounds.y
            );
            self.current_width = bounds.x;

            // Bounds for each child is constant
            let child_bounds = UVec2::new(bounds.x, self.item_height);
            for child in items.iter_mut().take(max_items as usize) {
                child.layout(child_bounds);
            }
        }
        self.max_items = max_items;

        bounds // always fill whole area
    }

    fn render(&self, pos: UVec2, pixmap: &mut PixmapMut) {
        for (i, child) in self
            .content
            .items
            .borrow()
            .iter()
            .take(self.max_items)
            .enumerate()
        {
            let offset = UVec2::new(0, i as u64 * (self.item_height + self.spacing));
            child.render(pos + offset, pixmap);
        }
    }
}
