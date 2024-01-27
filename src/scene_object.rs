use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::material::Material;

use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub trait SceneObject: Debug {
    fn id(&self) -> i32;
    fn transformation(&self) -> Matrix;
    fn set_transformation(&mut self, transform: &Matrix);
    fn normal_at(&self, point: Tuple) -> Tuple;
    fn material(&self) -> Material;
    fn set_material(&mut self, material: &Material);
}

pub type MutSceneObject =  Arc<Mutex<Box<dyn SceneObject>>>;