use crate::render::RenderBuffer;

use super::{Color, Element, Length, Rect, UVec2, Widget};

pub fn container(child: impl Widget + 'static) -> Container {
    Container {
        child: child.into_element(),
        bg_color: None,
        padding: 0,
        width: Length::Auto,
        height: Length::Auto,
        layout_size: UVec2::zero(),
    }
}

pub struct Container {
    child: Element,
    bg_color: Option<Color>,
    padding: u64,
    width: Length,
    height: Length,
    layout_size: UVec2,
}

impl Container {
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

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

impl Widget for Container {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let padding_x2 = 2 * self.padding;
        let child_size = self
            .child
            .layout(bounds - UVec2::new(padding_x2, padding_x2));

        let layout_width = match self.width {
            Length::Auto => child_size.x + padding_x2,
            Length::Fixed(width) => width,
            Length::Fill => bounds.x,
        };
        let layout_height = match self.height {
            Length::Auto => child_size.y + padding_x2,
            Length::Fixed(height) => height,
            Length::Fill => bounds.y,
        };
        log::debug!("container layout: {}x{}", layout_width, layout_height);
        self.layout_size = UVec2::new(layout_width, layout_height);
        self.layout_size
    }

    fn render(&self, pos: UVec2, buf: &mut RenderBuffer) {
        if let Some(bg_color) = self.bg_color {
            buf.fill_rect(Rect::from_pos_size(pos, self.layout_size), bg_color);
        }
        self.child
            .render(pos + UVec2::new(self.padding, self.padding), buf);
    }
}
