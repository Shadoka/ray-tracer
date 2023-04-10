use crate::{matrix::translation, tuple::vector};

use super::*;

#[test]
fn test_create_sphere() {
  let s1 = sphere();
  let s2 = sphere();

  assert_ne!(s1.id, s2.id);
}

#[test]
fn test_default_transform() {
  let s = sphere();

  assert_eq!(s.get_transform(), &identity())
}

#[test]
fn test_set_transform() {
  let mut s = sphere();
  let tm = translation(2., 3., 4.);

  s.set_transform(&tm);

  assert_eq!(s.get_transform(), &tm);
}

#[test]
pub fn test_normal_x_axis() {
  let s = sphere();
  let expected = vector(1.0, 0.0, 0.0);
  let result = s.normal_at(&point(1.0, 0.0, 0.0));

  assert_eq!(result, expected);
}

#[test]
pub fn test_normal_y_axis() {
  let s = sphere();
  let expected = vector(0.0, 1.0, 0.0);
  let result = s.normal_at(&point(0.0, 1.0, 0.0));

  assert_eq!(result, expected);
}

#[test]
pub fn test_normal_z_axis() {
  let s = sphere();
  let expected = vector(0.0, 0.0, 1.0);
  let result = s.normal_at(&point(0.0, 0.0, 1.0));

  assert_eq!(result, expected);
}

#[test]
pub fn test_normal_nonaxial() {
  let s = sphere();
  let pos = 3.0_f64.sqrt() / 3.0;
  let expected = vector(pos, pos, pos);
  let result = s.normal_at(&point(pos, pos, pos));

  assert_eq!(result, expected);
}

#[test]
pub fn test_normal_is_normalized() {
  let s = sphere();
  let pos = 3.0_f64.sqrt() / 3.0;
  let result = s.normal_at(&point(pos, pos, pos));
  let expected = result.normalize();

  assert_eq!(result, expected);
}