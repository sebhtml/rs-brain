use std::{ops::Deref, rc::Rc};

use crate::{Add, Device, Error, Instruction, OpCode, OptimizerTrait, Scale, Tensor, TensorF32};

pub struct GradientDescent {
    learning_rate: f32,
}

impl GradientDescent {
    pub fn new(learning_rate: f32) -> Self {
        Self { learning_rate }
    }
}

impl OptimizerTrait for GradientDescent {
    fn optimize(&self, device: &Device, tensors: &[Tensor]) -> Result<Vec<Instruction>, Error> {
        let mut instructions = vec![];
        for optimizable_tensor in tensors {
            let tensor: &TensorF32 = &optimizable_tensor.tensor().deref().borrow();
            let gradient: &TensorF32 = &optimizable_tensor.gradient().deref().borrow();
            debug_assert_eq!(gradient.size(), tensor.size(),);

            let scaled_gradient =
                device.tensor_f32(tensor.rows(), tensor.cols(), vec![0.0; tensor.len()]);

            instructions.push(Instruction::new(
                OpCode::DynOperator(Rc::new(Scale::new(device, 0.0))),
                &[&scaled_gradient],
                &[&scaled_gradient],
                crate::Category::Optimization,
            ));

            instructions.push(Instruction::new(
                OpCode::DynOperator(Rc::new(Add::new(device))),
                &[&scaled_gradient, gradient],
                &[&scaled_gradient],
                crate::Category::Optimization,
            ));

            instructions.push(Instruction::new(
                OpCode::DynOperator(Rc::new(Scale::new(device, -self.learning_rate))),
                &[&scaled_gradient],
                &[&scaled_gradient],
                crate::Category::Optimization,
            ));

            instructions.push(Instruction::new(
                OpCode::DynOperator(Rc::new(Add::new(device))),
                &[tensor, &scaled_gradient],
                &[tensor],
                crate::Category::Optimization,
            ));
        }

        Ok(instructions)
    }
}
