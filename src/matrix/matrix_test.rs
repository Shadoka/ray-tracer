use crate::tuple;

use super::*;
use std::f64::consts::PI;

#[test]
fn test_create_matrix4() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    5.5, 6.5, 7.5, 8.5,
    9.0, 10.0, 11.0, 12.0,
    13.5, 14.5, 15.5, 16.5
  );
  let m = matrix4(&values);
  
  assert_eq!(m.values[0][0], 1.0);
  assert_eq!(m.values[0][3], 4.0);
  assert_eq!(m.values[1][0], 5.5);
  assert_eq!(m.values[1][2], 7.5);
  assert_eq!(m.values[2][2], 11.0);
  assert_eq!(m.values[3][0], 13.5);
  assert_eq!(m.values[3][2], 15.5);
}

#[test]
fn test_create_matrix2() {
  let values = vec!(-3.0, 5.0,
    1.0, -2.0
  );
  let m = matrix2(&values);

  assert_eq!(m.values[0][0], -3.0);
  assert_eq!(m.values[0][1], 5.0);
  assert_eq!(m.values[1][0], 1.0);
  assert_eq!(m.values[1][1], -2.0);
}

#[test]
fn test_create_matrix3() {
  let values = vec!(-3.0, 5.0, 0.0,
    1.0, -2.0, -7.0,
    0.0, 1.0, 1.0
  );
  let m = matrix3(&values);

  assert_eq!(m.values[0][0], -3.0);
  assert_eq!(m.values[1][1], -2.0);
  assert_eq!(m.values[2][2], 1.0);
}

#[test]
fn test_matrix_equality1() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    5.5, 6.5, 7.5, 8.5,
    9.0, 10.0, 11.0, 12.0,
    13.5, 14.5, 15.5, 16.5
  );
  let m1 = matrix4(&values);
  let m2 = matrix4(&values);

  assert_eq!(m1, m2);
}

#[test]
fn test_matrix_equality2() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    5.5, 6.5, 7.5, 8.5,
    9.0, 10.0, 11.0, 12.0,
    13.5, 14.5, 15.5, 16.5
  );
  let m1 = matrix4(&values);
  let values2 = vec!(1.0, 2.0, 3.0, 4.0,
    5.5, 6.5, 7.5, 8.5,
    9.0, 10.0, 2.0, 12.0,
    13.5, 14.5, 15.5, 16.5
  );
  let m2 = matrix4(&values2);

  assert_eq!(m1 == m2, false);
}

#[test]
fn test_matrix_mul() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    5.0, 6.0, 7.0, 8.0,
    9.0, 8.0, 7.0, 6.0,
    5.0, 4.0, 3.0, 2.0
  );
  let m1 = matrix4(&values);
  let values2 = vec!(-2.0, 1.0, 2.0, 3.0,
    3.0, 2.0, 1.0, -1.0,
    4.0, 3.0, 6.0, 5.0,
    1.0, 2.0, 7.0, 8.0
  );
  let m2 = matrix4(&values2);

  let expected_values = vec!(20.0, 22.0, 50.0, 48.0,
    44.0, 54.0, 114.0, 108.0,
    40.0, 58.0, 110.0, 102.0,
    16.0, 26.0, 46.0, 42.0
  );
  let expected = matrix4(&expected_values);

  assert_eq!(m1 * m2, expected);
}

#[test]
fn test_matrix4_tuple_mul() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    2.0, 4.0, 4.0, 2.0,
    8.0, 6.0, 4.0, 1.0,
    0.0, 0.0, 0.0, 1.0
  );
  let m = matrix4(&values);

  let t = Tuple{x: 1.0, y: 2.0, z: 3.0, w: 1.0};

  let expected = Tuple{x: 18.0, y: 24.0, z: 33.0, w: 1.0};

  assert_eq!(m * t, expected);
}

#[test]
fn test_matrix_identity_mul() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    2.0, 4.0, 4.0, 2.0,
    8.0, 6.0, 4.0, 1.0,
    0.0, 0.0, 0.0, 1.0
  );
  let m = matrix4(&values);

  assert_eq!(&m * &IDENTITY, m);
}

#[test]
fn test_matrix4_transpose() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    2.0, 4.0, 4.0, 2.0,
    8.0, 6.0, 4.0, 1.0,
    0.0, 0.0, 0.0, 1.0
  );
  let m = matrix4(&values);

  let values_expected = vec!(1.0, 2.0, 8.0, 0.0,
    2.0, 4.0, 6.0, 0.0,
    3.0, 4.0, 4.0, 0.0,
    4.0, 2.0, 1.0, 1.0
  );
  let expected = matrix4(&values_expected);

  assert_eq!(m.transpose(), expected);
}

#[test]
fn test_identity_transpose() {
  assert_eq!(IDENTITY.transpose(), IDENTITY);
}

#[test]
fn test_matrix2_determinant() {
  let values = vec!(1.0, 5.0, -3.0, 2.0);
  let m = matrix2(&values);

  assert_eq!(m.det(), 17.0);
}

#[test]
fn test_matrix4_submatrix() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    2.0, 4.0, 4.0, 2.0,
    8.0, 6.0, 4.0, 1.0,
    0.0, 0.0, 0.0, 1.0
  );
  let m = matrix4(&values);

  let expected_values = vec!(1.0, 3.0, 4.0,
    2.0, 4.0, 2.0,
    0.0, 0.0, 1.0
  );
  let expected = matrix3(&expected_values);

  assert_eq!(m.submatrix(2, 1), expected);
}

#[test]
fn test_matrix3_submatrix() {
  let values = vec!(1.0, 3.0, 4.0,
    2.0, 4.0, 2.0,
    0.0, 0.0, 1.0
  );
  let m = matrix3(&values);

  let expected_values = vec!(4.0, 2.0,
    0.0, 1.0
  );
  let expected = matrix2(&expected_values);

  assert_eq!(m.submatrix(0, 0), expected);
}

#[test]
fn test_matrix3_minor() {
  let values = vec!(
    3.0, 5.0, 0.0,
    2.0, -1.0, -7.0,
    6.0, -1.0, 5.0
  );
  let m = matrix3(&values);

  assert_eq!(m.minor(1, 0), 25.0);
}

#[test]
fn test_matrix3_cofactor() {
  let values = vec!(3.0, 5.0, 0.0,
    2.0, -1.0, -7.0,
    6.0, -1.0, 5.0
  );
  let m = matrix3(&values);

  assert_eq!(m.cofactor(0, 0), -12.0);
  assert_eq!(m.cofactor(1, 0), -25.0);
}

#[test]
fn test_matrix3_determinant() {
  let values = vec!(
    1.0, 2.0, 6.0,
    -5.0, 8.0, -4.0,
    2.0, 6.0, 4.0
  );
  let m = matrix3(&values);

  assert_eq!(m.cofactor(0, 0), 56.0);
  assert_eq!(m.cofactor(0, 1), 12.0);
  assert_eq!(m.cofactor(0, 2), -46.0);
  assert_eq!(m.det(), -196.0);
}

#[test]
fn test_matrix4_cofactor() {
  let values = vec!(
    -2.0, -8.0, 3.0, 5.0,
    -3.0, 1.0, 7.0, 3.0,
    1.0, 2.0, -9.0, 6.0,
    -6.0, 7.0, 7.0, -9.0
  );
  let m = matrix4(&values);

  assert_eq!(m.cofactor(0, 0), 690.0);
  assert_eq!(m.cofactor(0, 1), 447.0);
  assert_eq!(m.cofactor(0, 2), 210.0);
  assert_eq!(m.cofactor(0, 3), 51.0);
}

#[test]
fn test_matrix4_determinant() {
  let values = vec!(
    -2.0, -8.0, 3.0, 5.0,
    -3.0, 1.0, 7.0, 3.0,
    1.0, 2.0, -9.0, 6.0,
    -6.0, 7.0, 7.0, -9.0
  );
  let m = matrix4(&values);

  assert_eq!(m.det(), -4071.0);
}

#[test]
fn test_matrix4_invertible() {
  let values = vec!(
    -2.0, -8.0, 3.0, 5.0,
    -3.0, 1.0, 7.0, 3.0,
    1.0, 2.0, -9.0, 6.0,
    -6.0, 7.0, 7.0, -9.0
  );
  let m = matrix4(&values);

  assert!(m.is_invertible());
}

#[test]
fn test_matrix4_noninvertible() {
  let values = vec!(
    -4.0, 2.0, -2.0, -3.0,
    9.0, 6.0, 2.0, 6.0,
    0.0, -5.0, 1.0, -5.0,
    0.0, 0.0, 0.0, 0.0
  );
  let m = matrix4(&values);

  assert_eq!(m.is_invertible(), false);
}

#[test]
fn test_matrix4_invert1() {
  let values = vec!(
    -5.0, 2.0, 6.0, -8.0,
    1.0, -5.0, 1.0, 8.0,
    7.0, 7.0, -6.0, -7.0,
    1.0, -3.0, 7.0, 4.0
  );
  let m = matrix4(&values);

  let expected_values = vec!(
    0.21805, 0.45113, 0.24060, -0.04511,
    -0.80827, -1.45677, -0.44361, 0.52068,
    -0.07895, -0.22368, -0.05263, 0.19737,
    -0.52256, -0.81391, -0.30075, 0.30639
  );
  let expected = matrix4(&expected_values);

  assert_eq!(m.det(), 532.0);
  assert_eq!(m.cofactor(2, 3), -160.0);
  assert_eq!(m.cofactor(3, 2), 105.0);
  assert_eq!(equals_float(expected.values[3][2], -160.0/532.0), true);
  assert_eq!(equals_float(expected.values[2][3], 105.0/532.0), true);
  assert_eq!(m.inverse(), expected);
}

#[test]
fn test_matrix4_invert2() {
  let values = vec!(
    8.0, -5.0, 9.0, 2.0,
    7.0, 5.0, 6.0, 1.0,
    -6.0, 0.0, 9.0, 6.0,
    -3.0, 0.0, -9.0, -4.0
  );
  let m = matrix4(&values);

  let expected_values = vec!(
    -0.15385, -0.15385, -0.28205, -0.53846,
    -0.07692, 0.12308, 0.02564, 0.03077,
    0.35897, 0.35897, 0.43590, 0.92308,
    -0.69231, -0.69231, -0.76923, -1.92308
  );
  let expected = matrix4(&expected_values);

  assert_eq!(m.inverse(),expected);
}

#[test]
fn test_matrix4_invert3() {
  let values = vec!(
    9.0, 3.0, 0.0, 9.0,
    -5.0, -2.0, -6.0, -3.0,
    -4.0, 9.0, 6.0, 4.0,
    -7.0, 6.0, 6.0, 2.0
  );
  let m = matrix4(&values);

  let expected_values = vec!(
    -0.04074, -0.07778, 0.14444, -0.22222,
    -0.07778, 0.03333, 0.36667, -0.33333,
    -0.02901, -0.14630, -0.10926, 0.12963,
    0.17778, 0.06667, -0.26667, 0.33333
  );
  let expected = matrix4(&expected_values);

  assert_eq!(m.inverse(),expected);
}

#[test]
fn test_matrix4_inverse_mul() {
  let values = vec!(
    3.0, -9.0, 7.0, 3.0,
    3.0, -8.0, 2.0, -9.0,
    -4.0, 4.0, 4.0, 1.0,
    -6.0, 5.0, -1.0, 1.0
  );
  let m = matrix4(&values);

  let values2 = vec!(
    8.0, 2.0, 2.0, 2.0,
    3.0, -1.0, 7.0, 1.0,
    7.0, 0.0, 5.0, 4.0,
    6.0, -2.0, 0.0, 5.0
  );
  let m2 = matrix4(&values2);

  let m3 = &m * &m2;

  assert_eq!(m3 * m2.inverse(), m);
}

#[test]
fn test_translate() {
  let p = tuple::point(-3.0, 4.0, 5.0);
  let tm = translation(5.0, -3.0, 2.0);

  let expected = tuple::point(2.0, 1.0, 7.0);

  assert_eq!(tm * p, expected);
}

#[test]
fn test_translate_inverse() {
  let p = tuple::point(-3.0, 4.0, 5.0);
  let tm = translation(5.0, -3.0, 2.0);
  let inv = tm.inverse();

  let expected = tuple::point(-8.0, 7.0, 3.0);

  assert_eq!(inv * p, expected);
}

#[test]
fn test_translate_vector() {
  let tm = translation(5.0, -3.0, 2.0);
  let v = tuple::vector(-3.0, 4.0, 5.0);

  assert_eq!(tm * v, v);
}

#[test]
fn test_scale_point() {
  let sm = scaling(2.0, 3.0, 4.0);
  let p = tuple::point(-4.0, 6.0, 8.0);

  let expected = tuple::point(-8.0, 18.0, 32.0);

  assert_eq!(sm * p, expected);
}

#[test]
fn test_scale_vector() {
  let sm = scaling(2.0, 3.0, 4.0);
  let p = tuple::vector(-4.0, 6.0, 8.0);

  let expected = tuple::vector(-8.0, 18.0, 32.0);

  assert_eq!(sm * p, expected);
}

#[test]
fn test_scale_inverse() {
  let sm = scaling(2.0, 3.0, 4.0);
  let p = tuple::vector(-4.0, 6.0, 8.0);
  let inv = sm.inverse();

  let expected = tuple::vector(-2.0, 2.0, 2.0);

  assert_eq!(inv * p, expected);
}

#[test]
fn test_reflect_x_axis() {
  let sm = scaling(-1.0, 1.0, 1.0);
  let p = tuple::point(2.0, 3.0, 4.0);

  let expected = tuple::point(-2.0, 3.0, 4.0);

  assert_eq!(sm * p, expected);
}

#[test]
fn test_rotation_x() {
  let p = tuple::point(0.0, 1.0, 0.0);
  let half_quarter = rotation_x(PI / 4.0);
  let full_quarter = rotation_x(PI / 2.0);

  let expected_hq = tuple::point(0.0, (2.0f64).sqrt() / 2.0, (2.0f64).sqrt() / 2.0);
  let expected_fq = tuple::point(0.0, 0.0, 1.0);

  assert_eq!(half_quarter * p, expected_hq);
  assert_eq!(full_quarter * p, expected_fq);
}

#[test]
fn test_rotation_x_inverse() {
  let p = tuple::point(0.0, 1.0, 0.0);
  let half_quarter = rotation_x(PI / 4.0);
  let inverse_hq = half_quarter.inverse();

  let expected_hq = tuple::point(0.0, (2.0f64).sqrt() / 2.0, -((2.0f64).sqrt() / 2.0));

  assert_eq!(inverse_hq * p, expected_hq);
}

#[test]
fn test_rotation_y() {
  let p = tuple::point(0.0, 0.0, 1.0);
  let half_quarter = rotation_y(PI / 4.0);
  let full_quarter = rotation_y(PI / 2.0);

  let expected_hq = tuple::point((2.0f64).sqrt() / 2.0, 0.0, (2.0f64).sqrt() / 2.0);
  let expected_fq = tuple::point(1.0, 0.0, 0.0);

  assert_eq!(half_quarter * p, expected_hq);
  assert_eq!(full_quarter * p, expected_fq);
}

#[test]
fn test_rotation_z() {
  let p = tuple::point(0.0, 1.0, 0.0);
  let half_quarter = rotation_z(PI / 4.0);
  let full_quarter = rotation_z(PI / 2.0);

  let expected_hq = tuple::point(-((2.0f64).sqrt() / 2.0), (2.0f64).sqrt() / 2.0, 0.0);
  let expected_fq = tuple::point(-1.0, 0.0, 0.0);

  assert_eq!(half_quarter * p, expected_hq);
  assert_eq!(full_quarter * p, expected_fq);
}

#[test]
fn test_shear_xy() {
  let p = tuple::point(2.0, 3.0, 4.0);
  let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

  let expected = tuple::point(5.0, 3.0, 4.0);

  assert_eq!(transform * p, expected);
}

#[test]
fn test_shear_zy() {
  let p = tuple::point(2.0, 3.0, 4.0);
  let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

  let expected = tuple::point(2.0, 3.0, 7.0);

  assert_eq!(transform * p, expected);
}

#[test]
fn test_shear_xz() {
  let p = tuple::point(2.0, 3.0, 4.0);
  let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);

  let expected = tuple::point(6.0, 3.0, 4.0);

  assert_eq!(transform * p, expected);
}

#[test]
fn test_shear_yx() {
  let p = tuple::point(2.0, 3.0, 4.0);
  let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);

  let expected = tuple::point(2.0, 5.0, 4.0);

  assert_eq!(transform * p, expected);
}

#[test]
fn test_shear_yz() {
  let p = tuple::point(2.0, 3.0, 4.0);
  let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);

  let expected = tuple::point(2.0, 7.0, 4.0);

  assert_eq!(transform * p, expected);
}

#[test]
fn test_shear_zx() {
  let p = tuple::point(2.0, 3.0, 4.0);
  let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);

  let expected = tuple::point(2.0, 3.0, 6.0);

  assert_eq!(transform * p, expected);
}

#[test]
fn test_transformation_sequence() {
  let p = tuple::point(1.0, 0.0, 1.0);
  let rot = rotation_x(PI / 2.0);
  let scaling = scaling(5.0, 5.0, 5.0);
  let translation = translation(10.0, 5.0, 7.0);

  let exp1 = tuple::point(1.0, -1.0, 0.0);
  let exp2 = tuple::point(5.0, -5.0, 0.0);
  let exp3 = tuple::point(15.0, 0.0, 7.0);

  let result_rot = rot * p;
  assert_eq!(result_rot, exp1);

  let result_scaling = scaling * result_rot;
  assert_eq!(result_scaling, exp2);

  let result_translate = translation * result_scaling;
  assert_eq!(result_translate, exp3);
}

#[test]
fn test_transformation_chaining() {
  let p = tuple::point(1.0, 0.0, 1.0);
  let transformation = rotation_x(PI / 2.0)
    .scale(5.0, 5.0, 5.0)
    .translate(10.0, 5.0, 7.0);

  let expected = tuple::point(15.0, 0.0, 7.0);

  assert_eq!(transformation * p, expected);
}

fn equals_float(a: f64, b: f64) -> bool {
  return (a - b).abs() < 0.00001
} 