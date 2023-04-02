use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub rgb: [f32; 3],
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { rgb: [r, g, b] }
    }

    pub fn new_arr(rgb: [f32; 3]) -> Self {
        Self { rgb }
    }

    pub fn to_rgb(&self) -> String {
        let rgb = self.rgb.map(|x| (x * 255.0).round() as u8);
        format!("{} {} {}", rgb[0], rgb[1], rgb[2])
    }
}

impl Add<Self> for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new_arr(self.rgb.zip(rhs.rgb).map(|x| x.0 + x.1))
    }
}

impl Sub<Self> for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new_arr(self.rgb.zip(rhs.rgb).map(|x| x.0 - x.1))
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new_arr(self.rgb.map(|x| x * rhs))
    }
}

impl Mul<Self> for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new_arr(self.rgb.zip(rhs.rgb).map(|x| x.0 * x.1))
    }
}
