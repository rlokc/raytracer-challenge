use crate::colors::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        return self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.specular == other.specular
            && self.shininess == other.shininess;
    }
}

impl Material {
    pub fn default() -> Material {
        let color = Color::new(1.0, 1.0, 1.0);
        let default_ambient = 0.1;
        let default_diffuse = 0.9;
        let default_specular = 0.9;
        let default_shininess = 200.0;
        Material::new(
            color,
            default_ambient,
            default_diffuse,
            default_specular,
            default_shininess,
        )
    }

    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}
