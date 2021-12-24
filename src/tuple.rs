use std::ops;
use std::cmp::{PartialEq, Eq};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64
}

// public static methods

pub fn point(x: f64, y: f64, z:f64) -> Tuple {
  Tuple{x: x, y: y, z: z, w: 1.}
}

pub fn vector(x: f64, y: f64, z:f64) -> Tuple {
  Tuple{x: x, y: y, z: z, w: 0.}
}

pub fn color(r: f64, g: f64, b:f64) -> Tuple {
  Tuple{x: r, y: g, z: b, w: 0.}
}

// private static methods

fn equals_float(a: f64, b: f64) -> bool {
  return (a - b).abs() < 0.00001
}

// trait impls

impl ops::Add for Tuple {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      w: self.w + other.w
    }
  }
}

impl PartialEq for Tuple {
  fn eq(&self, other: &Self) -> bool {
    return equals_float(self.x, other.x)
      && equals_float(self.y, other.y)
      && equals_float(self.z, other.z)
      && self.w == other.w
  }
}

impl Eq for Tuple {}

impl ops::Sub for Tuple {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
      w: self.w - other.w
    }
  }
}

impl ops::Neg for Tuple {
  type Output = Self;

  fn neg(self) -> Self {
    return Tuple {x: 0.0, y: 0.0, z: 0.0, w: 0.} - self;
  }
}

impl ops::Mul<f64> for Tuple {
  type Output = Self;

  fn mul(self, scalar: f64) -> Tuple {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
      w: self.w * scalar
    }
  }
}

impl ops::Mul<Tuple> for Tuple {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
      w: 0.
    }
  }
}

impl ops::Div<f64> for Tuple {
  type Output = Self;

  fn div(self, scalar: f64) -> Self {
    Self {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
      w: self.w / scalar
    }
  }
}

impl ops::Div<f64> for &Tuple {
  type Output = Tuple;

  fn div(self, scalar: f64) -> Tuple {
    Tuple {
      x: self.x / scalar,
      y: self.y / scalar,
      z: self.z / scalar,
      w: self.w / scalar
    }
  }
}

// struct impl

impl Tuple {
  pub fn is_point(&self) -> bool {
    return self.w == 1.
  }

  pub fn is_vector(&self) -> bool {
    return self.w == 0.
  }

  pub fn magnitude(&self) -> f64 {
    let magnitude_powered = self.x * self.x
      + self.y * self.y
      + self.z * self.z
      + self.w * self.w;
    return magnitude_powered.sqrt();
  }

  pub fn normalize(&self) -> Tuple {
    return self / self.magnitude();
  }

  pub fn dot(&self, other: &Tuple) -> f64 {
    return self.x * other.x
      + self.y * other.y
      + self.z * other.z
      + self.w * other.w;
  }

  pub fn cross(&self, other: &Tuple) -> Tuple {
    return Tuple {
      x: self.y * other.z - self.z * other.y,
      y: self.z * other.x - self.x * other.z,
      z: self.x * other.y - self.y * other.x,
      w: 0.0
    };
  }
}

#[cfg(test)]
mod tuple_test;