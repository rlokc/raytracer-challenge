use crate::colors::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::scene_object::{MutSceneObject, SceneObject};
use crate::sphere::sphere;
use crate::transformations::scale;
use crate::tuple::Tuple;

pub struct World {
    pub objects: Vec<MutSceneObject>,
    pub light_sources: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light_sources: Vec::new(),
        }
    }

    pub fn default_world() -> World {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let sphere1 = sphere();
        let sphere1color = Color::new(0.8, 1.0, 0.6);
        let mut sphere1mat = Material::default();
        sphere1mat.color = sphere1color;
        sphere1mat.diffuse = 0.7;
        sphere1mat.specular = 0.2;

        sphere1.lock().unwrap().set_material(&sphere1mat);

        let sphere2 = sphere();
        let sphere2transform = scale(0.5, 0.5, 0.5);
        sphere2.lock().unwrap().set_transformation(&sphere2transform);

        World {
            objects: vec![sphere1, sphere2],
            light_sources: vec![light],
        }
    }
}