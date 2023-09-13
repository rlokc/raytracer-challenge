use std::{sync::{Arc, Mutex}, ops::DerefMut};

use crate::{scene_object::SceneObject, matrix::Matrix};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32,
    pub transform: Arc<Mutex<Matrix>>,
}

impl SceneObject for Sphere {
    fn id(&self) -> i32 {
        self.id
    }

    fn transformation(&self) -> Arc<Mutex<Matrix>> {
        self.transform.clone()
    }

    fn set_transformation(&mut self, transform: Arc<Mutex<Matrix>>) {
        self.transform = transform
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        let mut rng = rand::thread_rng();
        Sphere { 
            id: rng.gen::<i32>(),
            transform: Arc::new(Mutex::new(Matrix::identity_matrix(4)))
        }
    }
}

pub fn sphere() -> Arc<Box<dyn SceneObject>> {
    Arc::new(Box::new(Sphere::new()))
}