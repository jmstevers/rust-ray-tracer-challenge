use unroll::unroll_for_loops;

use crate::{
    math::{Matrix4x4, Point},
    rendering::{self, canvas::Canvas, object::Object, ray::Ray, Color, Material, PointLight},
};
use std::{fs::File, io::Write};

#[unroll_for_loops]
pub fn run() -> std::io::Result<()> {
    // let mut canvas = Canvas::new(100, 100, Color::new(0.0, 0.0, 0.0));

    // let mut sphere = Object::new_sphere(
    //     Material::default(),
    //     Matrix4x4::identity()
    //         .translate(500.0, 500.0, 0.0)
    //         .scale(333.0, 333.0, 333.0),
    // );
    // sphere.calc_inverse_transform();
    // sphere.calc_inverse_transpose_transform();
    // sphere.material.color = Color::new(1.0, 0.2, 1.0);

    // let light = PointLight::new(Point::new(0.0, 0.0, 2000.0), Color::new(1.0, 1.0, 1.0));

    // let hits: Vec<(Ray, f32)> = (0..canvas.width)
    //     .flat_map(|i| (0..canvas.height).map(move |j| (i, j)))
    //     .map(|(i, j)| {
    //         let ray = Ray::new(i as f32, j as f32, 0.0, 0.0, 0.0, 1.0);
    //         let intersection = ray.intersection(&mut sphere);
    //         intersection.and_then(|mut x| Ray::hit(&mut x).map(|h| (ray, h.time)))
    //     })
    //     .filter(|x| x.is_some())
    //     .map(|x| x.unwrap())
    //     .collect();

    // hits.iter().for_each(|x| {
    //     let point = x.0.position_at(x.1);
    //     let normal = sphere.normal_at(point);
    //     let eye = x.0.direction.negate();

    //     canvas.pixels[x.0.origin.xyz[0] as usize][x.0.origin.xyz[1] as usize] =
    //         rendering::lighting(sphere.material, light, point, eye, normal);
    // });

    // let mut output = File::create("output.ppm")?;

    // output.write(canvas.to_ppm().as_bytes())?;

    // Ok(())

    let mut canvas = Canvas::new(1000, 1000, Color::new(0.0, 0.0, 0.0));

    let mut sphere = Object::new_sphere(
        Material::default(),
        Matrix4x4::identity()
            .translate(500.0, 500.0, 0.0)
            .scale(333.0, 333.0, 333.0),
    );
    sphere.calc_inverse_transform();
    sphere.calc_inverse_transpose_transform();
    sphere.material.color = Color::new(1.0, 0.2, 0.0);

    let light = PointLight::new(Point::new(0.0, 0.0, 2000.0), Color::new(1.0, 1.0, 1.0));

    let mut hits = Vec::new();

    for i in 0..canvas.width {
        for j in 0..canvas.height {
            let ray = Ray::new(i as f32, j as f32, 0.0, 0.0, 0.0, 1.0);
            let intersection = ray.intersection(&mut sphere);
            if let Some(mut x) = intersection {
                if let Some(h) = Ray::hit(&mut x) {
                    hits.push((ray, h.time));
                }
            }
        }
    }

    for x in &hits {
        let point = x.0.position_at(x.1);
        let normal = sphere.normal_at(point);
        let eye = x.0.direction.negate();

        canvas.pixels[point.xyz[0] as usize][point.xyz[1] as usize] =
            rendering::lighting(sphere.material, light, point, eye, normal);
    }

    let mut output = File::create("output.ppm")?;

    output.write(canvas.to_ppm().as_bytes())?;

    Ok(())
}
