use crate::{
    common_adam::optimize, tensor::Error, Device, Instruction, OptimizerTrait, TensorWithGrad,
};

/// See:
/// Adam: A Method for Stochastic Optimization
/// https://arxiv.org/abs/1412.6980
pub struct Adam {
    learning_rate: f32,
    beta1: f32,
    beta2: f32,
    epsilon: f32,
    weight_decay: f32,
}

impl Adam {
    pub fn try_new(
        learning_rate: f32,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        weight_decay: f32,
    ) -> Result<Self, Error> {
        let adam = Self {
            learning_rate,
            beta1,
            beta2,
            epsilon,
            weight_decay,
        };
        Ok(adam)
    }
}

impl OptimizerTrait for Adam {
    fn optimize(
        &self,
        device: &Device,
        tensors: &[TensorWithGrad],
    ) -> Result<Vec<Instruction>, Error> {
        let is_adam_w = false;
        optimize(
            device,
            self.learning_rate,
            self.beta1,
            self.beta2,
            self.epsilon,
            self.weight_decay,
            is_adam_w,
            tensors,
        )
    }
}
