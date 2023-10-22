use uuid::Uuid;
use crate::material::{Material, material};
use crate::shape::Shape;
use crate::matrix::{Matrix4, identity};
use crate::tuple::{point, Tuple};

#[derive(Debug, Clone)]
pub struct Sphere {
  pub id: String,
  pub transform: Matrix4,
  pub material: Material
}

// public static functions

pub fn sphere() -> Sphere {
  return Sphere {
    id: Uuid::new_v4().to_string(),
    transform: identity(),
    material: material()
  };
}

// trait impls

impl PartialEq for Sphere {
  fn eq(&self, other: &Self) -> bool {
      return self.get_transform() == other.get_transform()
      && self.get_material() == other.get_material()
  }
}

// object functions

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

  pub fn set_material(&mut self, m: &Material) {
    self.material = m.clone();
  }

  pub fn get_material(&self) -> &Material {
    &self.material
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