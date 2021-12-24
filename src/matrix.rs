
pub struct Matrix2 {
  pub values: [[f64; 2]; 2]
}

pub struct Matrix3 {
  pub values: [[f64; 3]; 3]
}

pub struct Matrix4 {
  pub values: [[f64; 4]; 4]
}

// public static functions

pub fn create_matrix2_by_rows(values: Vec<f64>) -> Matrix2 {
  if values.len() != 4 {
    panic!("Matrix2 needs 4 values to be created");
  }

  let mut m_values = [[0.0; 2]; 2];
  for i in 0..4 {
    m_values[i/2][i%2] = values[i];
  }

  return Matrix2{values: m_values};
}

pub fn create_matrix3_by_rows(values: Vec<f64>) -> Matrix3 {
  if values.len() != 9 {
    panic!("Matrix3 needs 9 values to be created");
  }

  let mut m_values = [[0.0; 3]; 3];
  for i in 0..9 {
    m_values[i/3][i%3] = values[i];
  }

  return Matrix3{values: m_values};
}

pub fn create_matrix4_by_rows(values: Vec<f64>) -> Matrix4 {
  if values.len() != 16 {
    panic!("Matrix4 needs 16 values to be created");
  }

  let mut m_values = [[0.0; 4]; 4];
  for i in 0..16 {
    m_values[i/4][i%4] = values[i];
  }

  return Matrix4{values: m_values};
}

impl Matrix4 {

}

#[cfg(test)]
mod matrix_test;