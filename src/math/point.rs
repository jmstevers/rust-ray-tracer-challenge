use std::ops::{Mul, Sub};

use super::Matrix4x4;
use super::Vector3;

#[derive(PartialEq, Debug)]
pub struct Point {
    pub xyz: [f32; 3],
}

impl Point {
    pub fn new(xyz: [f32; 3]) -> Point {
        Point { xyz }
    }
}

// subtracting two points results in a vector
impl Sub<Point> for Point {
    type Output = Vector3;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector3::new([
            self.xyz[0] - rhs.xyz[0],
            self.xyz[1] - rhs.xyz[0],
            self.xyz[2] - rhs.xyz[0],
        ])
    }
}

// adding a vector to a point results in a new point
impl Sub<Vector3> for Point {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self {
        Point::new([
            self.xyz[0] - rhs.xyz[0],
            self.xyz[1] - rhs.xyz[1],
            self.xyz[2] - rhs.xyz[2],
        ])
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

        Point::new([x, y, z])
    }
}
