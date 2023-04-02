use std::ops::MulAssign;
use std::ops::{Mul, Sub};

use super::Matrix4x4;
use super::Vector;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub xyz: [f32; 3],
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { xyz: [x, y, z] }
    }

    pub fn new_arr(xyz: [f32; 3]) -> Self {
        let xyz = [xyz[0], xyz[1], xyz[2]];
        Self { xyz }
    }

    pub const fn zero() -> Self {
        Self {
            xyz: [0.0, 0.0, 0.0],
        }
    }

    pub fn round(&self, epsilon: f32) -> Self {
        Self::new_arr(self.xyz.map(|v| (v / epsilon).round() * epsilon))
    }
}

// subtracting two points results in a vector
impl Sub<Self> for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Vector {
        Vector::new(
            self.xyz[0] - rhs.xyz[0],
            self.xyz[1] - rhs.xyz[0],
            self.xyz[2] - rhs.xyz[0],
        )
    }
}

// adding a vector to a point results in a new point
impl Sub<Vector> for Point {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self {
        Self::new(
            self.xyz[0] - rhs.xyz[0],
            self.xyz[1] - rhs.xyz[1],
            self.xyz[2] - rhs.xyz[2],
        )
    }
}

impl Mul<Matrix4x4> for Point {
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

        Self::new(x, y, z)
    }
}

impl MulAssign<Matrix4x4> for Point {
    fn mul_assign(&mut self, rhs: Matrix4x4) {
        *self = *self * rhs
    }
}
