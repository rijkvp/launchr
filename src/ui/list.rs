use crate::render::DrawHandle;

use super::{DynWidget, UVec2, Widget};
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct List {
    items: Vec<DynWidget>,
    spacing: u32,
    item_height: u32,
    item_width: u32,
    max_items: usize,
}

impl List {
    pub fn new(item_height: u32, spacing: u32) -> Self {
        Self {
            item_height,
            spacing,
            ..Default::default()
        }
    }

    fn relayout_items(&mut self) {
        log::debug!(
            "relayout list: max items={}, item width={}",
            self.max_items,
            self.item_width
        );
        let item_bounds = UVec2::new(self.item_width, self.item_height);
        for item in self.items.iter_mut().take(self.max_items) {
            item.layout(item_bounds);
        }
    }
}

impl Widget for List {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let new_max_items = (bounds.y / (self.item_height + self.spacing)) as usize;
        // only relayout if nessessary
        let relayout = bounds.x != self.item_width || new_max_items > self.max_items;
        self.item_width = bounds.x;
        self.max_items = new_max_items;
        if relayout {
            self.relayout_items();
        }

        bounds // always fill whole area
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        for (i, child) in self.items.iter().take(self.max_items).enumerate() {
            let offset = UVec2::new(0, i as u32 * (self.item_height + self.spacing));
            child.render(pos + offset, draw_handle);
        }
    }
}

#[derive(Clone)]
pub struct DynamicList(Rc<RefCell<List>>);

impl DynamicList {
    pub fn new(item_height: u32, spacing: u32) -> Self {
        Self(Rc::new(RefCell::new(List::new(item_height, spacing))))
    }

    pub fn update<I, E>(&mut self, new_items: I)
    where
        I: IntoIterator<Item = E>,
        E: Into<DynWidget>,
    {
        let mut new_items: Vec<DynWidget> = new_items.into_iter().map(|c| c.into()).collect();
        let mut inner = self.0.borrow_mut();
        inner.items.clear();
        inner.items.append(&mut new_items);
        inner.relayout_items();
    }

    pub fn max_items(&self) -> usize {
        self.0.borrow().max_items
    }
}

impl Widget for DynamicList {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        self.0.borrow_mut().layout(bounds)
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        self.0.borrow().render(pos, draw_handle);
    }
}
