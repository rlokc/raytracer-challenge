use crate::colors::Color;
use crate::light::lighting;
use std::{ops::Index, sync::Arc};

use crate::ray::Ray;
use crate::scene_object::MutSceneObject;
use crate::tuple::Tuple;
use crate::world::World;

#[derive(Debug)]
pub struct Intersection {
    pub t: f32,
    pub scene_object: MutSceneObject,
}

#[derive(Debug)]
pub struct IntersectionPrecomputations {
    pub t: f32,
    pub scene_object: MutSceneObject,
    pub point: Tuple,      // in world space
    pub eye_vector: Tuple, // pointing back towards the eye
    pub normal_vector: Tuple,
    pub is_inside_object: bool,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        let id_a = self.scene_object.lock().unwrap().id();
        let id_b = other.scene_object.lock().unwrap().id();
        return self.t == other.t && id_a == id_b;
    }

    fn ne(&self, other: &Self) -> bool {
        let id_a = self.scene_object.lock().unwrap().id();
        let id_b = other.scene_object.lock().unwrap().id();
        return self.t != other.t || id_a != id_b;
    }
}

impl Intersection {
    pub fn new(t: f32, scene_object: MutSceneObject) -> Intersection {
        Intersection { t, scene_object }
    }
}

#[derive(Debug)]
pub struct Intersections {
    pub values: Vec<Arc<Intersection>>,
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

    pub fn concat(&mut self, mut other: Intersections) {
        self.values.append(other.values.as_mut());
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn hit(&self) -> Option<Arc<Intersection>> {
        self.values
            .iter()
            .filter(|i| i.t > 0.0)
            // .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).expect("Tried to compare to NaN"))
            .min_by(|i1, i2| i1.t.total_cmp(&i2.t))
            .cloned()
    }

    pub fn sort(&mut self) {
        self.values.sort_by(|a, b| a.t.total_cmp(&b.t));
    }
}

pub fn intersect(so: MutSceneObject, r: Ray) -> Intersections {
    let mut tf = so.lock().unwrap().transformation();
    tf = tf.invert().unwrap();
    let r = r.transform(&tf);
    r.intersect(so)

    // so.get_mut().unwrap().set_transformation()
}

pub fn intersect_world(world: Arc<World>, r: Ray) -> Intersections {
    let mut res = Intersections::new();
    for obj in world.objects.iter() {
        res.concat(intersect(obj.clone(), r));
    }
    res.sort();
    res
}

pub fn prepare_computations(
    intersection: Arc<Intersection>,
    ray: Ray,
) -> IntersectionPrecomputations {
    let point = ray.position(intersection.t);

    let eye_vector = ray.direction.negate();
    let mut normal_vector = intersection.scene_object.lock().unwrap().normal_at(point);

    let mut is_inside_object = false;

    if normal_vector.dot(eye_vector) < 0.0 {
        is_inside_object = true;
        normal_vector = normal_vector.negate();
    }

    IntersectionPrecomputations {
        t: intersection.t,
        scene_object: intersection.scene_object.clone(),
        point,
        eye_vector,
        normal_vector,
        is_inside_object,
    }
}

pub fn shade_hit(world: Arc<World>, precomputed: &IntersectionPrecomputations) -> Color {
    lighting(
        &precomputed.scene_object.lock().unwrap().material(),
        &world.light_sources[0],
        precomputed.point,
        precomputed.eye_vector,
        precomputed.normal_vector,
    )
}

pub fn color_at(world: Arc<World>, ray: Ray) -> Color {
    let intersections = intersect_world(world.clone(), ray);
    match intersections.hit() {
        None => Color::new(0.0, 0.0, 0.0),
        Some(intersection) => {
            let precomputed = prepare_computations(intersection, ray);
            shade_hit(world.clone(), &precomputed)
        }
    }
}
