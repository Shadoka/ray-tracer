use uuid::Uuid;

pub struct Sphere {
  pub id: String
}

// public static functions

pub fn sphere() -> Sphere {
  return Sphere{id: Uuid::new_v4().to_string()};
}

impl Sphere {

}

#[cfg(test)]
mod sphere_test;