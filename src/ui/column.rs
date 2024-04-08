use super::{Element, Length, UVec2, Widget};

use tiny_skia::PixmapMut;

pub fn column(children: Vec<impl Into<Element>>) -> Column {
    Column {
        child_offsets: Vec::with_capacity(children.len()),
        children: children.into_iter().map(|c| c.into()).collect(),
        padding: 0,
        width: Length::Auto,
        height: Length::Auto,
        layout_size: UVec2::zero(),
    }
}

pub struct Column {
    children: Vec<Element>,
    child_offsets: Vec<u64>,
    padding: u64,
    width: Length,
    height: Length,
    layout_size: UVec2,
}

impl Column {
    pub fn padding(mut self, padding: u64) -> Self {
        self.padding = padding;
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

impl Widget for Column {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let padding_x2 = 2 * self.padding;
        let mut child_bounds = bounds - UVec2::new(padding_x2, padding_x2);
        let mut max_width = 0;
        let mut total_height = 0;
        self.child_offsets.clear();
        for child in &mut self.children {
            let size = child.layout(child_bounds);
            child_bounds.y = child_bounds.y.saturating_sub(size.y);
            max_width = max_width.max(size.x);
            self.child_offsets.push(total_height);
            total_height += size.y;
        }

        let layout_width = match self.width {
            Length::Auto => max_width + padding_x2,
            Length::Fixed(width) => width,
            Length::Fill => bounds.x,
        };
        let layout_height = match self.height {
            Length::Auto => total_height + padding_x2,
            Length::Fixed(height) => height,
            Length::Fill => bounds.y,
        };
        log::debug!("Column layout: {}x{}", layout_width, layout_height);
        self.layout_size = UVec2::new(layout_width, layout_height);
        self.layout_size
    }

    fn render(&self, pos: UVec2, pixmap: &mut PixmapMut) {
        for (child, offset) in self.children.iter().zip(self.child_offsets.iter()) {
            child.render(UVec2::new(pos.x, pos.y + offset), pixmap);
        }
    }
}
