use crate::colors::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
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
        ambient: f64,
        diffuse: f64,
        specular: f64,
        shininess: f64,
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
