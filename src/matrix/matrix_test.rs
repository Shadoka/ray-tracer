use super::*;

#[test]
fn test_create_matrix4() {
  let values = vec!(1.0, 2.0, 3.0, 4.0,
    5.5, 6.5, 7.5, 8.5,
    9.0, 10.0, 11.0, 12.0,
    13.5, 14.5, 15.5, 16.5
  );
  let m = create_matrix4_by_rows(values);

  assert_eq!(m.values[0][0], 1.0);
  assert_eq!(m.values[0][3], 4.0);
  assert_eq!(m.values[1][0], 5.5);
  assert_eq!(m.values[1][2], 7.5);
  assert_eq!(m.values[2][2], 11.0);
  assert_eq!(m.values[3][0], 13.5);
  assert_eq!(m.values[3][2], 15.5);
}