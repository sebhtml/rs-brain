use rand::Rng;

use crate::tensor::{Error, Tensor};

#[test]
fn new() {
    // Given rows and cols
    // When a matrix is built
    // Then it has the appropriate shape

    let matrix = Tensor::new(
        4,
        3,
        vec![
            0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, //
        ],
    );
    assert_eq!((matrix.rows(), matrix.cols()), (4, 3));
}

#[test]
fn multiplication_shape_compatibility() {
    // Given two matrices with incompatible shapes
    // When a matrix multiplication is done
    // Then there is an error

    let lhs = Tensor::new(
        1,
        1,
        vec![
            0.0, //
        ],
    );

    let rhs = Tensor::new(
        2,
        1,
        vec![
            0.0, //
            0.0, //
        ],
    );

    let mut result = Tensor::default();
    let error = lhs.matmul(&rhs, &mut result);
    assert_eq!(error, Err(Error::IncompatibleTensorShapes))
}

#[test]
fn matrix_multiplication_result() {
    // Given a left-hand side matrix and and a right-hand side matrix
    // When the multiplication lhs * rhs is done
    // Then the resulting matrix has the correct values

    let lhs = Tensor::new(
        3,
        2,
        vec![
            1.0, 2.0, //
            3.0, 4.0, //
            5.0, 6.0, //
        ],
    );
    let rhs = Tensor::new(
        2,
        3,
        vec![
            11.0, 12.0, 13.0, //
            14.0, 15.0, 16.0, //
        ],
    );
    let expected_result = Tensor::new(
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

    let mut result = Tensor::default();
    _ = lhs.matmul(&rhs, &mut result);
    assert_eq!(result, expected_result);
}

#[test]
fn matrix_addition_result() {
    // Given a left-hand side matrix and and a right-hand side matrix
    // When the addition lhs + rhs is done
    // Then the resulting matrix has the correct values

    let lhs = Tensor::new(
        3,
        2,
        vec![
            1.0, 2.0, //
            3.0, 4.0, //
            5.0, 6.0, //
        ],
    );
    let rhs: Tensor = Tensor::new(
        3,
        2,
        vec![
            11.0, 12.0, //
            14.0, 15.0, //
            13.0, 16.0, //
        ],
    );
    let expected_result = Tensor::new(
        3,
        2,
        vec![
            1.0 + 11.0,
            2.0 + 12.0, //
            3.0 + 14.0,
            4.0 + 15.0, //
            5.0 + 13.0,
            6.0 + 16.0, //
        ],
    );

    let mut result = Tensor::default();
    _ = &lhs.add(&rhs, &mut result);
    assert_eq!(result, expected_result);
}

#[test]
fn sub_result() {
    // Given a left-hand side matrix and and a right-hand side matrix
    // When the addition lhs + rhs is done
    // Then the resulting matrix has the correct values

    let lhs = Tensor::new(
        3,
        2,
        vec![
            1.0, 2.0, //
            3.0, 4.0, //
            5.0, 6.0, //
        ],
    );
    let rhs: Tensor = Tensor::new(
        3,
        2,
        vec![
            11.0, 12.0, //
            14.0, 15.0, //
            13.0, 16.0, //
        ],
    );
    let expected_result = Tensor::new(
        3,
        2,
        vec![
            1.0 - 11.0,
            2.0 - 12.0, //
            3.0 - 14.0,
            4.0 - 15.0, //
            5.0 - 13.0,
            6.0 - 16.0, //
        ],
    );

    let mut result = Tensor::default();
    _ = &lhs.sub(&rhs, &mut result);
    assert_eq!(result, expected_result);
}

#[test]
fn element_wise_mul_result() {
    // Given a left-hand side matrix and and a right-hand side matrix
    // When the element-wise multiplication is done
    // Then the resulting matrix has the correct values

    let lhs = Tensor::new(
        3,
        2,
        vec![
            1.0, 2.0, //
            3.0, 4.0, //
            5.0, 6.0, //
        ],
    );
    let rhs: Tensor = Tensor::new(
        3,
        2,
        vec![
            11.0, 12.0, //
            14.0, 15.0, //
            13.0, 16.0, //
        ],
    );
    let expected_result = Tensor::new(
        3,
        2,
        vec![
            1.0 * 11.0,
            2.0 * 12.0, //
            3.0 * 14.0,
            4.0 * 15.0, //
            5.0 * 13.0,
            6.0 * 16.0, //
        ],
    );

    let mut result = Tensor::default();
    _ = &lhs.element_wise_mul(&rhs, &mut result);
    assert_eq!(result, expected_result);
}

#[test]
fn big_matrix_multiplication() {
    let rows = 1024;
    let cols = 1024;
    let mut values = Vec::new();
    values.resize(rows * cols, 0.0);
    for index in 0..values.len() {
        values[index] = rand::thread_rng().gen_range(0.0..1.0)
    }
    let m = Tensor::new(rows, cols, values);

    let mut result = Tensor::default();
    _ = m.matmul(&m, &mut result);
}

#[test]
fn big_matrix_addition() {
    let rows = 1024;
    let cols = 1024;
    let mut values = Vec::new();
    values.resize(rows * cols, 0.0);
    for index in 0..values.len() {
        values[index] = rand::thread_rng().gen_range(0.0..1.0)
    }
    let m = Tensor::new(rows, cols, values);

    let mut result = Tensor::default();
    _ = m.add(&m, &mut result);
}

#[test]
fn transpose() {
    let matrix = Tensor::new(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let matrix2 = matrix.transpose();
    for row in 0..matrix.rows() {
        for col in 0..matrix.cols() {
            assert_eq!(matrix2.get(col, row), matrix.get(row, col));
        }
    }
}
