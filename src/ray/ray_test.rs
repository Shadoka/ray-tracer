use crate::tuple::{point, vector};
use crate::sphere::{Sphere, sphere};
use crate::intersection::{intersection};
use crate::shape::Shape;

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

  let expected = intersection(vec!(4.0, 6.0), &s);
  let result = s.intersect(&r);
  assert_eq!(result.intersections.len(), expected.intersections.len());
  assert_eq!(result.intersections[0], expected.intersections[0]);
  assert_eq!(result.intersections[1], expected.intersections[1]);
  assert_eq!(result.object.get_id(), expected.object.get_id());
  // assert_eq!(&result.object, &expected.object);
}