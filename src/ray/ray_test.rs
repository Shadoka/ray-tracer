use crate::tuple::{point, vector};
use crate::sphere::sphere;

use super::*;

#[test]
fn test_ray() {
  let origin = point(1.0, 2.0, 3.0);
  let direction = vector(4.0, 5.0, 6.0);
  let r = ray(&origin, &direction);

  assert_eq!(r.origin, origin);
  assert_eq!(r.direction, direction);
}

#[test]
fn test_position() {
  let origin = point(2.0, 3.0, 4.0);
  let direction = vector(1.0, 0.0, 0.0);
  let r = ray(&origin, &direction);

  let exp1 = point(2.0, 3.0, 4.0);
  let exp2 = point(3.0, 3.0, 4.0);
  let exp3 = point(1.0, 3.0, 4.0);
  let exp4 = point(4.5, 3.0, 4.0);

  assert_eq!(r.position(0.0), exp1);
  assert_eq!(r.position(1.0), exp2);
  assert_eq!(r.position(-1.0), exp3);
  assert_eq!(r.position(2.5), exp4);
}

#[test]
fn test_intersect_sphere_twice() {
  let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
  let s = sphere();

  let expected = vec!(4.0, 6.0);

  let result = r.intersect(&s);
  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0], expected[0]);
  assert_eq!(result[1], expected[1]);
}