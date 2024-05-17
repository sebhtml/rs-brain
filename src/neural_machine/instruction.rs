use std::rc::Rc;

use crate::{Error, Operator, TensorF32};

#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    Inference,
    Loss,
    Gradient,
    Optimization,
}

#[derive(Clone, Debug)]
pub struct Instruction {
    operator: Rc<dyn Operator>,
    inputs: Rc<Vec<TensorF32>>,
    outputs: Rc<Vec<TensorF32>>,
    category: Category,
}

impl Instruction {
    pub fn new(
        operator: Rc<dyn Operator>,
        inputs: &[&TensorF32],
        outputs: &[&TensorF32],
        category: Category,
    ) -> Self {
        let inputs: Vec<TensorF32> = inputs.into_iter().map(|x| (*x).clone()).collect();
        let outputs: Vec<TensorF32> = outputs.into_iter().map(|x| (*x).clone()).collect();
        Self {
            operator,
            inputs: inputs.into(),
            outputs: outputs.into(),
            category,
        }
    }

    pub fn category(&self) -> Category {
        self.category.clone()
    }

    pub fn operator(&self) -> &Rc<dyn Operator> {
        &self.operator
    }
    pub fn inputs(&self) -> &Rc<Vec<TensorF32>> {
        &self.inputs
    }
    pub fn outputs(&self) -> &Rc<Vec<TensorF32>> {
        &self.outputs
    }
    pub fn forward(&self) -> Result<(), Error> {
        let inputs: Vec<&TensorF32> = self.inputs.iter().collect();
        let outputs_f32: Vec<&TensorF32> = self.outputs.iter().collect();
        self.operator.forward(&inputs, &outputs_f32)
    }
}
