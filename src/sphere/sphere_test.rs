use crate::matrix::translation;

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