#[cfg(test)]
pub mod tests;
pub mod train;
use std::mem::swap;

use crate::{
    loss::{LossFunction, LossFunctionType},
    ActivationType, Error, Layer, LayerConfig, LayerType, Tensor, TensorTrait,
};

pub struct Network<'a> {
    layers: Vec<LayerType>,
    loss_function: &'a LossFunctionType,
    using_softmax_and_cross_entropy_loss: bool,
}

pub struct TrainWorkingMemory {
    pub next_layer_delta: Tensor,
    pub layer_delta: Tensor,
    pub previous_activation_tensor_f32: Tensor,
    pub last_activation_row: Tensor,
    pub loss: Tensor,
    pub tmp: Tensor,
}

impl Default for TrainWorkingMemory {
    fn default() -> Self {
        Self {
            next_layer_delta: Default::default(),
            layer_delta: Default::default(),
            previous_activation_tensor_f32: Default::default(),
            last_activation_row: Default::default(),
            loss: Default::default(),
            tmp: Default::default(),
        }
    }
}

pub struct DeltaWorkingMemory {
    pub output_diff: Tensor,
    pub layer_f_derivative: Tensor,
}

impl Default for DeltaWorkingMemory {
    fn default() -> Self {
        Self {
            output_diff: Default::default(),
            layer_f_derivative: Default::default(),
        }
    }
}

pub struct PredictWorkingMemory {
    pub last_activation_row: Tensor,
    pub previous_activation_tensor_f32: Tensor,
    pub activation_tensor: Tensor,
    pub activation_tensors: Vec<Tensor>,
}

impl PredictWorkingMemory {
    pub fn new(examples_count: usize) -> Self {
        Self {
            last_activation_row: Default::default(),
            previous_activation_tensor_f32: Default::default(),
            activation_tensor: Default::default(),
            activation_tensors: vec![Tensor::default(); examples_count],
        }
    }
}

impl<'a> Network<'a> {
    pub fn new(layer_configs: &Vec<LayerConfig>, loss_function: &'a LossFunctionType) -> Self {
        let mut using_softmax_and_cross_entropy_loss = false;
        match loss_function {
            LossFunctionType::CrossEntropyLoss(_) => match layer_configs.last() {
                Some(config) => match config {
                    LayerConfig::Linear(config) => match config.activation {
                        ActivationType::Softmax(_) => {
                            using_softmax_and_cross_entropy_loss = true;
                        }
                        _ => {
                            assert!(false, "CrossEntropyLoss only works with Softmax");
                        }
                    },
                    _ => (),
                },
                _ => (),
            },
            _ => (),
        }

        Self {
            layers: layer_configs
                .into_iter()
                .map(|layer_config| layer_config.into())
                .collect(),
            loss_function,
            using_softmax_and_cross_entropy_loss,
        }
    }

    pub fn train(
        &mut self,
        working_memory: &mut TrainWorkingMemory,
        error_working_memory: &mut DeltaWorkingMemory,
        epoch: usize,
        inputs: &Vec<Tensor>,
        outputs: &Vec<Tensor>,
    ) {
        for i in 0..inputs.len() {
            self.train_back_propagation(
                working_memory,
                error_working_memory,
                epoch,
                i,
                &inputs[i],
                &outputs[i],
            );
        }
    }

    fn get_last_layer_delta(
        &self,
        layer_activation_tensor: &Tensor,
        last_activation_row: &mut Tensor,
        tmp: &mut Tensor,
        loss: &mut Tensor,
        y: &Tensor,
        next_layer_delta: &mut Tensor,
    ) {
        let last_row = layer_activation_tensor.rows() - 1;
        layer_activation_tensor.row(last_row, last_activation_row);
        let op_result = self
            .loss_function
            .derive(tmp, y, &last_activation_row, loss);
        op_result.expect("Ok");

        next_layer_delta.reshape(
            layer_activation_tensor.rows(),
            layer_activation_tensor.cols(),
        );
        let mut col = 0;
        let cols = loss.cols();

        while col < cols {
            let value = loss.get(0, col);
            next_layer_delta.set(last_row, col, value);
            col += 1;
        }
    }

    pub fn total_error(
        &mut self,
        working_memory: &mut PredictWorkingMemory,
        inputs: &Vec<Tensor>,
        outputs: &Vec<Tensor>,
    ) -> Result<f32, Error> {
        let mut total_error = 0.0;
        let activation_tensor = &mut working_memory.activation_tensor;
        let previous_activation_tensor_f32 = &mut working_memory.previous_activation_tensor_f32;
        let last_activation_row = &mut working_memory.last_activation_row;

        for i in 0..inputs.len() {
            self.predict(
                previous_activation_tensor_f32,
                &inputs[i],
                activation_tensor,
            );
            let target = &outputs[i];
            let last_row = activation_tensor.rows() - 1;
            activation_tensor.row(last_row, last_activation_row);
            let example_error = self.loss_function.evaluate(target, &last_activation_row)?;
            total_error += example_error;
        }

        Ok(total_error)
    }

    fn train_back_propagation(
        &mut self,
        working_memory: &mut TrainWorkingMemory,
        error_working_memory: &mut DeltaWorkingMemory,
        _epoch: usize,
        _example_index: usize,
        x: &Tensor,
        y: &Tensor,
    ) {
        let learning_rate: f32 = 0.5;

        for layer_index in 0..self.layers.len() {
            let previous_activation_tensor = match layer_index {
                0 => x,
                _ => {
                    let previous_activation_tensor =
                        self.layers[layer_index - 1].get_activation_tensor();
                    let previous_activation_tensor_f32 =
                        &mut working_memory.previous_activation_tensor_f32;
                    previous_activation_tensor_f32.assign(previous_activation_tensor);
                    previous_activation_tensor_f32
                }
            };

            let layer = &mut self.layers[layer_index];
            let op_result = layer.forward(previous_activation_tensor);
            op_result.expect("Ok");
        }

        let next_layer_delta = &mut working_memory.next_layer_delta;
        let layer_delta = &mut working_memory.layer_delta;
        let layers_count = self.layers.len();

        // Back-propagation
        for layer_index in (0..layers_count).into_iter().rev() {
            let is_last_layer = layer_index == self.layers.len() - 1;

            let previous_activation_tensor = match layer_index {
                0 => x,
                _ => {
                    let previous_activation_tensor =
                        self.layers[layer_index - 1].get_activation_tensor();
                    let previous_activation_tensor_f32 =
                        &mut working_memory.previous_activation_tensor_f32;
                    previous_activation_tensor_f32.assign(previous_activation_tensor);
                    previous_activation_tensor_f32
                }
            };

            if is_last_layer {
                // For the output layer, the next layer delta is the loss.
                let last_activation_row = &mut working_memory.last_activation_row;
                let tmp = &mut working_memory.tmp;
                let loss = &mut working_memory.loss;
                let layer_activation_tensor = self.layers[layer_index].get_activation_tensor();
                self.get_last_layer_delta(
                    layer_activation_tensor,
                    last_activation_row,
                    tmp,
                    loss,
                    y,
                    next_layer_delta,
                );
            }

            {
                let next_layer = if is_last_layer {
                    None
                } else {
                    let next_layer_index = layer_index + 1;
                    Some(&self.layers[next_layer_index])
                };

                let layer = &self.layers[layer_index];
                layer.get_layer_delta(
                    error_working_memory,
                    next_layer,
                    next_layer_delta,
                    self.using_softmax_and_cross_entropy_loss,
                    layer_delta,
                );
            }

            {
                let layer = &mut self.layers[layer_index];
                layer.plan_change(learning_rate, previous_activation_tensor, layer_delta);
            }

            swap(next_layer_delta, layer_delta);
        }

        // Apply changes
        for layer in 0..self.layers.len() {
            let op_result = self.layers[layer].commit_change();
            op_result.expect("Ok");
        }
    }

    pub fn predict_many(
        &mut self,
        previous_activation_tensor: &mut Tensor,
        inputs: &Vec<Tensor>,
        activation_tensors: &mut Vec<Tensor>,
    ) {
        let len = inputs.len();
        let mut i = 0;
        while i < len {
            let input = &inputs[i];
            let activation_tensor = &mut activation_tensors[i];
            self.predict(previous_activation_tensor, input, activation_tensor);
            i += 1;
        }
    }

    pub fn predict(
        &mut self,
        previous_activation_tensor_f32: &mut Tensor,
        input: &Tensor,
        activation_tensor: &mut Tensor,
    ) {
        for layer_index in 0..self.layers.len() {
            let previous_activation_tensor = match layer_index {
                0 => input,
                _ => {
                    let previous_activation_tensor =
                        self.layers[layer_index - 1].get_activation_tensor();
                    previous_activation_tensor_f32.assign(previous_activation_tensor);
                    &previous_activation_tensor_f32
                }
            };
            let layer = &mut self.layers[layer_index];
            let op_result = layer.forward(previous_activation_tensor);
            activation_tensor.assign(layer.get_activation_tensor());
            op_result.expect("Ok");
        }
    }
}
