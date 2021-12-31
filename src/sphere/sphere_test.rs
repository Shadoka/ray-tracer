use super::*;

#[test]
fn test_create_sphere() {
  let s1 = sphere();
  let s2 = sphere();

  assert_ne!(s1.id, s2.id);
}