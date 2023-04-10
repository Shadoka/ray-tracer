use uuid::Uuid;
use crate::shape::Shape;
use crate::matrix::{Matrix4, identity};

#[derive(Clone)]
pub struct Sphere {
  pub id: String,
  pub transform: Matrix4
}

// public static functions

pub fn sphere() -> Sphere {
  return Sphere{
    id: Uuid::new_v4().to_string(),
    transform: identity()
  };
}

impl Sphere {
  pub fn get_id(&self) -> &String {
    &self.id
  }

  pub fn get_transform(&self) -> &Matrix4 {
    &self.transform
  }

  pub fn set_transform(&mut self, tm: &Matrix4) {
    self.transform = tm.clone();
  }

  pub fn shape() -> Shape {
    Shape::Sphere(sphere())
  }
}

#[cfg(test)]
mod sphere_test;