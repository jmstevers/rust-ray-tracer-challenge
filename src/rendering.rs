use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

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

    // turns our color format into the rgb format
    pub fn to_rgb(self) -> Rgb {
        Rgb::new(
            (self.red * 255.0).round() as u8,
            (self.green * 255.0).round() as u8,
            (self.blue * 255.0).round() as u8,
        )
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

pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn to_string(self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Coordinate {
    pub x: i16,
    pub y: i16,
}

impl Coordinate {
    pub fn new(x: i16, y: i16) -> Coordinate {
        Coordinate { x, y }
    }
}

pub struct Canvas {
    pub width: i16,
    pub height: i16,
    pub pixels: HashMap<Coordinate, Color>,
}

impl Canvas {
    // create a new canvas with all pixels set to black
    pub fn new(width: i16, height: i16, color: Color) -> Canvas {
        let mut pixels = HashMap::new();

        for i in 0..(height) {
            for j in 0..(width) {
                pixels.insert(Coordinate { x: j, y: i }, color);
            }
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    // write the canvas to a ppm file format
    pub fn to_ppm(self) -> String {
        // header of ppm file
        let mut result = format!(
            "\
P3
{} {}
255",
            self.width, self.height
        );

        // loops through each pixel and adds it to the result string
        for i in 0..(self.height) {
            result += "\n";

            // seperated into a new variable to make sure each line isnt too long
            let mut line = String::new();

            // so we dont make a new line for every value greater than 70, just the first one
            let mut count = 0;

            for j in 0..(self.width) {
                // gets the color of the pixel and converts it to a string
                let color_as_rgb_string = self
                    .pixels
                    .get(&Coordinate::new(j, i))
                    .unwrap()
                    .to_rgb()
                    .to_string();

                line += color_as_rgb_string.as_str();

                if line.len() >= 70 + count {
                    // makes sure each line isnt too long
                    line += "\n";
                    count += 70;
                } else if j != self.width - 1 {
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

#[cfg(test)]
mod test {
    use super::*;
}
