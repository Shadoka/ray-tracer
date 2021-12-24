use super::*;

#[test]
fn test_point_tuple() {
  let t = Tuple{x: 4.3, y: -4.2, z: 3.1, w: 1.};
  assert_eq!(t.x, 4.3);
  assert_eq!(t.y, -4.2);
  assert_eq!(t.z, 3.1);
  assert_eq!(t.w, 1.);
  assert!(t.is_point());
  assert!(!t.is_vector());
}

#[test]
fn test_vector_tuple() {
  let t = Tuple{x: 4.3, y: -4.2, z: 3.1, w: 0.};
  assert_eq!(t.x, 4.3);
  assert_eq!(t.y, -4.2);
  assert_eq!(t.z, 3.1);
  assert_eq!(t.w, 0.);
  assert!(!t.is_point());
  assert!(t.is_vector());
}

#[test]
fn test_create_point() {
  let point = point(4.0, -4.0, 3.0);
  assert!(point == Tuple{x: 4.0, y: -4.0, z: 3.0, w: 1.});
}

#[test]
fn test_create_vector() {
  let vector = vector(4.0, -4.0, 3.0);
  assert!(vector == Tuple{x: 4.0, y: -4.0, z: 3.0, w: 0.});
}

#[test]
fn test_add_vectors() {
  let vec1 = vector(1.0, 2.0, 3.0);
  let vec2 = vector(4.0, 5.0, 6.0);
  assert_eq!(vec1 + vec2, vector(5.0, 7.0, 9.0));
}

#[test]
fn test_add_vector_point() {
  let vec1 = vector(1.0, 2.0, 3.0);
  let vec2 = point(4.0, 5.0, 6.0);
  assert_eq!(vec1 + vec2, point(5.0, 7.0, 9.0));
}

#[test]
fn test_add_point_vector() {
  let vec1 = point(1.0, 2.0, 3.0);
  let vec2 = vector(4.0, 5.0, 6.0);
  assert_eq!(vec1 + vec2, point(5.0, 7.0, 9.0));
}

#[test]
fn test_add_point_point() {
  let vec1 = point(1.0, 2.0, 3.0);
  let vec2 = point(4.0, 5.0, 6.0);
  let result = vec1 + vec2;
  assert_eq!(result, Tuple{x: 5.0, y: 7.0, z: 9.0, w: 2.});
  assert!(!result.is_point());
  assert!(!result.is_vector());
}

#[test]
fn test_sub_vector_vector() {
  let vec1 = vector(1.0, 2.0, 3.0);
  let vec2 = vector(4.0, 5.0, 6.0);
  assert_eq!(vec1 - vec2, vector(-3.0, -3.0, -3.0));
}

#[test]
fn test_sub_point_point() {
  let vec1 = point(1.0, 2.0, 3.0);
  let vec2 = point(4.0, 5.0, 6.0);
  assert_eq!(vec1 - vec2, vector(-3.0, -3.0, -3.0));
}

#[test]
fn test_sub_point_vector() {
  let vec1 = point(1.0, 2.0, 3.0);
  let vec2 = vector(4.0, 5.0, 6.0);
  assert_eq!(vec1 - vec2, point(-3.0, -3.0, -3.0));
}

#[test]
fn test_sub_vector_point() {
  let vec1 = vector(1.0, 2.0, 3.0);
  let vec2 = point(4.0, 5.0, 6.0);
  let result = vec1 - vec2;
  assert_eq!(result, Tuple{x: -3.0, y: -3.0, z: -3.0, w: -1.});
  assert!(!result.is_point());
  assert!(!result.is_vector())
}

#[test]
fn test_negate_vector() {
  let vector = vector(1.0, -2.0, 3.0);
  assert_eq!(-vector, Tuple{x: -1.0, y: 2.0, z: -3.0, w: 0.});
}

#[test]
fn test_negate_point() {
  let point = point(1.0, -2.0, 3.0);
  let result = -point;
  assert_eq!(result, Tuple{x: -1.0, y: 2.0, z: -3.0, w: -1.});
  assert!(!result.is_point());
  assert!(!result.is_vector());
}

#[test]
fn test_mul_vector_scalar() {
  let v = vector(10., -15., 7.3);
  assert_eq!(v * 3.5, vector(35., -52.5, 25.55));
}

#[test]
fn test_mul_vector_scalar2() {
  let v = vector(10., -15., 7.3);
  assert_eq!(v * 0.5, vector(5.0, -7.5, 3.65));
}

#[test]
fn test_mul_tuple_scalar() {
  let t = Tuple{x: 13., y: 15.5, z: 9.9, w: 4.};
  assert_eq!(t * 2., Tuple{x: 26., y: 31., z: 19.8, w: 8.});
}

#[test]
fn test_div_vector_scalar() {
  let v = vector(13., 17., -7.);
  assert_eq!(v / 2., vector(6.5, 8.5, -3.5));
}

#[test]
fn test_magnitude1() {
  let v = vector(1., 0., 0.);
  assert_eq!(v.magnitude(), 1.);
}

#[test]
fn test_magnitude2() {
  let v = vector(0., 1., 0.);
  assert_eq!(v.magnitude(), 1.);
}

#[test]
fn test_magnitude3() {
  let v = vector(0., 0., 1.);
  assert_eq!(v.magnitude(), 1.);
}

#[test]
fn test_magnitude4() {
  let v = vector(1., 2., 3.);
  let expected = 14.0_f64;
  assert_eq!(v.magnitude(), expected.sqrt());
}

#[test]
fn test_magnitude5() {
  let v = vector(-1., -2., -3.);
  let expected = 14.0_f64;
  assert_eq!(v.magnitude(), expected.sqrt());
}

#[test]
fn test_normalize1() {
  let v = vector(4., 0., 0.);
  assert_eq!(v.normalize(), vector(1., 0., 0.));
}

#[test]
fn test_normalize2() {
  let v = vector(1., 2., 3.);
  assert_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178));
}

#[test]
fn test_normalize3() {
  let v = vector(1., 2., 3.);
  assert_eq!(v.normalize().magnitude(), 1.0);
}

#[test]
fn test_dot() {
  let v1 = vector(1., 2., 3.);
  let v2 = vector(2., 3., 4.);
  assert_eq!(v1.dot(&v2), 20.0);
}

#[test]
fn test_cross() {
  let v1 = vector(1., 2., 3.);
  let v2 = vector(2., 3., 4.);
  assert_eq!(v1.cross(&v2), vector(-1., 2., -1.));
  assert_eq!(v2.cross(&v1), vector(1., -2., 1.));
}

#[test]
fn test_color_add() {
  let c1 = color(0.9, 0.6, 0.75);
  let c2 = color(0.7, 0.1, 0.25);
  assert_eq!(c1 + c2, color(1.6, 0.7, 1.));
}

#[test]
fn test_color_sub() {
  let c1 = color(0.9, 0.6, 0.75);
  let c2 = color(0.7, 0.1, 0.25);
  assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
}

#[test]
fn test_color_mul_scalar() {
  let c = color(0.2, 0.3, 0.4);
  assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
}

#[test]
fn test_color_mul_color() {
  let c1 = color(1.0, 0.2, 0.4);
  let c2 = color(0.9, 1.0, 0.1);
  assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
}