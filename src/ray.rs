
use std::sync::{Arc, Mutex};

use crate::{tuple::Tuple, scene_object::SceneObject, intersection::{Intersections, Intersection}, matrix::Matrix};


#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        assert!(direction.is_vector());
        Ray { origin, direction }
    }

    pub fn new_flat(xo: f32, yo: f32, zo: f32, xd: f32, yd: f32, zd: f32) -> Ray {
        Ray {
            origin: Tuple::point(xo, yo, zo),
            direction: Tuple::vector(xd, yd, zd),
        }
    }

    pub fn position(&self, time: f32) -> Tuple {
        self.origin.add(self.direction.scalar_mul(time))
    }

    pub fn intersect(&self, scene_object: Arc<Mutex<Box<dyn SceneObject>>>) -> Intersections {

        let mut res  = Intersections::new();

        let sphere_to_ray = self.origin.sub(Tuple::point(0.0, 0.0, 0.0));

        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return res;
        }


        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        
        res.push(Arc::new(Intersection::new(t1, scene_object.clone())));
        res.push(Arc::new(Intersection::new(t2, scene_object.clone())));
        return res;
    }

    pub fn transform(&self, transformation_matrix: &Matrix) -> Self {
        let new_origin = transformation_matrix.tuple_mul(&self.origin);
        let new_direction = transformation_matrix.tuple_mul(&self.direction);
        Ray::new(new_origin, new_direction)
    }
}