use std::collections::HashMap;
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

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinate(pub i16, pub i16);

impl Add<(i16, i16)> for Coordinate {
    type Output = Self;
    fn add(self, rhs: (i16, i16)) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Canvas {
    pub width: i16,
    pub height: i16,
    pub pixels: HashMap<Coordinate, Color>,
}

impl Canvas {
    // create a new canvas with all pixels set to black
    pub fn new(width: i16, height: i16, color: Color) -> Self {
        let mut pixels = HashMap::new();

        for i in 0..(width) {
            for j in 0..(height) {
                pixels.insert(Coordinate(i, j), color);
            }
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    // write the canvas to a ppm file format
    pub fn to_ppm(&self) -> String {
        // header of ppm file
        let mut result = format!(
            "\
P3
{} {}
255",
            self.width, self.height
        );

        // loops through each pixel and adds it to the result string
        for i in 0..(self.width) {
            result += "\n";

            // separated into a new variable to make sure each line isn't too long
            let mut line = String::new();

            // so we don't make a new line for every value greater than 70, just the first one
            let mut count = 0;

            for j in 0..(self.height) {
                // gets the color of the pixel and converts it to a string
                let color_as_rgb_string = self
                    .pixels
                    .get(&Coordinate(i, j))
                    .unwrap()
                    .to_rgb()
                    .to_string();

                line += color_as_rgb_string.as_str();

                if line.len() >= 70 + count {
                    // makes sure each line isn't too long
                    line += "\n";
                    count += 70;
                } else if j != self.height - 1 {
                    // adds a space between each pixel
                    line += " ";
                }
            }
            result += line.as_str();
        }
        result += "\n";
        result
    }
}
