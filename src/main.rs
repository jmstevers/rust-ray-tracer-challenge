mod math;

use math::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Clone, Copy)]
struct Enviroment {
    gravity: Vector,
    wind: Vector,
}

fn main() {
    let environment = Enviroment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize() * 50.0,
    };

    loop {
        projectile = tick(environment, projectile);

        println!("{}", projectile.position.y);

        if projectile.position.y <= 0.0 {
            break;
        }
    }
}

fn tick(env: Enviroment, proj: Projectile) -> Projectile {
    Projectile {
        position: proj.velocity + proj.position,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
