use std::ops;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}

impl Vector2f {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_coords(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl ops::Add<&Vector2f> for Vector2f {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<&Vector2f> for Vector2f {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
