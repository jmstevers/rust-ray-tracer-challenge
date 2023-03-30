use crate::math::{Matrix4x4, Point, Vector};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Shape {
    Sphere,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix4x4,
}

impl Object {
    pub fn new_sphere(transform: Matrix4x4) -> Self {
        Object {
            shape: Shape::Sphere,
            transform,
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        match self.shape {
            Shape::Sphere => self.sphere_normal(point),
        }
    }

    fn sphere_normal(&self, point: Point) -> Vector {
        let sphere_point = point * self.transform.inverse();
        let sphere_normal = sphere_point - Point::zero();
        let world_normal = sphere_normal * self.transform.inverse().transpose();

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
