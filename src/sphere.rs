use uuid::Uuid;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::tuple::point;
use crate::intersection::{Intersection, intersection};

pub struct Sphere {
  pub id: String
}

// trait impls

impl Shape for Sphere {
  fn intersect(&self, ray: &Ray) -> Intersection {
    let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * (ray.direction.dot(&sphere_to_ray));
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    let mut interscts = Vec::new();
    if discriminant >= 0.0 {
      interscts.push((-b - discriminant.sqrt()) / (2.0 * a));
      interscts.push((-b + discriminant.sqrt()) / (2.0 * a));
    }

    intersection(interscts, self)
  }

  fn get_id(&self) -> &String {
    &self.id
  }
}

// public static functions

pub fn sphere() -> Sphere {
  return Sphere{id: Uuid::new_v4().to_string()};
}

impl Sphere {

}

#[cfg(test)]
mod sphere_test;