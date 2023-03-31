use std::collections::HashMap;
use std::ops::Add;

use unroll::unroll_for_loops;

use super::Color;

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
    #[unroll_for_loops]
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
    #[unroll_for_loops]
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
