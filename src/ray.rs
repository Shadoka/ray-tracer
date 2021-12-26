use crate::tuple::Tuple;

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
  
}