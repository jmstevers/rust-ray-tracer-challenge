use crate::math::types::Color;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
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
    pub fn new(width: i16, height: i16) -> Canvas {
        let mut pixels = HashMap::new();

        for i in 0..(width - 1) {
            for j in 0..(height - 1) {
                pixels.insert(
                    Coordinate { x: i, y: j },
                    Color {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                    },
                );
            }
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn to_ppm<'a>(self) -> String {
        let mut result = String::from(
            "\
P3
5 3
255",
        );

        for i in 0..(self.width - 1) {
            result += "\n";
            for j in 0..(self.height - 1) {
                result += self
                    .pixels
                    .get(&Coordinate::new(i, j))
                    .unwrap()
                    .to_string()
                    .as_str();
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn canvas() {
        let mut canvas = Canvas::new(5, 3);

        let color_a = Color::new(1.5, 0.0, 0.0);
        let color_b = Color::new(0.0, 0.5, 0.0);
        let color_c = Color::new(-0.5, 0.0, 1.0);

        canvas.pixels.insert(Coordinate::new(0, 0), color_a);
        canvas.pixels.insert(Coordinate::new(2, 1), color_b);
        canvas.pixels.insert(Coordinate::new(4, 2), color_c);

        assert_eq!(
            canvas.to_ppm(),
            "\
P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 "
        );
    }

    #[test]
    fn print() {
        let canvas = Canvas::new(5, 3);

        println!("{:?}", canvas.pixels);
    }
}
