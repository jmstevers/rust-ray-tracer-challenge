// use crate::{
//     math::Matrix4x4,
//     rendering::{
//         canvas::{Canvas, Coordinate},
//         object::Object,
//         ray::Ray,
//         Color, Material,
//     },
// };
// use std::{fs::File, io::Write};

// pub fn run() -> std::io::Result<()> {
//     let mut canvas = Canvas::new(1000, 1000, Color::new(0.0, 0.0, 0.0));
//     let white = Color::new(1.0, 1.0, 1.0);

//     let sphere = Object::new_sphere(
//         Material::default(),
//         Matrix4x4::identity().translate(500.0, 500.0, 2000000.0),
//     );

//     let hits: Vec<Ray> = canvas
//         .pixels
//         .iter()
//         .map(|x| Ray::new(x.0 .0 as f32, x.0 .1 as f32, 0.0, 0.0, 0.0, 1.0))
//         .filter(|x| x.intersection(&sphere).is_some())
//         .collect();

//     for i in hits.iter() {
//         canvas.pixels.insert(
//             Coordinate(i.origin.xyz[0] as i16, i.origin.xyz[1] as i16),
//             white,
//         );
//     }

//     let mut output = File::create("output.ppm")?;

//     output.write(canvas.to_ppm().as_bytes())?;

//     Ok(())
// }
