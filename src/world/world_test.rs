use crate::{ray::ray, tuple::vector, intersection::intersection, tuple::color};

use super::*;

#[test]
fn test_world() {
    let w = world();
    assert_eq!(0, w.objects.len());
    assert_eq!(None, w.light_source);
}

#[test]
fn test_default_world() {
    let w = default_world();

    let expected_light = point_light(
        &point(-10.0, 10.0, -10.0),
        &color(1.0, 1.0, 1.0)
    );

    let mut expected_s1 = Sphere::shape();
    let mut m1 = material();
    m1.color = color(0.8, 1.0, 0.6);
    m1.diffuse = 0.7;
    m1.specular = 0.2;
    expected_s1.set_material(&m1);

    let mut expected_s2 = Sphere::shape();
    let scaling_s2 = scaling(0.5, 0.5, 0.5);
    expected_s2.set_transform(&scaling_s2);

    let actual_objects = w.get_objects();

    assert_eq!(2, actual_objects.len());
    assert_eq!(&expected_s1, actual_objects.get(0).unwrap());
    assert_eq!(&expected_s2, actual_objects.get(1).unwrap());
    assert_eq!(expected_light, w.get_light().unwrap())
}

#[test]
fn test_intersect_world() {
    let mut w = default_world();
    let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let xs = w.intersect(&r);

    assert_eq!(4, xs.count());
    assert_eq!(4.0, xs[0].intersection_t);
    assert_eq!(4.5, xs[1].intersection_t);
    assert_eq!(5.5, xs[2].intersection_t);
    assert_eq!(6.0, xs[3].intersection_t);
}

#[test]
fn test_shade_hit() {
    let w = default_world();
    let ray = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
    let world_objects = w.get_objects();
    let shape = world_objects[0].clone();
    let i = intersection(4.0, &shape);

    let comp_data = i.prepare_computations(&ray);
    let actual_color = w.shade_hit(&comp_data);
    let expected_color = color(0.38066, 0.47583, 0.2855);

    assert_eq!(expected_color, actual_color);
}

#[test]
fn test_shade_hit_inside() {
    let mut w = default_world();
    let light = point_light(&point(0.0, 0.25, 0.0), &color(1.0, 1.0, 1.0));
    w.light_source = Some(light);

    let ray = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
    let world_objects = w.get_objects();
    let shape = world_objects[1].clone();
    let i = intersection(0.5, &shape);

    let comp_data = i.prepare_computations(&ray);
    let actual_color = w.shade_hit(&comp_data);
    let expected_color = color(0.90498, 0.90498, 0.90498);

    assert_eq!(expected_color, actual_color);
}