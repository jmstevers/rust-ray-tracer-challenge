use super::object::Object;

pub struct Intersection<T: Object> {
    pub time: f32,
    pub object: T,
}

impl<T: Object> Intersection<T> {
    pub fn new(time: f32, object: T) -> Intersection<T> {
        Intersection { time, object }
    }

    pub fn group(self, other: Intersection<T>) -> [Intersection<T>; 2] {
        [self, other]
    }
}
