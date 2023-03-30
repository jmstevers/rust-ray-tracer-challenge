pub mod canvas;
pub mod color;
pub mod intersection;
pub mod material;
pub mod object;
pub mod point_light;
pub mod ray;

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

        let reflect = light_vector.negate().reflect(normal);
        let reflect_dot_eye = reflect.dot(eye);

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}

#[cfg(test)]
mod test {
    use crate::math::{Point, Vector};

    use super::*;

    #[test]
    fn lighting_d() {
        let material = Material::default();
        let position = Point::zero();

        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            lighting(material, light, position, eye_vector, normal_vector),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_a() {
        let material = Material::default();
        let position = Point::zero();

        let eye_vector = Vector::new(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            lighting(material, light, position, eye_vector, normal_vector),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn lighting_b() {
        let material = Material::default();
        let position = Point::zero();

        let eye_vector = Vector::new(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            lighting(material, light, position, eye_vector, normal_vector),
            Color::new(1.6364, 1.6364, 1.6364)
        );
    }

    #[test]
    fn lighting_c() {
        let material = Material::default();
        let position = Point::zero();

        let eye_vector = Vector::new(0.0, 0.0, -1.0);
        let normal_vector = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        assert_eq!(
            lighting(material, light, position, eye_vector, normal_vector),
            Color::new(0.1, 0.1, 0.1)
        );
    }
}
