use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    pub fn to_tuple(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, other: Point) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, other: Vector) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }

    pub fn to_tuple(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    pub fn round(self) -> Vector {
        Vector::new(
            (self.x * 100000.0).round() / 100000.0,
            (self.y * 100000.0).round() / 100000.0,
            (self.z * 100000.0).round() / 100000.0,
        )
    }

    pub fn negate(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
            .abs()
            .sqrt()
    }

    pub fn normalize(self) -> Vector {
        let mag = self.magnitude();

        Vector::new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn dot(self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, other: f32) -> Vector {
        Vector::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, other: f32) -> Vector {
        Vector::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_vector() {
        let vector_a = Vector::new(1.0, 2.0, 3.0);
        let vector_b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(vector_a.cross(vector_b), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(vector_b.cross(vector_a), Vector::new(1.0, -2.0, 1.0));
    }
}
