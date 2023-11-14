use std::{sync::{Arc, Mutex}, ops::Index};

use crate::{scene_object::SceneObject, ray::Ray};


#[derive(Debug)]
pub struct Intersection {
    pub t: f32,
    pub scene_object: Arc<Mutex<Box<dyn SceneObject>>>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        let id_a = self.scene_object.lock().unwrap().id();
        let id_b = other.scene_object.lock().unwrap().id();
        return self.t == other.t && id_a == id_b
    }

    fn ne(&self, other: &Self) -> bool {
        let id_a = self.scene_object.lock().unwrap().id();
        let id_b = other.scene_object.lock().unwrap().id();
        return self.t != other.t || id_a != id_b
    }
}

impl Intersection {
    pub fn new(t: f32, scene_object: Arc<Mutex<Box<dyn SceneObject>>>) -> Intersection {
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

pub fn intersect(so: Arc<Mutex<Box<dyn SceneObject>>>, r: Ray) -> Intersections {
    let mut tf = so.lock().unwrap().transformation();
    tf = tf.invert().unwrap();
    let r = r.transform(&tf);
    r.intersect(so)

    // so.get_mut().unwrap().set_transformation()
}