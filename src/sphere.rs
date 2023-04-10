use uuid::Uuid;
use crate::shape::Shape;
use crate::matrix::{Matrix4, identity};
use crate::tuple::{point, Tuple};

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

  pub fn normal_at(&self, p: &Tuple) -> Tuple {
    let object_point = self.transform.inverse() * p;
    let object_normal = object_point - point(0.0, 0.0, 0.0);
    let mut world_normal = self.transform.inverse().transpose() * object_normal;
    world_normal.w = 0.0;
    return world_normal.normalize();
  }
}

#[cfg(test)]
mod sphere_test;