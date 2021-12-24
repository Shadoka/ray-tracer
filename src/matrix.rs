
pub struct Matrix4 {
  pub values: [[f64; 4]; 4]
}

// public static functions

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