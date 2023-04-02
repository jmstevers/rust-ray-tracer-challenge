use std::fmt::Write;

use unroll::unroll_for_loops;

use super::Color;

pub struct Canvas {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    // create a new canvas with all pixels set to black
    #[unroll_for_loops]
    pub fn new(width: i32, height: i32, color: Color) -> Self {
        let mut pixels = Vec::new();

        for i in 0..(width) {
            pixels.push(Vec::new());
            for _ in 0..(height) {
                pixels[i as usize].push(color);
            }
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    #[unroll_for_loops]
    pub fn to_ppm(&self) -> String {
        let size: usize = (self.width * self.height * 12 + 16) as usize;
        let mut result = String::with_capacity(size); // pre-allocate buffer

        write!(&mut result, "P3\n{} {}\n255\n", self.width, self.height).unwrap();

        self.pixels.iter().for_each(|row| {
            row.iter().for_each(|pixel| {
                let rgb = pixel.to_rgb();

                if !result.ends_with('\n') {
                    result.push(' ');
                }

                if result.len() - result.rfind('\n').unwrap_or(0) + rgb.len() > 70 {
                    result.push('\n');
                }

                result.push_str(&rgb);
                result.push(' ');
            });

            result.push('\n');
        });

        result
    }
}
