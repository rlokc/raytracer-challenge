use std::sync::{Arc, Mutex};

use crate::{scene_object::SceneObject, matrix::Matrix};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32,
    pub transform: Matrix,
}

impl SceneObject for Sphere {
    fn id(&self) -> i32 {
        self.id
    }

    fn transformation(&self) -> Matrix {
        self.transform.clone()
    }

    fn set_transformation(&mut self, transform: &Matrix) {
        self.transform = transform.clone()
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        let mut rng = rand::thread_rng();
        Sphere { 
            id: rng.gen::<i32>(),
            transform: Matrix::identity_matrix(4)
        }
    }
}

pub fn sphere() -> Arc<Mutex<Box<dyn SceneObject>>> {
    Arc::new(Mutex::new(Box::new(Sphere::new())))
}