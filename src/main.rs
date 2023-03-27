mod math;
mod rendering;

use math::{Point, Vector3};
use rendering::{Canvas, Color, Coordinate};
use std::{fs::File, io::Write};

struct Projectile {
    position: Point,
    velocity: Vector3,
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Vector3,
    wind: Vector3,
}

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::new(900, 550, Color::new(0.0, 0.0, 0.0));

    let environment = Environment {
        gravity: Vector3::new([0.0, -0.1, 0.0]),
        wind: Vector3::new([-0.01, 0.0, 0.0]),
    };

    let mut projectile = Projectile {
        position: Point::new([0.0, 1.0, 0.0]),
        velocity: Vector3::new([1.0, 1.8, 0.0]).normalize() * 11.25,
    };

    loop {
        projectile = tick(environment, projectile);

        println!("{}", projectile.position.xyz[0]);

        if projectile.position.xyz[1] <= 0.0 {
            break;
        }

        let coordinate = Coordinate::new(
            projectile.position.xyz[0] as i16,
            canvas.height - projectile.position.xyz[1] as i16,
        );

        canvas.pixels.insert(coordinate, Color::new(1.0, 0.0, 0.0));
    }

    let mut output = File::create("output.ppm")?;

    output.write(canvas.to_ppm().as_bytes())?;
    Ok(())
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.velocity + proj.position,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
