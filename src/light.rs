use crate::colors::Color;
use crate::material::Material;
use crate::tuple::Tuple;

#[derive(Debug)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight {
            intensity,
            position
        }
    }
}

pub fn lighting(material: &Material, light: &PointLight, position: Tuple, eye_vector: Tuple, normal_vector: Tuple) -> Color {
    // Combine surface color with light's color/intensity
    let effective_color = material.color.mul(light.intensity);

    // Direction of the light source
    let light_vector = light.position.sub(position).normalize();

    // Ambient contribution
    let ambient = effective_color.scalar_mul(material.ambient);

    // light_dot_normal represents the cosine of the angle between the light vector and the normal
    // vector. A negative number means the light is on the other side of the surface.
    // Thus only return ambient
    let light_dot_normal = light_vector.dot(normal_vector);
    let mut diffuse = Color::new(0.0, 0.0, 0.0); // black
    let mut specular = Color::new(0.0, 0.0, 0.0);
    if light_dot_normal > 0.0 {
        diffuse = effective_color.scalar_mul(material.diffuse).scalar_mul(light_dot_normal);

        // reflect_dot_eye represents the cosine of the angle between the reflection vector
        // and the eye vector. A negative number means the light reflects away from the eye,
        // thus no specular component
        let reflect_vector = light_vector.scalar_mul(-1.0).reflect(normal_vector);
        let reflect_dot_eye = reflect_vector.dot(eye_vector);
        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity.scalar_mul(material.specular).scalar_mul(factor);
        }
    }

    ambient.add(diffuse).add(specular)
}