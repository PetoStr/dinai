//! A module for operations related to math.

use std::ops;

/// A 2D `f32` vector.
#[derive(Debug, Copy, Clone, Default)]
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

/// An axis-aligned bounding box.
#[derive(Debug, Clone)]
pub struct AABBf {
    /// The minimal point of this box (upper left corner).
    pub min: Vector2f,

    /// The maximal point of this box (lower right corner).
    pub max: Vector2f,
}

impl AABBf {
    /// Test whether two AABB boxes intersect.
    ///
    /// # Examples
    ///
    /// ```
    /// use dinai::math::{AABBf, Vector2f};
    ///
    /// let left = AABBf {
    ///     min: Vector2f::from_coords(0.0, 0.0),
    ///     max: Vector2f::from_coords(25.0, 25.0),
    /// };
    ///
    /// let right = AABBf {
    ///     min: Vector2f::from_coords(20.0, 0.0),
    ///     max: Vector2f::from_coords(45.0, 25.0),
    /// };
    ///
    /// assert!(left.intersects(&right));
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x > other.min.x
            && other.max.x > self.min.x
            && self.max.y > other.min.y
            && other.max.y > self.min.y
    }
}

impl ops::Add<Vector2f> for Vector2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vector2f> for Vector2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<f32> for Vector2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vector2f::from_coords(1.0, 1.0);
        let b = Vector2f::from_coords(2.0, 3.0);

        let res = a + b;

        assert!((res.x - res.y) - (3.0 - 4.0) < 0.0001);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vector2f::from_coords(1.0, 1.0);
        let b = Vector2f::from_coords(2.0, 3.0);

        a += b;

        assert!((a.x - a.y) - (3.0 - 4.0) < 0.0001);
    }

    #[test]
    fn test_intersection() {
        let left = AABBf {
            min: Vector2f::from_coords(-20.0, 0.0),
            max: Vector2f::from_coords(25.0, 25.0),
        };

        let right = AABBf {
            min: Vector2f::from_coords(20.0, 0.0),
            max: Vector2f::from_coords(45.0, 25.0),
        };

        assert!(left.intersects(&right));
    }

    #[test]
    fn test_no_intersection() {
        let left = AABBf {
            min: Vector2f::from_coords(-20.0, 0.0),
            max: Vector2f::from_coords(25.0, 25.0),
        };

        let right = AABBf {
            min: Vector2f::from_coords(25.1, 0.0),
            max: Vector2f::from_coords(50.1, 25.0),
        };

        assert!(!left.intersects(&right));
    }
}
