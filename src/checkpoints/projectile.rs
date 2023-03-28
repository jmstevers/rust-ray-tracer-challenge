use std::{fs::File, io::Write};

use crate::{
    math::{Point, Vector},
    rendering::canvas::{Canvas, Color, Coordinate},
};

pub struct Projectile {
    pub position: Point,
    pub velocity: Vector,
}

#[derive(Clone, Copy)]
pub struct Environment {
    pub gravity: Vector,
    pub wind: Vector,
}

pub fn run() -> std::io::Result<()> {
    let mut canvas = Canvas::new(900, 550, Color::new(0.0, 0.0, 0.0));

    let environment = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 10.25,
    };

    loop {
        projectile = tick(environment, projectile);

        if projectile.position.xyz[1] <= 0.0 {
            break;
        }

        let coordinate = Coordinate(
            projectile.position.xyz[0] as i16,
            canvas.height - projectile.position.xyz[1] as i16,
        );

        canvas.pixels.insert(coordinate, Color::new(1.0, 0.0, 0.0));
    }

    let mut output = File::create("output.ppm")?;

    output.write(canvas.to_ppm().as_bytes())?;
    Ok(())
}

pub fn tick(env: Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.velocity + proj.position,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
