pub mod canvas;
pub mod color;
pub mod intersection;
pub mod material;
pub mod object;
pub mod point_light;
pub mod ray;
pub mod world;

pub use canvas::Canvas;
pub use color::Color;
pub use material::Material;
pub use object::Object;
pub use point_light::PointLight;
pub use ray::Ray;

use crate::math::{Point, Vector};

pub fn lighting(
    material: Material,
    light: PointLight,
    position: Point,
    eye: Vector,
    normal: Vector,
) -> Color {
    let effective_color = material.color * light.intensity;
    let light_vector = (light.position - position).normalize();
    let ambient = effective_color * material.ambient;
    let light_dot_normal = light_vector.dot(normal);

    let mut diffuse = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);
    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflect = light_vector.reflect(normal);
        let reflect_dot_eye = reflect.dot(eye);

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}
