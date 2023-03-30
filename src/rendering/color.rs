use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub rgb: [f32; 3],
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { rgb: [r, g, b] }
    }

    pub fn new_arr(rgb: [f32; 3]) -> Color {
        Color { rgb }
    }

    // turns our color format into the rgb format
    pub fn to_rgb(self) -> Rgb {
        Rgb::new_arr(self.rgb.map(|x| (x * 255.0).round() as u8))
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::new_arr(self.rgb.zip(rhs.rgb).map(|x| x.0 + x.1))
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color::new_arr(self.rgb.zip(rhs.rgb).map(|x| x.0 - x.1))
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color::new_arr(self.rgb.map(|x| x * rhs))
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new_arr(self.rgb.zip(rhs.rgb).map(|x| x.0 * x.1))
    }
}

pub struct Rgb {
    pub rgb: [u8; 3],
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { rgb: [r, g, b] }
    }

    pub fn new_arr(rgb: [u8; 3]) -> Rgb {
        Rgb { rgb }
    }

    pub fn to_string(self) -> String {
        format!("{} {} {}", self.rgb[0], self.rgb[1], self.rgb[2])
    }
}
