use std::sync::{Arc, Mutex};

use crate::{scene_object::SceneObject, matrix::Matrix};
use rand::Rng;
use crate::material::Material;
use crate::scene_object::MutSceneObject;
use crate::tuple::Tuple;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32,
    pub transform: Matrix,
    pub material: Material,
}

impl SceneObject for Sphere {
    fn id(&self) -> i32 {
        self.id
    }

    fn transformation(&self) -> Matrix {
        self.transform.clone()
    }

    fn set_transformation(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.invert().unwrap().tuple_mul(&world_point);
        let object_normal = object_point.sub(Tuple::point(0.0, 0.0, 0.0));
        let mut world_normal = self.transform.invert().unwrap().transpose().tuple_mul(&object_normal);
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: &Material) {
        self.material = material.clone();
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        let mut rng = rand::thread_rng();
        Sphere {
            material: Material::default(),
            id: rng.gen::<i32>(),
            transform: Matrix::identity_matrix(4)
        }
    }
}

pub fn sphere() -> MutSceneObject {
    Arc::new(Mutex::new(Box::new(Sphere::new())))
}