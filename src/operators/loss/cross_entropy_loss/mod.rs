use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::LossFunction;
use crate::{devices::Device, Error, OperatorTrait, Tensor, TensorF32};

#[derive(Clone)]
pub struct CrossEntropyLoss {}

impl Default for CrossEntropyLoss {
    fn default() -> Self {
        Self {}
    }
}

const EPSILON: f32 = 1e-8;

impl LossFunction for CrossEntropyLoss {
    /// H(P, Q) = - Σ (P(i) * log(Q(i)))
    fn evaluate(
        &self,
        _device: &Device,
        expected: &TensorF32,
        actual: &TensorF32,
    ) -> Result<f32, Error> {
        debug_assert_eq!(actual.shape(), expected.shape());
        let p = expected;
        let q = actual;
        if p.shape() != q.shape() {
            return Err(Error::IncompatibleTensorShapes);
        }
        let rows = p.rows();
        let cols = p.cols();
        let mut row = 0;
        let mut col = 0;
        let mut sum = 0.0;
        let p_values = p.get_values()?;
        let q_values = q.get_values()?;
        while row < rows {
            while col < cols {
                let p_i = p_values[p.index(row, col)];
                let q_i = q_values[q.index(row, col)] + EPSILON;
                sum += p_i * f32::ln(q_i);
                col += 1;
            }
            row += 1;
        }
        debug_assert!(sum.is_finite());
        Ok(-sum)
    }

    /// When Cross-Entropy Loss is used with a Softmax activation function,
    /// then we don't need to derive the softmax activations.
    /// The derivative of the Loss in respect to logits (before activation) is
    /// output of the softmax function - expected output (one-hot encoded)
    fn derive(
        &self,
        device: &Device,
        expected: &TensorF32,
        actual: &TensorF32,
        result: &mut TensorF32,
    ) -> Result<(), Error> {
        TensorF32::copy(device, actual, result)?;
        TensorF32::sub(device, expected, result)
    }
}

impl OperatorTrait for CrossEntropyLoss {
    fn backward(&self, device: &Device, inputs: &[Tensor], _output: &Tensor) -> Result<(), Error> {
        debug_assert_eq!(inputs.len(), 2);
        let expected: &TensorF32 = &inputs[0].tensor().deref().borrow();
        let actual: &TensorF32 = &inputs[1].tensor().deref().borrow();
        let backward_gradient: &mut TensorF32 = &mut inputs[1].gradient().deref().borrow_mut();
        self.derive(device, expected, actual, backward_gradient)?;
        Ok(())
    }

    fn forward(&self, device: &Device, inputs: &[Tensor]) -> Result<Tensor, Error> {
        debug_assert_eq!(inputs.len(), 2);
        let expected: &TensorF32 = &inputs[0].tensor().deref().borrow();
        let actual: &TensorF32 = &inputs[1].tensor().deref().borrow();
        let loss = self.evaluate(device, expected, actual)?;
        let output = device.tensor(
            Rc::new(RefCell::new(Box::new(self.clone()))),
            inputs,
            1,
            1,
            vec![loss],
            false,
        );
        Ok(output)
    }

    fn name(&self) -> &str {
        "CrossEntropyLoss"
    }
}
