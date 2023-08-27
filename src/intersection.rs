use std::sync::Arc;

use crate::scene_object::{SceneObject};



pub struct Intersection {
    pub t: f32,
    pub scene_object: Arc<Box<dyn SceneObject>>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        return self.t == other.t && self.scene_object.id() == other.scene_object.id()
    }

    fn ne(&self, other: &Self) -> bool {
        return self.t != other.t || self.scene_object.id() != other.scene_object.id()
    }
}

impl Intersection {
    pub fn new(t: f32, scene_object: Arc<Box<dyn SceneObject>>) -> Intersection {
        Intersection {
            t,
            scene_object: scene_object,
        }
    }
}

pub struct Intersections {
    pub values: Vec<Intersection>
}

impl Intersections {
    pub fn new() -> Intersections {
        Intersections { values: vec![] }
    }

    pub fn push(&mut self, intersection: Intersection) -> &mut Self {
        self.values.push(intersection);
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}