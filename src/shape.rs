use crate::ray::Ray;
use crate::intersection::Intersection;

use std::cmp::{PartialEq};
use std::fmt::Formatter;
use std::fmt::Debug;

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Intersection;
    fn get_id(&self) -> &String;
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl Debug for dyn Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "shape_id: {}", self.get_id())
    }
}