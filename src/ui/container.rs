use super::{Color, DynWidget, Length, Rect, UVec2, Widget};
use crate::render::DrawHandle;

pub struct Container {
    child: DynWidget,
    bg_color: Option<Color>,
    padding: UVec2,
    width: Length,
    height: Length,
    layout_size: UVec2,
}

pub fn container(child: impl Widget + 'static) -> Container {
    Container {
        child: child.into_dyn(),
        bg_color: None,
        padding: UVec2::ZERO,
        width: Length::Auto,
        height: Length::Auto,
        layout_size: UVec2::ZERO,
    }
}

impl Container {
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn padding_all(mut self, padding: u32) -> Self {
        self.padding = UVec2::new(padding, padding);
        self
    }

    pub fn padding(mut self, padding: impl Into<UVec2>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl Widget for Container {
    fn layout(&mut self, bounds: UVec2) -> UVec2 {
        let padding_x2 = self.padding * 2;
        let child_size = self.child.layout(bounds - padding_x2);

        let layout_width = match self.width {
            Length::Auto => child_size.x + padding_x2.x,
            Length::Fixed(width) => width,
            Length::Fill => bounds.x,
        };
        let layout_height = match self.height {
            Length::Auto => child_size.y + padding_x2.y,
            Length::Fixed(height) => height,
            Length::Fill => bounds.y,
        };
        log::debug!("container layout: {}x{}", layout_width, layout_height);
        self.layout_size = UVec2::new(layout_width, layout_height);
        self.layout_size
    }

    fn render(&self, pos: UVec2, draw_handle: &mut DrawHandle) {
        if let Some(bg_color) = self.bg_color {
            draw_handle.draw_rect(Rect::from_pos_size(pos, self.layout_size), bg_color);
        }
        self.child.render(pos + self.padding, draw_handle);
    }
}
