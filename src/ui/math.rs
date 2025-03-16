#[derive(Clone, Copy, Debug, Default)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for UVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for UVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl std::ops::Mul<u32> for UVec2 {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(u32, u32)> for UVec2 {
    fn from((x, y): (u32, u32)) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub pos: UVec2,
    pub size: UVec2,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            pos: UVec2::new(x, y),
            size: UVec2::new(width, height),
        }
    }

    pub fn from_pos_size(pos: UVec2, size: UVec2) -> Self {
        Self { pos, size }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Length {
    #[default]
    Auto,
    Fixed(u32),
    Fill,
}

impl From<u32> for Length {
    fn from(val: u32) -> Self {
        Length::Fixed(val)
    }
}
