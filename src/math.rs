//! A module for operations related to math.

use std::ops;

/// A 2D vector.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Vector2f {
    /// x coordinate of the vector.
    pub x: f32,

    /// y coordinate of the vector.
    pub y: f32,
}

impl Vector2f {
    /// Creates a new `Vector2f` with default values.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates new `Vector2f` with given `x` and `y` coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// # use dinai::math::Vector2f;
    /// let vector = Vector2f::from_coords(1.23, 3.21);
    ///
    /// assert!(((vector.x - vector.y) - (1.23 - 3.21)).abs() < 0.00001);
    /// ```
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
