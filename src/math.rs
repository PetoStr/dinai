//! A module for operations related to math.

use rand::distributions::uniform::SampleUniform;
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

/// A generic 2D matrix.
#[derive(Debug, Clone)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Copy + Default, const ROWS: usize, const COLS: usize> Default for Matrix<T, ROWS, COLS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    /// Creates new `Matrix` with default values.
    pub fn new() -> Self {
        Self::with_val(Default::default())
    }
}

impl<T: Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    /// Creates new `Matrix` with the given value.
    pub fn with_val(val: T) -> Self {
        Self {
            data: [[val; COLS]; ROWS],
        }
    }

    /// Applies given operation to every cell of this matrix.
    pub fn apply<U>(&mut self, func: U)
    where
        U: Fn(T) -> T,
    {
        for row in self.data.iter_mut() {
            for cell in row.iter_mut() {
                *cell = func(*cell);
            }
        }
    }
}

impl<T: Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: ops::Mul<Output = T>,
{
    /// Multiplies every cell of this matrix with given scalar.
    pub fn mul_scalar(&mut self, scalar: T) {
        self.apply(|x| x * scalar);
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    /// Multiplies this matrix with the `rhs` matrix on the right producing a new matrix.
    pub fn mul_matrix<const OTH_COLS: usize>(
        &self,
        rhs: &Matrix<T, COLS, OTH_COLS>,
    ) -> Matrix<T, ROWS, OTH_COLS> {
        let mut res: Matrix<T, ROWS, OTH_COLS> = Matrix::new();

        for ly in 0..ROWS {
            for rx in 0..OTH_COLS {
                let mut val = Default::default();
                for lx in 0..COLS {
                    val += self.data[ly][lx] * rhs.data[lx][rx];
                }

                res.data[ly][rx] = val;
            }
        }

        res
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Copy + std::ops::AddAssign,
{
    /// Performs addition with a matrix on the right. These matrices must have same dimensions.
    pub fn add_matrix(&mut self, rhs: &Matrix<T, ROWS, COLS>) {
        for y in 0..ROWS {
            for x in 0..COLS {
                self.data[y][x] += rhs.data[y][x];
            }
        }
    }
}

impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS>
where
    T: Default + Copy + SampleUniform,
{
    /// Creates new `Matrix` with random values.
    pub fn with_random(low: T, high: T) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut res = Matrix::new();
        for y in 0..ROWS {
            for x in 0..COLS {
                res.data[y][x] = rng.gen_range(low, high);
            }
        }

        res
    }

    /// Crossovers two matrices at one random position producing a new matrix.
    pub fn crossover(&self, other: &Matrix<T, ROWS, COLS>) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let pr: usize = rng.gen_range(0, ROWS);
        let pc: usize = rng.gen_range(0, COLS);

        let mut res = self.clone();

        for y in pr..ROWS {
            for x in pc..COLS {
                res.data[y][x] = other.data[y][x];
            }
        }

        res
    }
}

/// Randomly adds Gaussian random value to every cell of the given matrix.
pub fn mutate_matrixf<const ROWS: usize, const COLS: usize>(
    matrix: &mut Matrix<f32, ROWS, COLS>,
    probability: f32,
) {
    use rand::Rng;
    use rand_distr::StandardNormal;

    let mut rng = rand::thread_rng();
    for row in matrix.data.iter_mut() {
        for cell in row.iter_mut() {
            if rng.gen::<f32>() < probability {
                let val: f32 = rng.sample(StandardNormal);
                *cell += val / 5.0;

                if *cell > 1.0 {
                    *cell = 1.0;
                } else if *cell < -1.0 {
                    *cell = -1.0;
                }
            }
        }
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Matrix<T, R, C> {
    fn from(data: [[T; C]; R]) -> Self {
        Self { data }
    }
}

impl<T: Clone, const R: usize, const C: usize> From<&Matrix<T, R, C>> for Matrix<T, R, C> {
    #[inline]
    fn from(matrix: &Matrix<T, R, C>) -> Self {
        matrix.clone()
    }
}

impl<T, const R: usize, const C: usize> AsRef<[[T; C]; R]> for Matrix<T, R, C> {
    #[inline]
    fn as_ref(&self) -> &[[T; C]; R] {
        &self.data
    }
}

impl<T, const R: usize, const C: usize> ops::AddAssign<&Matrix<T, R, C>> for Matrix<T, R, C>
where
    T: Copy + ops::AddAssign,
{
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_matrix(rhs);
    }
}

impl<T, const R: usize, const C: usize> ops::Add<&Matrix<T, R, C>> for Matrix<T, R, C>
where
    T: Copy + ops::AddAssign,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_matrix(rhs);
        self
    }
}

impl<T, const R: usize, const C: usize> ops::MulAssign<T> for Matrix<T, R, C>
where
    T: Copy + ops::Mul<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.mul_scalar(rhs);
    }
}

impl<T, const R: usize, const C: usize> ops::Mul<T> for Matrix<T, R, C>
where
    T: Copy + ops::Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(mut self, rhs: T) -> Self::Output {
        self.mul_scalar(rhs);
        self
    }
}

impl<T, const R: usize, const C: usize, const OC: usize> ops::MulAssign<&Matrix<T, C, OC>>
    for Matrix<T, R, C>
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    #[inline]
    fn mul_assign(&mut self, rhs: &Matrix<T, C, OC>) {
        self.mul_matrix(rhs);
    }
}

impl<T, const R: usize, const C: usize, const OC: usize> ops::Mul<&Matrix<T, C, OC>>
    for Matrix<T, R, C>
where
    T: Copy + Default + std::ops::Mul<Output = T> + std::ops::AddAssign,
{
    type Output = Matrix<T, R, OC>;

    #[inline]
    fn mul(self, rhs: &Matrix<T, C, OC>) -> Self::Output {
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

    fn matrix_eq<const R: usize, const C: usize>(
        a: &Matrix<f32, R, C>,
        b: &Matrix<f32, R, C>,
    ) -> bool {
        a.as_ref()
            .iter()
            .zip(b.as_ref().iter())
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
        let a = Matrix::from([[0.0, 5.0, 1.5], [2.0, 2.5, -0.5]]);
        let b = Matrix::from([[0.0, 5.0], [2.0, 2.5], [1.0, -2.5]]);

        let expected = Matrix::from([[11.5, 8.75], [4.5, 17.5]]);

        let res = a * &b;

        assert!(
            matrix_eq(&expected, &res),
            "expected: {:?}, got: {:?}",
            expected,
            res
        );
    }

    #[test]
    fn test_matrix_mul2() {
        let a = Matrix::from([[2.3, 1.4, 4.5], [6.8, 3.1, 2.55]]);
        let b = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
        ]);

        let expected = Matrix::from([[49.8, 58.0, 66.2, 74.4], [45.25, 57.7, 70.15, 82.6]]);

        let res = a * &b;

        assert!(
            matrix_eq(&expected, &res),
            "expected: {:?}, got: {:?}",
            expected,
            res
        );
    }

    #[test]
    fn test_matrix_mul_scalar() {
        let mut a = Matrix::from([[2.3, 1.4, 4.5], [6.8, 3.1, 2.55]]);
        let expected = Matrix::from([[4.6, 2.8, 9.0], [13.6, 6.2, 5.1]]);

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
        let mut a = Matrix::from([[1.2, 4.4, 1.5], [0.8, 8.1, 8.5]]);
        let b = Matrix::with_val(1.0);

        let expected = Matrix::from([[2.2, 5.4, 2.5], [1.8, 9.1, 9.5]]);

        a += &b;

        assert!(
            matrix_eq(&expected, &a),
            "expected: {:?}, got: {:?}",
            expected,
            a
        );
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
