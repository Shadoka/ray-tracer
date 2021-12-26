use crate::tuple::{point, vector};

use super::*;

#[test]
fn test_ray() {
  let origin = point(1.0, 2.0, 3.0);
  let direction = vector(4.0, 5.0, 6.0);
  let r = ray(&origin, &direction);

  assert_eq!(r.origin, origin);
  assert_eq!(r.direction, direction);
}