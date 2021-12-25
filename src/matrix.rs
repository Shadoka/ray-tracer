use crate::tuple::Tuple;

use std::ops;

#[derive(Debug)]
pub struct Matrix2 {
  pub values: [[f64; 2]; 2]
}

#[derive(Debug)]
pub struct Matrix3 {
  pub values: [[f64; 3]; 3]
}

#[derive(Debug)]
pub struct Matrix4 {
  pub values: [[f64; 4]; 4]
}

// const

const IDENTITY: Matrix4 = Matrix4 {values: [
  [1.0, 0.0, 0.0, 0.0],
  [0.0, 1.0, 0.0, 0.0],
  [0.0, 0.0, 1.0, 0.0],
  [0.0, 0.0, 0.0, 1.0]
  ]};

// trait impls

impl PartialEq for Matrix2 {
  fn eq(&self, other: &Self) -> bool {
    let mut result = true;
    for i in 0..2 {
      for j in 0..2 {
        result = result && self.values[i][j] == other.values[i][j];
      }
    }
    result
  }
}

impl Eq for Matrix2 {}

impl PartialEq for Matrix3 {
  fn eq(&self, other: &Self) -> bool {
    let mut result = true;
    for i in 0..3 {
      for j in 0..3 {
        result = result && self.values[i][j] == other.values[i][j];
      }
    }
    result
  }
}

impl Eq for Matrix3 {}

impl PartialEq for Matrix4 {
  fn eq(&self, other: &Self) -> bool {
    let mut result = true;
    for i in 0..4 {
      for j in 0..4 {
        result = result && equals_float(self.values[i][j], other.values[i][j]);
      }
    }
    result
  }
}

impl Eq for Matrix4 {}

impl ops::Mul<Matrix4> for Matrix4 {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    let mut values: Vec<f64> = Vec::new();
    for i in 0..4 {
      for j in 0..4 {
        values.push(mul_row_by_column(&self, &other, i, j));
      }
    }
    return matrix4(&values);
  }
}

impl ops::Mul<&Matrix4> for &Matrix4 {
  type Output = Matrix4;

  fn mul(self, other: &Matrix4) -> Matrix4 {
    let mut values: Vec<f64> = Vec::new();
    for i in 0..4 {
      for j in 0..4 {
        values.push(mul_row_by_column(&self, &other, i, j));
      }
    }
    return matrix4(&values);
  }
}

impl ops::Mul<Tuple> for Matrix4 {
  type Output = Tuple;

  fn mul(self, other: Tuple) -> Tuple {
    let mut values: Vec<f64> = Vec::new();
    for i in 0..4 {
      values.push(self.values[i][0] * other.x
        + self.values[i][1] * other.y
        + self.values[i][2] * other.z
        + self.values[i][3] * other.w
      );
    }
    return Tuple{x: values[0], y: values[1], z: values[2], w: values[3]};
  }
}

// private static methods

fn equals_float(a: f64, b: f64) -> bool {
  return (a - b).abs() < 0.00001
}

fn mul_row_by_column(a: &Matrix4, b: &Matrix4, row: usize, column: usize) -> f64 {
  let mut result = 0.0;
  for x in 0..4 {
    result = result + a.values[row][x] * b.values[x][column];
  }
  return result;
}

// public static functions

pub fn matrix2(values: &Vec<f64>) -> Matrix2 {
  if values.len() != 4 {
    panic!("Matrix2 needs 4 values to be created");
  }

  let mut m_values = [[0.0; 2]; 2];
  for i in 0..4 {
    m_values[i/2][i%2] = values[i];
  }

  return Matrix2{values: m_values};
}

pub fn matrix3(values: &Vec<f64>) -> Matrix3 {
  if values.len() != 9 {
    panic!("Matrix3 needs 9 values to be created");
  }

  let mut m_values = [[0.0; 3]; 3];
  for i in 0..9 {
    m_values[i/3][i%3] = values[i];
  }

  return Matrix3{values: m_values};
}

pub fn matrix4(values: &Vec<f64>) -> Matrix4 {
  if values.len() != 16 {
    panic!("Matrix4 needs 16 values to be created");
  }

  let mut m_values = [[0.0; 4]; 4];
  for i in 0..16 {
    m_values[i/4][i%4] = values[i];
  }

  return Matrix4{values: m_values};
}

impl Matrix2 {
  pub fn det(&self) -> f64 {
    return self.values[0][0] * self.values[1][1]
      - self.values[0][1] * self.values[1][0];
  }
}

impl Matrix3 {
  pub fn submatrix(&self, row: usize, column: usize) -> Matrix2 {
    let mut values: Vec<f64> = Vec::new();

    for i in 0..3 {
      for j in 0..3 {
        if i != row && j != column {
          values.push(self.values[i][j]);
        }
      }
    }

    return matrix2(&values);
  }

  pub fn minor(&self, row: usize, column: usize) -> f64 {
    let submatrix = self.submatrix(row, column);
    return submatrix.det();
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f64 {
    let minor = self.minor(row, column);
    if (row + column) % 2 == 0 {
      return minor;
    } else {
      return -minor;
    }
  }

  pub fn det(&self) -> f64 {
    let mut cofactor_multiplications: Vec<f64> = Vec::new();
    for column in 0..3 {
      cofactor_multiplications.push(self.cofactor(0, column) * self.values[0][column]);
    }

    let mut result = 0.0;
    for x in 0..cofactor_multiplications.len() {
      result = result + cofactor_multiplications[x];
    }
    return result;
  }
}

impl Matrix4 {
  pub fn transpose(&self) -> Matrix4 {
    let mut transposed: Vec<f64> = Vec::new();
    for i in 0..4 {
      for j in 0..4 {
        transposed.push(self.values[j][i]);
      }
    }
    return matrix4(&transposed);
  }

  pub fn submatrix(&self, row: usize, column: usize) -> Matrix3 {
    let mut values: Vec<f64> = Vec::new();

    for i in 0..4 {
      for j in 0..4 {
        if i != row && j != column {
          values.push(self.values[i][j]);
        }
      }
    }

    return matrix3(&values);
  }

  pub fn minor(&self, row: usize, column: usize) -> f64 {
    let submatrix = self.submatrix(row, column);
    return submatrix.det();
  }

  pub fn cofactor(&self, row: usize, column: usize) -> f64 {
    let minor = self.minor(row, column);
    if (row + column) % 2 == 0 {
      return minor;
    } else {
      return -minor;
    }
  }

  pub fn det(&self) -> f64 {
    let mut cofactor_multiplications: Vec<f64> = Vec::new();
    for column in 0..4 {
      cofactor_multiplications.push(self.cofactor(0, column) * self.values[0][column]);
    }

    let mut result = 0.0;
    for x in 0..cofactor_multiplications.len() {
      result = result + cofactor_multiplications[x];
    }
    return result;
  }

  pub fn is_invertible(&self) -> bool {
    return !equals_float(self.det(), 0.0);
  }

  pub fn inverse(&self) -> Matrix4 {
    let determinant = self.det();
    if determinant == 0.0 {
      println!("determinant is 0!!!");
    }

    let mut values: Vec<f64> = Vec::new();
    for row in 0..4 {
      for column in 0..4 {
        let c = self.cofactor(column, row);
        values.push(c / determinant);
      }
    }
    return matrix4(&values);
  }
}

#[cfg(test)]
mod matrix_test;