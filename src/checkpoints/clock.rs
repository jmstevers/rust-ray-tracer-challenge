use crate::{
    math::{Matrix4x4, Point},
    rendering::{Canvas, Color, Coordinate},
};
use std::{f32::consts::PI, fs::File, io::Write};

pub fn run() -> std::io::Result<()> {
    let mut canvas = Canvas::new(1000, 1000, Color::new(0.0, 0.0, 0.0));
    let white = Color::new(1.0, 1.0, 1.0);
    let mut hand = Point::new(0.0, 450.0, 0.0);

    for _ in 0..100000000 {
        let xy = Coordinate::new((hand.xyz[0] + 500.0) as i16, (hand.xyz[1] + 500.0) as i16);
        canvas.pixels.insert(xy, white);
        canvas.pixels.insert(xy + (0, 1), white);
        canvas.pixels.insert(xy + (1, 0), white);
        canvas.pixels.insert(xy + (1, 1), white);
        hand *= Matrix4x4::identity().z_rotation(PI / 50000000.0)
    }

    let mut output = File::create("output.ppm")?;

    output.write(canvas.to_ppm().as_bytes())?;

    Ok(())
}
