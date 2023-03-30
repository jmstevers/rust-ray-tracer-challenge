use std::{cmp::Ordering, ops::Mul};

use crate::math::{Matrix4x4, Point, Vector};

use super::{intersection::Intersection, object::Object};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(px: f32, py: f32, pz: f32, vx: f32, vy: f32, vz: f32) -> Ray {
        Ray {
            origin: Point::new(px, py, pz),
            direction: Vector::new(vx, vy, vz),
        }
    }

    pub fn point_vector(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position_at(&self, time: f32) -> Point {
        self.direction * time + self.origin
    }

    pub fn intersection(&self, object: &Object) -> Option<[Intersection; 2]> {
        let inverse_transform = *self * object.transform.inverse();
        let sphere_to_ray = inverse_transform.origin - Point::new(0.0, 0.0, 0.0);

        let a = inverse_transform.direction.dot(inverse_transform.direction);
        let b = inverse_transform.direction.dot(sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            return Some([
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), *object),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), *object),
            ]);
        } else {
            return None;
        }
    }

    pub fn hit(intersections: &mut Vec<Intersection>) -> Option<Intersection> {
        loop {
            let a = intersections
                .iter()
                .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap_or(Ordering::Less))
                .copied()?;

            if a.time < 0.0 {
                intersections.retain(|x| x.time != a.time);
            } else if intersections.len() == 0 {
                return None;
            } else {
                return Some(a);
            }
        }
    }
}

impl Mul<Matrix4x4> for Ray {
    type Output = Self;
    fn mul(self, rhs: Matrix4x4) -> Self::Output {
        Ray::point_vector(self.origin * rhs, self.direction * rhs)
    }
}

#[cfg(test)]
mod test {
    use crate::rendering::object::Shape;

    use super::*;

    #[test]
    fn intersect() {
        let ray = Ray::new(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let mut sphere = Object::new_sphere(Matrix4x4::identity());
        let transform = Matrix4x4::identity().scale(2.0, 2.0, 2.0);
        sphere.transform *= transform;

        let xs = match ray.intersection(&sphere) {
            Some(x) => x,
            None => panic!(),
        };

        assert_eq!(xs[0].time, 3.0);
        assert_eq!(xs[1].time, 7.0);
    }

    #[test]
    fn intersect_a() {
        let ray = Ray::new(0.0, 0.0, -5.0, 0.0, 0.0, 1.0);
        let mut sphere = Object::new_sphere(Matrix4x4::identity());
        let transform = Matrix4x4::identity().translate(5.0, 0.0, 0.0);
        sphere.transform *= transform;

        match ray.intersection(&sphere) {
            Some(_) => panic!(),
            None => return,
        };
    }
}
