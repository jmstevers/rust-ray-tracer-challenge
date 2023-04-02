use super::{Object, PointLight};

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
}
