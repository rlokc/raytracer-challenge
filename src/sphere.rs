use std::sync::Arc;

use crate::scene_object::SceneObject;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Sphere {
    id: i32
}

impl SceneObject for Sphere {
    fn id(&self) -> i32 {
        self.id
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        let mut rng = rand::thread_rng();
        Sphere { id: rng.gen::<i32>() }
    }
}

pub fn sphere() -> Arc<Box<Sphere>> {
    Arc::new(Box::new(Sphere::new()))
}