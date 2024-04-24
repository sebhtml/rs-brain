use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::LossFunction;
use crate::{devices::Device, DeltaWorkingMemory, Error, Gradient, OperatorTrait, Tensor};

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
    fn evaluate(&self, _device: &Device, expected: &Tensor, actual: &Tensor) -> Result<f32, Error> {
        debug_assert_eq!(actual.shape(), expected.shape());
        let p = expected;
        let q = actual;
        if p.shape() != q.shape() {
            return Err(Error::IncompatibleTensorShapes);
        }
        let rows = p.rows();
        debug_assert_eq!(rows, 1);
        let cols = p.cols();
        let mut col = 0;
        let mut sum = 0.0;
        while col < cols {
            let p_i = p.get(0, col);
            let q_i = q.get(0, col) + EPSILON;
            sum += p_i * f32::ln(q_i);
            col += 1;
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
        expected: &Tensor,
        actual: &Tensor,
        result: &mut Tensor,
    ) -> Result<(), Error> {
        result.assign(device, actual);
        Tensor::sub(device, expected, result)
    }
}

impl OperatorTrait for CrossEntropyLoss {
    fn backward(
        &self,
        device: &Device,
        _error_working_memory: &mut DeltaWorkingMemory,
        inputs: &Vec<Rc<RefCell<Tensor>>>,
        _output: &Rc<RefCell<Tensor>>,
        _back_propagated_delta: &Rc<RefCell<Tensor>>,
    ) -> Result<(Rc<RefCell<Tensor>>, Vec<Gradient>), Error> {
        debug_assert_eq!(inputs.len(), 2);
        let expected: &Tensor = &inputs[0].deref().borrow();
        let actual: &Tensor = &inputs[1].deref().borrow();
        let mut gradient = device.tensor(0, 0, vec![]);
        self.derive(device, expected, actual, &mut gradient)?;

        Ok((Rc::new(RefCell::new(gradient)), vec![]))
    }

    fn forward(
        &self,
        device: &Device,
        inputs: &Vec<Rc<RefCell<Tensor>>>,
    ) -> Result<Rc<RefCell<Tensor>>, Error> {
        debug_assert_eq!(inputs.len(), 2);
        let expected: &Tensor = &inputs[0].deref().borrow();
        let actual: &Tensor = &inputs[1].deref().borrow();
        let loss = self.evaluate(device, expected, actual)?;
        let output = device.tensor(1, 1, vec![loss]);
        Ok(Rc::new(RefCell::new(output)))
    }

    fn name(&self) -> &str {
        "CrossEntropyLoss"
    }
}
