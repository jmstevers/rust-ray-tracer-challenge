use std::ops::Mul;

use crate::math::{Matrix4x4, Point, Vector};

use super::{intersection::Intersection, object::Object};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(px: f32, py: f32, pz: f32, vx: f32, vy: f32, vz: f32) -> Self {
        Self {
            origin: Point::new(px, py, pz),
            direction: Vector::new(vx, vy, vz),
        }
    }

    pub fn point_vector(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn position_at(&self, time: f32) -> Point {
        self.direction * time + self.origin
    }

    pub fn intersection(&self, object: &Object) -> Option<[Intersection; 2]> {
        let inverse_transform = *self * object.inverse_transform;
        let sphere_to_ray = inverse_transform.origin - Point::zero();

        let a = inverse_transform.direction.dot(inverse_transform.direction);
        let b = inverse_transform.direction.dot(sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let two_a = 2.0 * a;
            let inv_two_a = 1.0 / two_a;
            Some([
                Intersection::new((-b - sqrt_discriminant) * inv_two_a, *object),
                Intersection::new((-b + sqrt_discriminant) * inv_two_a, *object),
            ])
        } else {
            None
        }
    }

    pub fn hit(intersections: &mut [Intersection; 2]) -> Option<Intersection> {
        let a = match intersections[0].time > intersections[1].time {
            true => intersections[0],
            false => intersections[1],
        };

        if a.time < 0.0 {
            return None;
        } else {
            return Some(a);
        }
    }
}

impl Mul<Matrix4x4> for Ray {
    type Output = Self;
    fn mul(self, rhs: Matrix4x4) -> Self {
        Self::point_vector(self.origin * rhs, self.direction * rhs)
    }
}
