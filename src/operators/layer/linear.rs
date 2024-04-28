use std::ops::Deref;

use rand::{distributions::Uniform, thread_rng, Rng};

use crate::{devices::Device, DeltaWorkingMemory, Error, LearningTensor, OperatorTrait, TensorF32};

pub struct Linear {
    weights: LearningTensor,
    biases: LearningTensor,
}

impl Linear {
    pub fn new(
        weights_rows: usize,
        weights_cols: usize,
        bias_rows: usize,
        device: &Device,
    ) -> Self {
        // Xavier Initialization, or Glorot Initialization,
        let mut rng = thread_rng();
        let right = (6.0 as f32).sqrt() / (weights_cols as f32 + weights_rows as f32).sqrt();
        let left = -right;
        let uniform = Uniform::new(left, right);

        let mut weights = Vec::new();
        weights.resize(weights_rows * weights_cols, 0.0);
        for index in 0..weights.len() {
            weights[index] = rng.sample(uniform);
        }
        let weights = device.learning_tensor(weights_rows, weights_cols, weights, true);

        let biases_len = bias_rows * weights_rows;
        let biases = device.learning_tensor(bias_rows, weights_rows, vec![0.0; biases_len], true);

        Linear { weights, biases }
    }
}

impl OperatorTrait for Linear {
    fn forward(&self, device: &Device, inputs: &[LearningTensor]) -> Result<LearningTensor, Error> {
        debug_assert_eq!(inputs.len(), 1);
        let input: &TensorF32 = &inputs[0].tensor().deref().borrow();
        let output = device.learning_tensor(0, 0, vec![], false);
        // Use the same convention that is used in tensorflow:
        // Y = X @ W^T + B
        // Weights is on the right.
        // X is not transposed.
        // W is transposed.

        // use GEMM to do C = A * W^T + C  with weights and biases all together.
        {
            let output: &mut TensorF32 = &mut output.tensor().deref().borrow_mut();
            let weights: &TensorF32 = &self.weights.tensor().deref().borrow();
            let biases: &TensorF32 = &self.biases.tensor().deref().borrow();
            let a = input;
            let b = weights;
            let c = output;
            c.assign(device, biases)?;
            let op_result = TensorF32::gemm(device, false, true, 1.0, a, b, 1.0, c, false);
            match op_result {
                Ok(_) => (),
                Err(_) => {
                    let mut w_t = device.tensor(0, 0, vec![]);
                    b.transpose(&mut w_t)?;
                    println!("Incompatible shapes in matrix multiplication");
                    println!("Between X {:?} and W^T {:?}", input.shape(), w_t.shape(),);
                    debug_assert!(false);
                }
            }
        }

        Ok(output)
    }

    fn backward(
        &self,
        device: &Device,
        _error_working_memory: &mut DeltaWorkingMemory,
        inputs: &[LearningTensor],
        output: &LearningTensor,
    ) -> Result<(), Error> {
        let back_propagated_delta: &TensorF32 = &output.gradient().deref().borrow();
        {
            let weights_gradient: &mut TensorF32 =
                &mut self.weights.gradient().deref().borrow_mut();
            let biases_gradient: &mut TensorF32 = &mut self.biases.gradient().deref().borrow_mut();
            let input: &TensorF32 = &inputs[0].tensor().deref().borrow();
            let a: &TensorF32 = input;
            let b: &TensorF32 = back_propagated_delta;
            let c: &mut TensorF32 = weights_gradient;
            c.reset(b.cols(), a.cols(), 0.0)?;
            TensorF32::matmul(device, true, false, a, b, c, true)?;

            biases_gradient.assign(device, back_propagated_delta)?;
        }

        {
            let backward_gradient: &mut TensorF32 = &mut inputs[0].gradient().deref().borrow_mut();
            let weights: &TensorF32 = &self.weights.tensor().deref().borrow();
            let a: &TensorF32 = weights;
            let b: &TensorF32 = back_propagated_delta;
            let c: &mut TensorF32 = backward_gradient;
            c.reset(b.rows(), a.cols(), 0.0)?;
            TensorF32::matmul(device, true, true, a, b, c, true)?;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        "Linear"
    }
}
