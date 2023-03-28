use std::{cmp::Ordering, ops::Mul};

use crate::math::{Matrix4x4, Point, Vector};

use super::{
    intersection::Intersection,
    object::{Object, Sphere},
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> Ray {
        Ray {
            origin: Point::new(x1, y1, z1),
            direction: Vector::new(x2, y2, z2),
        }
    }

    pub fn point_vector(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position_at(&self, t: f32) -> Point {
        self.direction * t + self.origin
    }

    pub fn intersection(&self, sphere: Sphere) -> Option<[Intersection<Sphere>; 2]> {
        let sphere_to_ray = self.origin - Point::new(0.0, 0.0, 0.0);

        let a = self.direction.dot(self.direction);
        let b = self.direction.dot(sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant >= 0.0 {
            return Some([
                Intersection::new((-b - discriminant.sqrt()) / (2.0 * a), sphere),
                Intersection::new((-b + discriminant.sqrt()) / (2.0 * a), sphere),
            ]);
        } else {
            return None;
        }
    }

    pub fn hit(intersections: &mut Vec<Intersection<Sphere>>) -> Option<Intersection<Sphere>> {
        // need to make it not return a negative value if it finds a min
        loop {
            let a = intersections
                .iter()
                .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap_or(Ordering::Less))
                .copied();

            let b = match a {
                Some(x) => x,
                None => return None,
            };

            if b.time < 0.0 {
                intersections.retain(|x| x.time != b.time);
            } else if intersections.len() == 0 {
                return None;
            } else {
                return Some(b);
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
    use crate::rendering::{intersection::Intersection, object::Sphere};

    use super::*;

    #[test]
    fn transform() {
        let ray = Ray::new(1.0, 2.0, 3.0, 0.0, 1.0, 0.0);
        let translation = Matrix4x4::identity().translation(3.0, 4.0, 5.0);

        let ray2 = ray * translation;

        assert_eq!(ray2.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(ray2.direction, Vector::new(0.0, 1.0, 0.0));
    }
}
