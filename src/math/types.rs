use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z }
    }
}

// subtracting two points results in a vector
impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// adding a vector to a point results in a new point
impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    // rounds the vector to 5 decimal places (for floating point approximation)
    pub fn round(self) -> Vector {
        Vector::new(
            (self.x * 100000.0).round() / 100000.0,
            (self.y * 100000.0).round() / 100000.0,
            (self.z * 100000.0).round() / 100000.0,
        )
    }

    // negates the vector (-x, -y, -z)
    pub fn negate(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }

    // returns the magnitude of the vector (sqrt(x^2 + y^2 + z^2)
    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
            .abs()
            .sqrt()
    }

    // returns a new vector with the same direction as the original but with a magnitude of 1
    pub fn normalize(self) -> Vector {
        let mag = self.magnitude();

        Vector::new(self.x / mag, self.y / mag, self.z / mag)
    }

    // returns the dot product of two vectors (x1 * x2 + y1 * y2 + z1 * z2)
    pub fn dot(self, rhs: Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    // returns the cross product of two vectors (y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    pub fn cross(self, rhs: Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

// adding two vectors results in a new vector
impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// subtracting two vectors results in a new vector
impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// subtracting a vector from a vector results in a new vector
impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// multiplying a vector by a scalar results in a new vector
impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// dividing a vector by a scalar results in a new vector
impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
