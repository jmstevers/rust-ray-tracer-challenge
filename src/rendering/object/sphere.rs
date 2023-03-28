use super::Object;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub id: u8,
}

impl Sphere {
    pub fn new(id: u8) -> Sphere {
        Sphere { id }
    }
}

impl Object for Sphere {}
