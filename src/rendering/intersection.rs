use super::object::Object;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Intersection<T: Object> {
    pub time: f32,
    pub object: T,
}

impl<T: Object> Intersection<T> {
    pub fn new(time: f32, object: T) -> Intersection<T> {
        Intersection { time, object }
    }
}
