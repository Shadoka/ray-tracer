use crate::{material::material, tuple::{point, vector}, lights::point_light};

use super::*;

#[test]
pub fn test_between_light_and_wall() {
    let m = material();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0));

    let result = lighting(&m, &light, &p, &eyev, &normalv);
    let expected = color(1.9, 1.9, 1.9);

    assert_eq!(result, expected);
}

#[test]
pub fn test_45_degree_angle() {
    let m = material();
    let p = point(0.0, 0.0, 0.0);

    let angled_dir = 2.0_f64.sqrt() / 2.0;
    let eyev = vector(0.0, angled_dir, -angled_dir);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(&point(0.0, 0.0, -10.0), &color(1.0, 1.0, 1.0));

    let result = lighting(&m, &light, &p, &eyev, &normalv);
    let expected = color(1.0, 1.0, 1.0);

    assert_eq!(result, expected);
}

#[test]
pub fn test_45_degree_angle_light() {
    let m = material();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(&point(0.0, 10.0, -10.0), &color(1.0, 1.0, 1.0));

    let result = lighting(&m, &light, &p, &eyev, &normalv);
    let expected = color(0.7364, 0.7364, 0.7364);

    assert_eq!(result, expected);
}

#[test]
pub fn test_90_degree_angle() {
    let m = material();
    let p = point(0.0, 0.0, 0.0);

    let angled_dir = 2.0_f64.sqrt() / 2.0;
    let eyev = vector(0.0, -angled_dir, -angled_dir);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(&point(0.0, 10.0, -10.0), &color(1.0, 1.0, 1.0));

    let result = lighting(&m, &light, &p, &eyev, &normalv);
    let expected = color(1.6364, 1.6364, 1.6364);

    assert_eq!(result, expected);
}

#[test]
pub fn test_light_behind_point() {
    let m = material();
    let p = point(0.0, 0.0, 0.0);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(&point(0.0, 0.0, 10.0), &color(1.0, 1.0, 1.0));

    let result = lighting(&m, &light, &p, &eyev, &normalv);
    let expected = color(0.1, 0.1, 0.1);

    assert_eq!(result, expected);
}