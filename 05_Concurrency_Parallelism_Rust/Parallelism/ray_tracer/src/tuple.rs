use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    z: f64,
    w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Converting an arbitrary vector into a unit vector.
    /// A unit vector is a vector with magnitude 1.
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        let mut v = Self::new(self.x / magnitude, self.y / magnitude, self.z / magnitude);
        v.w = self.w / magnitude;
        v
    }

    /// The distance of a vector.
    /// It's the length of a straight line from end to end of the vector.
    fn magnitude(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    /// Multiply this vector with another one to reduce it to one scalar number.
    /// This applies the directional growth of one vector to another.
    /// The smaller the value, the larger the angle between the vectors.
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Cross product of two vectors.
    /// The returned vector is perpendicular to the other two.
    /// Order is important. `other.cross(&self)` would return a vector in the
    /// opposite direction.
    fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

trait Tuple<T> {
    fn is_equal(&self, other: &T) -> bool;
}

impl Tuple<Point> for Vector {
    fn is_equal(&self, other: &Point) -> bool {
        is_float_equal(self.x, other.x)
            && is_float_equal(self.y, other.y)
            && is_float_equal(self.z, other.z)
            && is_float_equal(self.w, other.w)
    }
}

impl Tuple<Self> for Vector {
    fn is_equal(&self, other: &Self) -> bool {
        is_float_equal(self.x, other.x)
            && is_float_equal(self.y, other.y)
            && is_float_equal(self.z, other.z)
            && is_float_equal(self.w, other.w)
    }
}

impl Tuple<Self> for Color {
    fn is_equal(&self, other: &Self) -> bool {
        is_float_equal(self.red, other.red)
            && is_float_equal(self.green, other.green)
            && is_float_equal(self.blue, other.blue)
    }
}

const EPSILON: f64 = 0.00001;

fn is_float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Self> for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub<Self> for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Self> for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Mul<i32> for Color {
    type Output = Self;

    fn mul(self, scalar: i32) -> Self::Output {
        Self {
            red: self.red * scalar as f64,
            green: self.green * scalar as f64,
            blue: self.blue * scalar as f64,
        }
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self::Output {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert!(Color::new(0.9, 0.2, 0.04).is_equal(&(c1 * c2)));
    }

    #[test]
    fn multiplying_colors_by_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        assert!(Color::new(0.4, 0.6, 0.8).is_equal(&(c1 * 2)));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!(Color::new(0.2, 0.5, 0.5).is_equal(&(c1 - c2)));
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_new() {
        let color = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(color.red, -0.5);
        assert_eq!(color.green, 0.4);
        assert_eq!(color.blue, 1.7);
    }

    #[test]
    fn cross_product_of_two_vectors_returns_a_vector() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.cross(&v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(&v1), Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn dot_product_of_two_vectors_returns_a_scalar() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert_eq!(v1.dot(&v2), 20.0);
    }

    #[test]
    fn normalizing_vectors() {
        assert_eq!(
            Vector::new(4.0, 0.0, 0.0).normalize(),
            Vector::new(1.0, 0.0, 0.0)
        );

        let sqrt = 14.0_f64.sqrt();
        assert_eq!(
            Vector::new(1.0, 2.0, 3.0).normalize(),
            Vector::new(1.0 / sqrt, 2.0 / sqrt, 3.0 / sqrt)
        );
    }

    #[test]
    fn computing_magnitude_of_other_vectors() {
        let expected_value = 14.0_f64.sqrt();
        assert_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), expected_value);
        assert_eq!(Vector::new(-1.0, -2.0, -3.0).magnitude(), expected_value);
    }

    #[test]
    fn computing_magnitude_of_unit_vectors() {
        assert_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let t1 = Vector::new(1.0, -2.0, 3.0);
        let t2 = Vector::new(0.5, -1.0, 1.5);
        assert_eq!(t1 / 2.0, t2);
    }

    #[test]
    fn dividing_a_point_by_a_scalar() {
        let t1 = Point::new(1.0, -2.0, 3.0);
        let mut t2 = Point::new(0.5, -1.0, 1.5);
        t2.w = 0.5;
        assert_eq!(t1 / 2.0, t2);
    }

    #[test]
    fn multiplying_a_point_by_a_scalar() {
        let t1 = Point::new(1.0, -2.0, 3.0);
        let mut t2 = Point::new(3.5, -7.0, 10.5);
        t2.w = 3.5;
        assert_eq!(t1 * 3.5, t2);
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let t1 = Vector::new(1.0, -2.0, 3.0);
        let t2 = Vector::new(3.5, -7.0, 10.5);
        assert_eq!(t1 * 3.5, t2);
    }

    #[test]
    fn negating_a_vector() {
        let tuple = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(-tuple, Vector::new(-5.0, -6.0, -7.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(zero - v, Vector::new(-5.0, -6.0, -7.0));
    }

    #[test]
    fn subtracting_two_vectors_creates_a_vector() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point_creates_a_point() {
        let point = Point::new(3.0, 2.0, 1.0);
        let vector = Vector::new(5.0, 6.0, 7.0);
        assert_eq!(point - vector, Point::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_points_creates_a_vector() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    #[test]
    fn adding_a_vector_to_a_vector_creates_a_vector() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn adding_a_vector_to_a_point_creates_a_point() {
        let vector = Vector::new(3.0, -2.0, 5.0);
        let point = Point::new(-2.0, 3.0, 1.0);
        assert_eq!(vector + point, Point::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn two_equal_vectors() {
        let vec1 = Vector::new(4.9, -4.1, 3.0);
        let vec2 = Vector::new(4.9, -4.1, 3.0);
        assert!(vec1.is_equal(&vec2));
    }

    #[test]
    fn two_unequal_vectors() {
        let vec1 = Vector::new(4.9, -4.1, 3.0);
        let vec2 = Vector::new(4.9, -4.1, 3.1);
        assert!(!vec1.is_equal(&vec2));
    }

    #[test]
    fn points_and_vectors_are_not_equal() {
        let vector = Vector::new(4.9, -4.1, 3.0);
        let point = Point::new(4.9, -4.1, 3.0);
        assert!(!vector.is_equal(&point));
    }

    #[test]
    fn tuple_with_0_w_is_a_vector() {
        let vector = Vector::new(4.9, -4.1, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn tuple_with_1_w_is_a_point() {
        let point = Point::new(4.9, -4.1, 3.0);
        assert_eq!(point.w, 1.0);
    }
}
