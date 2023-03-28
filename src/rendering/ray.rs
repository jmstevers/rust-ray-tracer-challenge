use crate::math::{Point, Vector};

use super::{
    intersection::Intersection,
    object::{Object, Sphere},
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
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
}

#[cfg(test)]
mod test {
    use crate::rendering::{intersection::Intersection, object::Sphere};

    use super::*;

    #[test]
    fn intersection() {
        let sphere = Sphere::new(0);
        let xs = Intersection::new(3.5, sphere);

        assert_eq!(xs.time, 3.5);
        assert_eq!(xs.object, sphere);
    }

    #[test]
    fn intersection_a() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let sphere = Sphere::new(0);
        let xs = match ray.intersection(sphere) {
            Some(_) => ray.intersection(sphere).unwrap(),
            None => return,
        };

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, sphere);
        assert_eq!(xs[1].object, sphere);
    }
}
