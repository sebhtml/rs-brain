use std::fmt::Display;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Tensor {
    rows: usize,
    cols: usize,
    values: Vec<f32>,
}

impl Default for Tensor {
    fn default() -> Self {
        Self {
            rows: Default::default(),
            cols: Default::default(),
            values: Default::default(),
        }
    }
}

impl Tensor {
    pub fn new(rows: usize, cols: usize, values: Vec<f32>) -> Self {
        Self { rows, cols, values }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn reshape(&mut self, new_rows: usize, new_cols: usize) {
        self.rows = new_rows;
        self.cols = new_cols;
        let values = self.rows * self.cols;
        self.values.resize(values, 0.0)
    }

    pub fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        let index = self.index(row, col);
        self.values[index]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        let index = self.index(row, col);
        self.values[index] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut other: Tensor = Tensor::new(self.cols, self.rows, self.values.clone());
        let rows = self.rows;
        let cols = self.cols;
        let mut row = 0;
        while row < rows {
            let mut col = 0;
            while col < cols {
                let value = self.get(row, col);
                other.set(col, row, value);
                col += 1;
            }
            row += 1;
        }
        other
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompatibleTensorShapes,
}

fn add_matrix_tensor_and_matrix_tensor(
    left: &Tensor,
    right: &Tensor,
    result: &mut Tensor,
) -> Result<(), Error> {
    if left.rows != right.rows || left.cols != right.cols {
        return Err(Error::IncompatibleTensorShapes);
    }

    result.reshape(left.rows, left.cols);

    let result_ptr = result.values.as_mut_ptr();
    let left_ptr = left.values.as_ptr();
    let right_ptr = right.values.as_ptr();

    unsafe {
        let mut index = 0;
        let len = left.values.len();
        while index < len {
            let left_cell = left_ptr.add(index);
            let right_cell = right_ptr.add(index);
            let result_cell = result_ptr.add(index);
            *result_cell = *left_cell + *right_cell;
            index += 1;
        }
    }

    Ok(())
}

impl Tensor {
    pub fn add(&self, right: &Tensor, result: &mut Tensor) -> Result<(), Error> {
        add_matrix_tensor_and_matrix_tensor(self, right, result)
    }
}

fn multiply_matrix_tensor_and_matrix_tensor(
    left: &Tensor,
    right: &Tensor,
    result: &mut Tensor,
) -> Result<(), Error> {
    if left.cols != right.rows {
        return Err(Error::IncompatibleTensorShapes);
    }

    result.reshape(left.rows, right.cols);

    let result_ptr = result.values.as_mut_ptr();
    let left_ptr = left.values.as_ptr();
    let right_ptr = right.values.as_ptr();

    let left_rows = left.rows;
    let left_cols = left.cols;
    let right_cols = right.cols;

    unsafe {
        let mut row = 0;
        while row != left_rows {
            let mut inner = 0;
            while inner != left_cols {
                let mut col = 0;
                while col != right_cols {
                    let left_cell = left_ptr.add(row * left_cols + inner);
                    let right_cell = right_ptr.add(inner * right_cols + col);
                    let result_cell = result_ptr.add(row * right_cols + col);
                    *result_cell += *left_cell * *right_cell;
                    col += 1;
                }
                inner += 1;
            }
            row += 1;
        }
    }

    Ok(())
}

impl Tensor {
    pub fn mul(&self, right: &Tensor, result: &mut Tensor) -> Result<(), Error> {
        multiply_matrix_tensor_and_matrix_tensor(self, right, result)
    }
}

impl Into<Vec<f32>> for Tensor {
    fn into(self) -> Vec<f32> {
        self.values
    }
}

impl Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        _ = write!(f, "Shape: {:?}", (self.rows, self.cols));
        _ = write!(f, "\n");
        for row in 0..self.rows {
            for col in 0..self.cols {
                let value = self.get(row, col);
                if value < 0.0 {
                    _ = write!(f, " {:2.8}", value);
                } else {
                    _ = write!(f, " +{:2.8}", value);
                }
            }
            _ = write!(f, "\n");
        }
        Ok(())
    }
}