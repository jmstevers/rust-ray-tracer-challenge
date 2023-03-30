pub mod matrix;
pub mod point;
pub mod util;
pub mod vector;

pub use self::matrix::*;
pub use self::point::Point;
pub use self::vector::Vector;

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn transforms() {
        let point = Point::new(1.0, 0.0, 1.0);
        let transform = Matrix4x4::identity()
            .translate(8.0, 5.0, 7.0)
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0);

        assert_eq!((point * transform).xyz, [13.0, 0.0, 7.0]);
    }
}
