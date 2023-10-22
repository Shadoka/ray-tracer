use super::*;
use crate::{sphere::Sphere, ray::ray, tuple::{point, vector}};

#[test]
fn test_intersection_add() {
    let s = Sphere::shape();
    let i1 = intersection(0.0, &s);
    let i2 = intersection(0.0, &s);

    let i3 = intersection(0.0, &s);
    let i4 = intersection(0.0, &s);
    let v = vec!(i3, i4);
    let expected = Intersections{values: v};

    let result = i1 + i2;

    assert_eq!(result.values.len(), expected.values.len());
}

#[test]
fn test_intersection_index_access() {
    let s = Sphere::shape();
    let i1 = intersection(0.0, &s);
    let i2 = intersection(1.0, &s);

    let result = i1 + i2;

    assert_eq!(result[0].intersection_t, 0.0);
    assert_eq!(result[0].intersection_t, 0.0);
    assert_eq!(result[1].intersection_t, 1.0);
}

#[test]
fn test_add_intersection_to_intersections() {
    let s = Sphere::shape();
    let i1 = intersection(0.0, &s);
    let i2 = intersection(1.0, &s);
    let i3 = intersection(2.0, &s);

    let temp = i1 + i2;
    let result = temp + i3;

    assert_eq!(result[0].intersection_t, 0.0);
    assert_eq!(result[1].intersection_t, 1.0);
    assert_eq!(result[2].intersection_t, 2.0);
}

#[test]
fn test_intersections_hit1() {
    let s = Sphere::shape();
    let i1 = intersection(1.0, &s);
    let i2 = intersection(2.0, &s);
    let obj_id = s.get_id();
    let xs = i1 + i2;

    let result = xs.hit();

    match result {
        None => assert_eq!(true, false),
        Some(hit) => {
            assert_eq!(hit.intersection_t, 1.0);
            assert_eq!(hit.object.get_id(), obj_id);
        }
    }
}

#[test]
fn test_intersections_hit2() {
    let s = Sphere::shape();
    let i1 = intersection(-1.0, &s);
    let i2 = intersection(1.0, &s);
    let obj_id = s.get_id();
    let xs = i1 + i2;

    let result = xs.hit();

    match result {
        None => assert_eq!(true, false),
        Some(hit) => {
            assert_eq!(hit.intersection_t, 1.0);
            assert_eq!(hit.object.get_id(), obj_id);
        }
    }
}

#[test]
fn test_intersections_hit3() {
    let s = Sphere::shape();
    let i1 = intersection(-2.0, &s);
    let i2 = intersection(-1.0, &s);
    let xs = i1 + i2;

    let result = xs.hit();

    match result {
        None => (),
        Some(_hit) => assert_eq!(true, false)
    }
}

#[test]
fn test_intersections_hit4() {
    let s = Sphere::shape();
    let i1 = intersection(5.0, &s);
    let i2 = intersection(7.0, &s);
    let i3 = intersection(-3.0, &s);
    let i4 = intersection(2.0, &s);
    let obj_id = s.get_id();
    let xs = i1 + i2 + i3 + i4;

    let result = xs.hit();

    match result {
        None => assert_eq!(true, false),
        Some(hit) => {
            assert_eq!(hit.intersection_t, 2.0);
            assert_eq!(hit.object.get_id(), obj_id);
        }
    }
}

#[test]
fn test_prepare_computations() {
    let ray = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let shape = Sphere::shape();
    let i = intersection(4.0, &shape);
    let comp_data = i.prepare_computations(&ray);

    assert_eq!(comp_data.t, i.intersection_t);
    assert_eq!(comp_data.object, i.object.clone());
    assert_eq!(comp_data.point, point(0.0, 0.0, -1.0));
    assert_eq!(comp_data.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(comp_data.normalv, vector(0.0, 0.0, -1.0));
}