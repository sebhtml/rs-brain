use std::{
    fmt::Display,
    ops::{Mul, Neg, Sub},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<f32>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, values: Vec<f32>) -> Self {
        Self { rows, cols, values }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

pub trait ElementWiseProduct {
    fn element_wise_product(&self, rhs: &Matrix) -> Result<Matrix, Error>;
}

impl ElementWiseProduct for Matrix {
    fn element_wise_product(&self, rhs: &Matrix) -> Result<Matrix, Error> {
        let lhs: &Matrix = self;
        let (lhs_rows, lhs_cols) = lhs.shape();
        let (rhs_rows, rhs_cols) = rhs.shape();
        if lhs_rows != rhs_rows || lhs_cols != rhs_cols {
            return Err(Error::IncompatibleMatrixShapes);
        }
        let mut values = Vec::new();
        values.resize(lhs.values.len(), 0.0);
        for i in 0..lhs.values.len() {
            values[i] = lhs.values[i] * rhs.values[i];
        }
        Ok(Matrix::new(lhs_rows, lhs_cols, values))
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompatibleMatrixShapes,
}

impl Mul for &Matrix {
    type Output = Result<Matrix, Error>;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        let lhs: &Matrix = self;
        let (lhs_rows, lhs_cols) = lhs.shape();
        let (rhs_rows, rhs_cols) = rhs.shape();
        if lhs_cols != rhs_rows {
            return Err(Error::IncompatibleMatrixShapes);
        }
        let (output_rows, output_cols) = (lhs_rows, rhs_cols);
        let mut values = Vec::new();
        values.resize(output_rows * output_cols, 0.0);

        for output_row in 0..output_rows {
            for output_col in 0..output_cols {
                let mut lhs_index = output_row * lhs_cols;
                let mut rhs_index = output_col;
                let output_value: &mut f32 = &mut values[output_row * output_cols + output_col];
                for _ in 0..lhs_cols {
                    *output_value += lhs.values[lhs_index] * rhs.values[rhs_index];
                    lhs_index += 1;
                    rhs_index += rhs_cols;
                }
            }
        }
        Ok(Matrix::new(output_rows, output_cols, values))
    }
}

// TODO add Sub test
impl Sub for &Matrix {
    type Output = Result<Matrix, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs: &Matrix = self;
        let (lhs_rows, lhs_cols) = lhs.shape();
        let (rhs_rows, rhs_cols) = rhs.shape();
        if lhs_rows != rhs_rows || lhs_cols != rhs_cols {
            return Err(Error::IncompatibleMatrixShapes);
        }
        let mut values = Vec::new();
        values.resize(lhs.values.len(), 0.0);
        for i in 0..lhs.values.len() {
            values[i] = lhs.values[i] - rhs.values[i];
        }
        Ok(Matrix::new(lhs_rows, lhs_cols, values))
    }
}

impl Neg for Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        let mut values = Vec::new();
        values.resize(self.values.len(), 0.0);
        for i in 0..self.values.len() {
            values[i] = -self.values[i];
        }
        Matrix::new(self.rows, self.cols, values)
    }
}

impl Into<Vec<f32>> for Matrix {
    fn into(self) -> Vec<f32> {
        self.values
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = write!(f, "Shape: {}x{}", self.rows, self.cols);
        _ = write!(f, "\n");
        for row in 0..self.rows {
            for col in 0..self.cols {
                _ = write!(f, " {:2.2}", self.values[row * self.cols + col]);
            }
            _ = write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{Error, Matrix};

    #[test]
    fn new() {
        // Given rows and cols
        // When a matrix is built
        // Then it has the appropriate shape

        let matrix = Matrix::new(
            4,
            3,
            vec![
                0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, //
                0.0, 0.0, 0.0, //
            ],
        );
        assert_eq!(matrix.shape(), (4, 3));
    }

    #[test]
    fn multiplication_shape_compatibility() {
        // Given two matrices with incompatible shapes
        // When a matrix multiplication is done
        // Then there is an error

        let lhs = Matrix::new(
            1,
            1,
            vec![
                0.0, //
            ],
        );

        let rhs = Matrix::new(
            2,
            1,
            vec![
                0.0, //
                0.0, //
            ],
        );
        let actual_product = &lhs * &rhs;
        assert_eq!(actual_product, Err(Error::IncompatibleMatrixShapes))
    }

    #[test]
    fn multiplication() {
        // Given a left-hand side matrix and and a right-hand side matrix
        // When the multiplication lhs * rhs is done
        // Then the resulting matrix has the correct values

        let lhs = Matrix::new(
            3,
            2,
            vec![
                1.0, 2.0, //
                3.0, 4.0, //
                5.0, 6.0, //
            ],
        );
        let rhs = Matrix::new(
            2,
            3,
            vec![
                11.0, 12.0, 13.0, //
                14.0, 15.0, 16.0, //
            ],
        );
        let actual_product = &lhs * &rhs;
        let expected_product = Matrix::new(
            3,
            3,
            vec![
                1.0 * 11.0 + 2.0 * 14.0,
                1.0 * 12.0 + 2.0 * 15.0,
                1.0 * 13.0 + 2.0 * 16.0, //
                3.0 * 11.0 + 4.0 * 14.0,
                3.0 * 12.0 + 4.0 * 15.0,
                3.0 * 13.0 + 4.0 * 16.0, //
                5.0 * 11.0 + 6.0 * 14.0,
                5.0 * 12.0 + 6.0 * 15.0,
                5.0 * 13.0 + 6.0 * 16.0, //
            ],
        );

        assert_eq!(actual_product, Ok(expected_product));
    }
}
