use super::{Element, Length, UVec2, Widget};

use tiny_skia::PixmapMut;

pub fn column<C, E>(children: C) -> Flex
where
    C: IntoIterator<Item = E>,
    E: Into<Element>,
{
    let children: Vec<Element> = children.into_iter().map(|c| c.into()).collect();
    create_flex(FlexDirection::Column, children)
}

pub fn row<C, E>(children: C) -> Flex
where
    C: IntoIterator<Item = E>,
    E: Into<Element>,
{
    let children: Vec<Element> = children.into_iter().map(|c| c.into()).collect();
    create_flex(FlexDirection::Row, children)
}

fn create_flex(direction: FlexDirection, children: Vec<Element>) -> Flex {
    Flex {
        direction,
        children,
        child_offsets: Vec::new(),
        padding: 0,
        width: Length::Auto,
        height: Length::Auto,
        layout_size: UVec2::zero(),
    }
}

pub enum FlexDirection {
    Row,
    Column,
}

pub struct Flex {
    direction: FlexDirection,
    children: Vec<Element>,
    child_offsets: Vec<u64>,
    padding: u64,
    width: Length,
    height: Length,
    layout_size: UVec2,
}

impl Flex {
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

impl Widget for Flex {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let padding_x2 = 2 * self.padding;
        let mut child_bounds = bounds - UVec2::new(padding_x2, padding_x2);

        let mut total_width = 0;
        let mut total_height = 0;
        self.child_offsets.clear();
        for child in &mut self.children {
            let child_size = child.layout(child_bounds);
            match self.direction {
                FlexDirection::Row => {
                    child_bounds.x = child_bounds.x.saturating_sub(child_size.x);
                    total_height = total_height.max(child_size.y);
                    self.child_offsets.push(total_width);
                    total_width += child_size.x;
                }
                FlexDirection::Column => {
                    child_bounds.y = child_bounds.y.saturating_sub(child_size.y);
                    total_width = total_width.max(child_size.x);
                    self.child_offsets.push(total_height);
                    total_height += child_size.y;
                }
            }
        }

        let layout_width = match self.width {
            Length::Auto => total_width + padding_x2,
            Length::Fixed(width) => width,
            Length::Fill => bounds.x,
        };
        let layout_height = match self.height {
            Length::Auto => total_height + padding_x2,
            Length::Fixed(height) => height,
            Length::Fill => bounds.y,
        };
        log::debug!("flex layout: {}x{}", layout_width, layout_height);
        self.layout_size = UVec2::new(layout_width, layout_height);
        self.layout_size
    }

    fn render(&self, pos: UVec2, pixmap: &mut PixmapMut) {
        for (child, offset) in self.children.iter().zip(self.child_offsets.iter()) {
            let offset = match self.direction {
                FlexDirection::Row => UVec2::new(*offset, 0),
                FlexDirection::Column => UVec2::new(0, *offset),
            };
            child.render(pos + offset, pixmap);
        }
    }
}
