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
    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

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

    pub fn dot(self, rhs: Vector) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vector) -> Vector {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn to_string<'a>(self) -> String {
        format!("{} {} {} ", self.red, self.green, self.blue)
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color() {
        let color_a = Color::new(1.0, 0.2, 0.4);
        let color_b = Color::new(0.9, 1.0, 0.1);

        assert_eq!(color_a * color_b, Color::new(0.9, 0.2, 0.04));
    }
}
