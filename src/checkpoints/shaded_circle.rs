use crate::{
    math::{Matrix4x4, Point, Vector},
    rendering::{
        self,
        canvas::{Canvas, Coordinate},
        object::Object,
        ray::Ray,
        Color, Material, PointLight,
    },
};
use std::{fs::File, io::Write};

pub fn run() -> std::io::Result<()> {
    let mut canvas = Canvas::new(1000, 1000, Color::new(0.0, 0.0, 0.0));

    let mut sphere = Object::new_sphere(
        Material::default(),
        Matrix4x4::identity()
            .translate(500.0, 500.0, 0.0)
            .scale(333.0, 333.0, 333.0),
    );
    sphere.material.color = Color::new(1.0, 0.2, 1.0);

    let light = PointLight::new(Point::new(0.0, 0.0, 2000.0), Color::new(1.0, 1.0, 1.0));

    let hits: Vec<Ray> = canvas
        .pixels
        .iter()
        .map(|x| {
            Ray::point_vector(
                Point::new(x.0 .0 as f32, x.0 .1 as f32, 0.0),
                Vector::new(0.0, 0.0, 1.0).normalize(),
            )
        })
        .filter(|x| x.intersection(&sphere).is_some())
        .collect();

    for i in hits.iter() {
        let point = i.position_at(
            Ray::hit(&mut i.intersection(&sphere).unwrap().to_vec())
                .unwrap()
                .time,
        );
        let normal = sphere.normal_at(point);
        let eye = i.direction.negate();

        canvas.pixels.insert(
            Coordinate(i.origin.xyz[0] as i16, i.origin.xyz[1] as i16),
            rendering::lighting(sphere.material, light, point, eye, normal),
        );
    }

    let mut output = File::create("output.ppm")?;

    output.write(canvas.to_ppm().as_bytes())?;

    Ok(())
}
