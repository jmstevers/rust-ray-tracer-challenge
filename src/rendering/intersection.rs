use super::object::Object;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Intersection {
    pub time: f32,
    pub object: Object,
}

impl Intersection {
    pub fn new(time: f32, object: Object) -> Intersection {
        Self { time, object }
    }
}
