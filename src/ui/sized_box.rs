use crate::render::DrawHandle;

use super::{Color, Length, Rect, UVec2, Widget};

#[allow(dead_code)]
// NOTE: not used at the moment
pub struct SizedBox {
    color: Option<Color>,
    width: Length,
    height: Length,
    layout_size: UVec2,
}

impl Default for SizedBox {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl SizedBox {
    pub fn new() -> Self {
        Self {
            color: None,
            width: Length::Auto,
            height: Length::Auto,
            layout_size: UVec2::ZERO,
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
