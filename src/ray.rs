use crate::tuple::{Tuple, point};
use crate::sphere::Sphere;

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

  pub fn intersect(&self, sphere: &Sphere) -> Vec<f64> {
    let sphere_to_ray = self.origin - point(0.0, 0.0, 0.0);
    let a = self.direction.dot(&self.direction);
    let b = 2.0 * (self.direction.dot(&sphere_to_ray));
    let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
      return Vec::new();
    } else {
      let mut intersections: Vec<f64> = Vec::new();
      intersections.push((-b - discriminant.sqrt()) / (2.0 * a));
      intersections.push((-b + discriminant.sqrt()) / (2.0 * a));
      return intersections;
    }
  }
}

#[cfg(test)]
mod ray_test;