use crate::intersection::{Intersection, intersection};
use crate::shape::Shape;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, point};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Tuple,
  pub direction: Tuple
}

// public static functions

pub fn ray(origin: &Tuple, direction: &Tuple) -> Ray {
  return Ray{origin: origin.clone(), direction: direction.clone()};
}

impl Ray {
  pub fn position(&self, t: f64) -> Tuple {
    return self.origin + self.direction * t;
  }

  pub fn intersect<'a>(&self, shape: &'a Shape) -> Vec<Intersection<'a>> {
    match shape {
      Shape::Sphere(s) => self.intersect_sphere(s, shape)
    }
  }

  fn intersect_sphere<'a>(&self, sphere: &Sphere, shape: &'a Shape) -> Vec<Intersection<'a>> {
    let sphere_to_ray = self.origin - point(0.0, 0.0, 0.0);
    let a = self.direction.dot(&self.direction);
    let b = 2.0 * (self.direction.dot(&sphere_to_ray));
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    let mut interscts = Vec::new();
    if discriminant >= 0.0 {
      let first_intersection = (-b - discriminant.sqrt()) / (2.0 * a);
      let second_intersection = (-b + discriminant.sqrt()) / (2.0 * a);
      interscts.push(intersection(first_intersection, shape));
      interscts.push(intersection(second_intersection, shape));
    }

    interscts
  }
}

#[cfg(test)]
mod ray_test;