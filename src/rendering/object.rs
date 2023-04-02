use crate::math::{Matrix4x4, Point, Vector};

use super::Material;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Shape {
    Sphere,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Object {
    pub shape: Shape,
    pub material: Material,
    pub transform: Matrix4x4,
    pub inverse_transform: Matrix4x4,
    pub inverse_transpose_transform: Matrix4x4,
}

impl Object {
    pub fn new_sphere(material: Material, transform: Matrix4x4) -> Self {
        Self {
            shape: Shape::Sphere,
            material,
            transform,
            inverse_transform: Matrix4x4::zero(),
            inverse_transpose_transform: Matrix4x4::zero(),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        match self.shape {
            Shape::Sphere => self.sphere_normal(point),
        }
    }

    pub fn calc_inverse_transform(&mut self) {
        self.inverse_transform = self.transform.inverse().unwrap();
    }

    pub fn calc_inverse_transpose_transform(&mut self) {
        self.inverse_transpose_transform = self.inverse_transform.transpose();
    }

    fn sphere_normal(&self, point: Point) -> Vector {
        let sphere_point = point * self.inverse_transform;
        let sphere_normal = sphere_point - Point::zero();
        let world_normal = sphere_normal * self.inverse_transpose_transform;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reflect() {
        let vector = Vector::new(1.0, -1.0, 0.0);
        let normal = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(vector.reflect(normal), Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_a() {
        let vector = Vector::new(0.0, -1.0, 0.0);
        let normal = Vector::new(2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0);

        assert_eq!(
            vector.reflect(normal).round(1.0),
            Vector::new(1.0, 0.0, 0.0)
        );
    }
}
