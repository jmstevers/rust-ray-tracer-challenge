use super::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl Material {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn default() -> Self {
        Self::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
    }
}
