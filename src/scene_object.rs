use std::fmt::Debug;

use crate::matrix::Matrix;

pub trait SceneObject: Debug {
    fn id(&self) -> i32;
    fn transformation(&self) -> Matrix;
    fn set_transformation(&mut self, transform: &Matrix);
}