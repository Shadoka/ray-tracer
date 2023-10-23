use crate::{tuple::{point, vector}, matrix::{IDENTITY, scaling}};

use super::*;

#[test]
fn test_view_transform_default_orientation() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, -1.0);
    let up = vector(0.0, 1.0, 0.0);

    let t = view_transform(&from, &to, &up);

    assert_eq!(IDENTITY, t);
}

#[test]
fn test_view_transform_positive_z() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, 1.0);
    let up = vector(0.0, 1.0, 0.0);

    let t = view_transform(&from, &to, &up);
    let expected = scaling(-1.0, 1.0, -1.0);
    
    assert_eq!(expected, t);
}

#[test]
fn test_view_transform_move_world() {
    let from = point(0.0, 0.0, 8.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);

    let t = view_transform(&from, &to, &up);
    let expected = translation(0.0, 0.0, -8.0);

    assert_eq!(expected, t);
}

#[test]
fn test_view_transform_arbitrary() {
    let from = point(1.0, 3.0, 2.0);
    let to = point(4.0, -2.0, 8.0);
    let up = vector(1.0, 1.0, 0.0);

    let t = view_transform(&from, &to, &up);

    let expected_values = vec!(
        -0.50709, 0.50709, 0.67612, -2.36643,
        0.76772, 0.60609, 0.12122, -2.82843,
        -0.35857, 0.59761, -0.71714, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    let expected = matrix4(&expected_values);

    assert_eq!(expected, t);
}