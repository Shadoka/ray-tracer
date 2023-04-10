use crate::tuple::{point, vector};
use crate::sphere::{Sphere};
use crate::intersection::{intersection};
use crate::matrix::{translation, scaling};

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
  let s = Sphere::shape();

  let mut expected = Vec::new();
  expected.push(intersection(4.0, &s));
  expected.push(intersection(6.0, &s));
  let result = r.intersect(&s);
  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0].intersection_t, expected[0].intersection_t);
  assert_eq!(result[1].intersection_t, expected[1].intersection_t);
  assert_eq!(result[0].object.get_id(), expected[0].object.get_id());
  // assert_eq!(&result.object, &expected.object);
}

#[test]
fn test_intersect_sphere_tangent() {
  let r = ray(&point(0.0, 1.0, -5.0), &vector(0.0, 0.0, 1.0));
  let s = Sphere::shape();

  let mut expected = Vec::new();
  expected.push(intersection(5.0, &s));
  expected.push(intersection(5.0, &s));
  let result = r.intersect(&s);
  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0].intersection_t, expected[0].intersection_t);
  assert_eq!(result[1].intersection_t, expected[1].intersection_t);
  assert_eq!(result[0].object.get_id(), expected[0].object.get_id());
}

#[test]
fn test_intersect_sphere_miss() {
  let r = ray(&point(0.0, 2.0, -5.0), &vector(0.0, 0.0, 1.0));
  let s = Sphere::shape();

  let expected: Vec<Intersection> = Vec::new();
  let result = r.intersect(&s);
  assert_eq!(result.len(), expected.len());
}

#[test]
fn test_intersect_sphere_inside() {
  let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
  let s = Sphere::shape();

  let mut expected = Vec::new();
  expected.push(intersection(-1.0, &s));
  expected.push(intersection(1.0, &s));
  let result = r.intersect(&s);
  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0].intersection_t, expected[0].intersection_t);
  assert_eq!(result[1].intersection_t, expected[1].intersection_t);
  assert_eq!(result[0].object.get_id(), expected[0].object.get_id());
}

#[test]
fn test_intersect_sphere_behind() {
  let r = ray(&point(0.0, 0.0, 5.0), &vector(0.0, 0.0, 1.0));
  let s = Sphere::shape();

  let mut expected = Vec::new();
  expected.push(intersection(-6.0, &s));
  expected.push(intersection(-4.0, &s));
  let result = r.intersect(&s);
  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0].intersection_t, expected[0].intersection_t);
  assert_eq!(result[1].intersection_t, expected[1].intersection_t);
  assert_eq!(result[0].object.get_id(), expected[0].object.get_id());
}

#[test]
fn test_translate_ray() {
  let r = ray(&point(1., 2., 3.), &vector(0., 1., 0.));
  let tm = translation(3.0, 4.0, 5.0);

  let expected = ray(&point(4., 6., 8.), &vector(0., 1., 0.));
  let result = r.transform(&tm);

  assert_eq!(result.origin, expected.origin);
  assert_eq!(result.direction, expected.direction);
}

#[test]
fn test_scale_ray() {
  let r = ray(&point(1., 2., 3.), &vector(0., 1., 0.));
  let tm = scaling(2., 3., 4.);

  let expected = ray(&point(2., 6., 12.), &vector(0., 3., 0.));
  let result = r.transform(&tm);

  assert_eq!(result.origin, expected.origin);
  assert_eq!(result.direction, expected.direction);
}

#[test]
fn test_scale_ray2() {
  let r = ray(&point(0., 0., -5.), &vector(0., 0., 1.));
  let tm = scaling(2., 2., 2.);

  let expected = ray(&point(0., 0., -10.), &vector(0., 0., 2.));
  let result = r.transform(&tm);

  assert_eq!(result.origin, expected.origin);
  assert_eq!(result.direction, expected.direction);
}

#[test]
fn test_intersect_scaled_sphere() {
  let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
  let mut s = Sphere::shape();
  s.set_transform(&scaling(2.0, 2.0, 2.0));

  let result = r.intersect(&s);
  let mut expected = Vec::new();
  expected.push(intersection(3.0, &s));
  expected.push(intersection(7.0, &s));

  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0].intersection_t, expected[0].intersection_t);
  assert_eq!(result[1].intersection_t, expected[1].intersection_t);
}

#[test]
fn test_intersect_translated_sphere() {
  let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
  let mut s = Sphere::shape();
  s.set_transform(&translation(0.0, 0.0, 2.0));

  let result = r.intersect(&s);
  let mut expected = Vec::new();
  expected.push(intersection(6.0, &s));
  expected.push(intersection(8.0, &s));

  assert_eq!(result.len(), expected.len());
  assert_eq!(result[0].intersection_t, expected[0].intersection_t);
  assert_eq!(result[1].intersection_t, expected[1].intersection_t);
}