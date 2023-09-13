use std::{sync::Arc, ops::Index};

use crate::{scene_object::SceneObject, sphere::Sphere};


#[derive(Debug)]
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

#[derive(Debug)]
pub struct Intersections {
    pub values: Vec<Arc<Intersection>>
}

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl Intersections {
    pub fn new() -> Intersections {
        Intersections { values: vec![] }
    }

    pub fn push(&mut self, intersection: Arc<Intersection>) -> &mut Self {
        self.values.push(intersection);
        self
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn hit(&self) -> Option<Arc<Intersection>> {
        self.values.iter()
            .filter(|i| i.t > 0.0)
            // .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).expect("Tried to compare to NaN"))
            .min_by(|i1, i2| i1.t.total_cmp(&i2.t))
            .cloned()
    }
}