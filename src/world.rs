use std::sync::{Arc, Mutex};
use crate::light::PointLight;
use crate::scene_object::SceneObject;

pub struct World {
    pub objects: Vec<Arc<Mutex<Box<dyn SceneObject>>>>,
    pub light_sources: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
            light_sources: Vec::new(),
        }
    }
}