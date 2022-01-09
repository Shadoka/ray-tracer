use uuid::Uuid;
use crate::shape::Shape;

#[derive(Clone)]
pub struct Sphere {
  pub id: String
}

// public static functions

pub fn sphere() -> Sphere {
  return Sphere{id: Uuid::new_v4().to_string()};
}

impl Sphere {
  pub fn get_id(&self) -> &String {
    &self.id
  }

  pub fn shape() -> Shape {
    Shape::Sphere(sphere())
  }
}

#[cfg(test)]
mod sphere_test;