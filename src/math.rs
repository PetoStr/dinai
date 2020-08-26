//! A module for operations related to math.

use std::ops;

/// Performs the sigmoid function.
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + std::f32::consts::E.powf(-x))
}

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

/// A matrix with dynamic dimensions. Every cell is of type `f32`.
#[derive(Debug, Clone)]
pub struct Matrixf {
    data: Vec<Vec<f32>>,
    rows: usize,
    columns: usize,
}

impl Matrixf {
    /// Creates new `Matrixf` with given dimension. Every cell is initialized to `0.0f32`.
    pub fn new(rows: usize, columns: usize) -> Self {
        Self::with_val(0.0, rows, columns)
    }

    /// Creates new `Matrixf` with given dimension. Every cell is initialized to `val`.
    pub fn with_val(val: f32, rows: usize, columns: usize) -> Self {
        Self {
            data: vec![vec![val; columns]; rows],
            rows,
            columns,
        }
    }

    /// Creates new `Matrixf` with given dimension. Every cell is randomly initialized in given
    /// interval. For more information see [`rand::Rng::gen_range`].
    ///
    /// [`rand::Rng::gen_range`]: ../../rand/trait.Rng.html#method.gen_range
    pub fn with_random(rows: usize, columns: usize, low: f32, high: f32) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut data = Vec::with_capacity(rows);
        for y in 0..rows {
            data.push(Vec::with_capacity(columns));

            for _ in 0..columns {
                data[y].push(rng.gen_range(low, high));
            }
        }

        Self {
            data,
            rows,
            columns,
        }
    }

    /// Returns a reference to the content of this matrix. Specifically, the returned data is a
    /// reference to a vector of rows and every row is a vector of cells.
    pub fn data(&self) -> &Vec<Vec<f32>> {
        &self.data
    }

    /// Returns how many rows this matrix has.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns how many columns this matrix has.
    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Multiplies this matrix with the `rhs` matrix on the right producing a new matrix.
    ///
    /// # Panics
    ///
    /// This multiplication panics if the matrix on the left does not have as many columns as there
    /// are rows in the right matrix.
    pub fn mul_matrix(&self, rhs: &Matrixf) -> Self {
        assert!(
            self.columns == rhs.rows,
            "left matrix should have as many columns \
            as there are rows in the right matrix"
        );

        let mut res = Matrixf::new(self.rows, rhs.columns);

        for ly in 0..self.rows {
            for rx in 0..rhs.columns {
                let mut val = 0.0;
                for lx in 0..self.columns {
                    val += self.data[ly][lx] * rhs.data[lx][rx];
                }

                res.data[ly][rx] = val;
            }
        }

        res
    }

    /// Applies given operation to every cell of this matrix.
    pub fn apply<T>(&mut self, func: T)
    where
        T: Fn(f32) -> f32,
    {
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell = func(*cell);
            }
        }
    }

    /// Multiplies every cell of this matrix with given scalar.
    pub fn mul_scalar(&mut self, scalar: f32) {
        self.apply(|x| x * scalar);
    }

    /// Performs addition with a matrix on the right. These matrices must have same dimensions.
    ///
    /// # Panics
    ///
    /// This addition panics if these matrices do not have same dimensions.
    pub fn add_matrix(&mut self, rhs: &Matrixf) {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.columns, rhs.columns);

        for y in 0..self.rows {
            for x in 0..self.columns {
                self.data[y][x] += rhs.data[y][x];
            }
        }
    }
}

impl From<Vec<Vec<f32>>> for Matrixf {
    fn from(data: Vec<Vec<f32>>) -> Self {
        let rows = data.len();
        let columns = if data.is_empty() { 0 } else { data[0].len() };

        Self {
            data,
            rows,
            columns,
        }
    }
}

impl From<&Matrixf> for Matrixf {
    #[inline]
    fn from(matrix: &Matrixf) -> Self {
        matrix.clone()
    }
}

impl AsRef<Vec<Vec<f32>>> for Matrixf {
    #[inline]
    fn as_ref(&self) -> &Vec<Vec<f32>> {
        self.data()
    }
}

impl ops::Index<usize> for Matrixf {
    type Output = Vec<f32>;

    #[inline]
    fn index(&self, key: usize) -> &Self::Output {
        &self.data[key]
    }
}

impl ops::AddAssign<&Matrixf> for Matrixf {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_matrix(rhs);
    }
}

impl ops::Add<&Matrixf> for Matrixf {
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_matrix(rhs);
        self
    }
}

impl ops::MulAssign<f32> for Matrixf {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.mul_scalar(rhs);
    }
}

impl ops::Mul<f32> for Matrixf {
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: f32) -> Self::Output {
        self.mul_scalar(rhs);
        self
    }
}

impl ops::Mul<Matrixf> for f32 {
    type Output = Matrixf;

    #[inline]
    fn mul(self, mut rhs: Self::Output) -> Self::Output {
        rhs.mul_scalar(self);
        rhs
    }
}

impl ops::MulAssign<&Matrixf> for Matrixf {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_matrix(rhs);
    }
}

impl ops::Mul<&Matrixf> for Matrixf {
    type Output = Matrixf;

    #[inline]
    fn mul(self, rhs: &Self::Output) -> Self::Output {
        self.mul_matrix(rhs)
    }
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

    fn f32_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.00001
    }

    fn matrix_eq(a: &Matrixf, b: &Matrixf) -> bool {
        a.data
            .iter()
            .zip(b.data.iter())
            .all(|(r1, r2)| r1.iter().zip(r2.iter()).all(|(&a, &b)| f32_eq(a, b)))
    }

    #[test]
    fn test_vec_add() {
        let a = Vector2f::from_coords(1.0, 1.0);
        let b = Vector2f::from_coords(2.0, 3.0);

        let res = a + b;

        assert!(f32_eq(res.x, 3.0) && f32_eq(res.y, 4.0));
    }

    #[test]
    fn test_vec_add_assign() {
        let mut a = Vector2f::from_coords(1.0, 1.0);
        let b = Vector2f::from_coords(2.0, 3.0);

        a += b;

        assert!(f32_eq(a.x, 3.0) && f32_eq(a.y, 4.0));
    }

    #[test]
    fn test_matrix_mul1() {
        let a = Matrixf::from(vec![vec![0.0, 5.0, 1.5], vec![2.0, 2.5, -0.5]]);
        let b = Matrixf::from(vec![vec![0.0, 5.0], vec![2.0, 2.5], vec![1.0, -2.5]]);

        let expected = Matrixf::from(vec![vec![11.5, 8.75], vec![4.5, 17.5]]);

        let res = a * &b;

        assert_eq!(res.rows, expected.rows);
        assert_eq!(res.columns, expected.columns);

        assert!(
            matrix_eq(&expected, &res),
            "expected: {:?}, got: {:?}",
            expected,
            res
        );
    }

    #[test]
    fn test_matrix_mul2() {
        let a = Matrixf::from(vec![vec![2.3, 1.4, 4.5], vec![6.8, 3.1, 2.55]]);
        let b = Matrixf::from(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 10.0, 11.0, 12.0],
        ]);

        let expected = Matrixf::from(vec![
            vec![49.8, 58.0, 66.2, 74.4],
            vec![45.25, 57.7, 70.15, 82.6],
        ]);

        let res = a * &b;

        assert_eq!(res.rows, expected.rows);
        assert_eq!(res.columns, expected.columns);

        assert!(
            matrix_eq(&expected, &res),
            "expected: {:?}, got: {:?}",
            expected,
            res
        );
    }

    #[test]
    #[should_panic]
    fn test_matrix_mul_panic() {
        let a = Matrixf::from(vec![vec![0.0, 5.0, 1.5], vec![2.0, 2.5, -0.5]]);
        let b = Matrixf::from(vec![vec![0.0, 5.0], vec![2.0, 2.5]]);

        let _ = a.mul_matrix(&b);
    }

    #[test]
    fn test_matrix_mul_scalar() {
        let mut a = Matrixf::from(vec![vec![2.3, 1.4, 4.5], vec![6.8, 3.1, 2.55]]);
        let expected = Matrixf::from(vec![vec![4.6, 2.8, 9.0], vec![13.6, 6.2, 5.1]]);

        a *= 2.0;

        assert!(
            matrix_eq(&expected, &a),
            "expected: {:?}, got: {:?}",
            expected,
            a
        );
    }

    #[test]
    fn test_matrix_add() {
        let mut a = Matrixf::from(vec![vec![1.2, 4.4, 1.5], vec![0.8, 8.1, 8.5]]);
        let b = Matrixf::with_val(1.0, 2, 3);

        let expected = Matrixf::from(vec![vec![2.2, 5.4, 2.5], vec![1.8, 9.1, 9.5]]);

        a += &b;

        assert!(
            matrix_eq(&expected, &a),
            "expected: {:?}, got: {:?}",
            expected,
            a
        );
    }

    #[test]
    #[should_panic]
    fn test_matrix_add_panic() {
        let mut a = Matrixf::from(vec![vec![1.2, 4.4, 1.5], vec![0.8, 8.1, 8.5]]);
        let b = Matrixf::with_val(1.0, 3, 3);

        a.add_matrix(&b);
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

    #[test]
    fn test_sigmoid() {
        assert!(f32_eq(sigmoid(1.234), 0.7745179));
    }
}
