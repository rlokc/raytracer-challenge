use std::{fmt::Debug, sync::{Arc, Mutex}, ops::DerefMut};

use crate::matrix::Matrix;

pub trait SceneObject: Debug {
    fn id(&self) -> i32;
    fn transformation(&self) -> Arc<Mutex<Matrix>>;
    fn set_transformation(&mut self, transform: Arc<Mutex<Matrix>>);
}