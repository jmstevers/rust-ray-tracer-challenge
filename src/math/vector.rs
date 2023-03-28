use std::ops::{Add, Div, Mul, Sub};

use super::{Matrix4x4, Point};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector3 {
    pub xyz: [f32; 3],
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { xyz: [x, y, z] }
    }

    pub fn new_arr(xyz: [f32; 3]) -> Vector3 {
        Vector3 { xyz }
    }

    // rounds the vector to 5 decimal places (for floating point approximation)
    pub fn round(self, decimal_point: f32) -> Vector3 {
        Vector3::new_arr(
            self.xyz
                .map(|v| (v * decimal_point).round() / decimal_point),
        )
    }

    // negates the vector (-x, -y, -z)
    pub fn negate(self) -> Vector3 {
        Vector3::new_arr(self.xyz.map(|x| -x))
    }

    // returns the magnitude of the vector (sqrt(x^2 + y^2 + z^2)
    pub fn magnitude(self) -> f32 {
        self.xyz
            .into_iter()
            .fold(0.0, |acc, x| acc + x * x)
            .abs()
            .sqrt()
    }

    // returns a new vector with the same direction as the original but with a magnitude of 1
    pub fn normalize(self) -> Vector3 {
        let mag = self.magnitude();

        Vector3::new_arr(self.xyz.map(|x| x / mag))
    }

    // returns the dot product of two vectors (x1 * x2 + y1 * y2 + z1 * z2)
    pub fn dot(self, rhs: Vector3) -> f32 {
        self.xyz
            .into_iter()
            .zip(rhs.xyz.into_iter())
            .fold(0.0, |acc, x| acc + x.0 * x.1)
    }

    // returns the cross product of two vectors (y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.xyz[1] * rhs.xyz[2] - self.xyz[2] * rhs.xyz[1],
            self.xyz[2] * rhs.xyz[0] - self.xyz[0] * rhs.xyz[2],
            self.xyz[0] * rhs.xyz[1] - self.xyz[1] * rhs.xyz[0],
        )
    }
}

// adding two vectors results in a new vector
impl Add<Vector3> for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self {
        Vector3::new(
            self.xyz[0] + rhs.xyz[0],
            self.xyz[1] + rhs.xyz[1],
            self.xyz[2] + rhs.xyz[2],
        )
    }
}

// subtracting two vectors results in a new vector
impl Add<Point> for Vector3 {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(
            self.xyz[0] + rhs.xyz[0],
            self.xyz[1] + rhs.xyz[1],
            self.xyz[2] + rhs.xyz[2],
        )
    }
}

// subtracting a vector from a vector results in a new vector
impl Sub<Vector3> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self {
        Vector3::new(
            self.xyz[0] - rhs.xyz[0],
            self.xyz[1] - rhs.xyz[1],
            self.xyz[2] - rhs.xyz[2],
        )
    }
}

// multiplying a vector by a scalar results in a new vector
impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vector3::new_arr(self.xyz.map(|x| x * rhs))
    }
}

// dividing a vector by a scalar results in a new vector
impl Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Vector3::new_arr(self.xyz.map(|x| x / rhs))
    }
}

impl Mul<Matrix4x4> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: Matrix4x4) -> Self {
        let x = self.xyz[0] * rhs.data[0][0]
            + self.xyz[1] * rhs.data[0][1]
            + self.xyz[2] * rhs.data[0][2]
            + 1.0 * rhs.data[0][3];
        let y = self.xyz[0] * rhs.data[1][0]
            + self.xyz[1] * rhs.data[1][1]
            + self.xyz[2] * rhs.data[1][2]
            + 1.0 * rhs.data[1][3];
        let z = self.xyz[0] * rhs.data[2][0]
            + self.xyz[1] * rhs.data[2][1]
            + self.xyz[2] * rhs.data[2][2]
            + 1.0 * rhs.data[2][3];

        Vector3::new(x, y, z)
    }
}
