pub mod matrix;
pub mod point;
pub mod util;
pub mod vector;

pub use self::matrix::*;
pub use self::point::Point;
pub use self::vector::Vector3;

#[cfg(test)]
mod test {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn scale() {
        let transform = Matrix4x4::scaling(2.0, 3.0, 4.0);
        let point = Point::new([-4.0, 6.0, 8.0]);

        assert_eq!(point * transform, Point::new([-8.0, 18.0, 32.0]))
    }

    #[test]
    fn rotation_x() {
        let point_a = Point::new([0.0, 1.0, 0.0]);
        let point_b = Point::new([0.0, 1.0, 0.0]);
        let half_quarter = Matrix4x4::x_rotation(PI / 4.0);
        let full_quarter = Matrix4x4::x_rotation(PI / 2.0);

        assert_eq!(
            point_a * half_quarter,
            Point::new([0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0])
        );

        assert_eq!(point_b * full_quarter, Point::new([0.0, 0.0, 1.0]));
    }

    #[test]
    fn inverse_rotation_x() {
        let point_a = Point::new([0.0, 1.0, 0.0]);
        let point_b = Point::new([0.0, 1.0, 0.0]);
        let half_quarter = Matrix4x4::x_rotation(PI / 4.0);
        let inverse_rotation = half_quarter.inverse().unwrap();

        assert_eq!(
            point_a * half_quarter,
            Point::new([0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0])
        );

        assert_eq!(
            point_b * inverse_rotation,
            Point::new([0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / -2.0])
        );
    }
}
