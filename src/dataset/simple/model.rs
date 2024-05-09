use crate::{
    Device, Embedding, Error, Linear, Model, OperatorTrait, Reshape, Sigmoid, Softmax, Tensor,
};

pub struct SimpleModel {
    input_shape: Vec<usize>,
    output_shape: Vec<usize>,
    embedding: Embedding,
    linear_0: Linear,
    sigmoid_0: Sigmoid,
    reshape: Reshape,
    linear_1: Linear,
    sigmoid_1: Sigmoid,
    linear_2: Linear,
    softmax: Softmax,
}

impl SimpleModel {
    pub fn new(device: &Device) -> Self {
        let sequence_length = 6;
        let vocab_size = 256;
        let n_embd = 384;
        let output_rows = 1;

        let linear_1 = Linear::new(device, n_embd, sequence_length * n_embd, output_rows);

        Self {
            input_shape: vec![sequence_length, vocab_size],
            output_shape: vec![output_rows, vocab_size],
            embedding: Embedding::new(device, vocab_size, n_embd),
            linear_0: Linear::new(device, n_embd, n_embd, sequence_length),
            sigmoid_0: Sigmoid::new(device),
            reshape: Reshape::new(
                device,
                vec![sequence_length, n_embd],
                vec![output_rows, sequence_length * n_embd],
            ),
            linear_1,
            sigmoid_1: Sigmoid::new(device),
            linear_2: Linear::new(device, vocab_size, n_embd, output_rows),
            softmax: Softmax::new(device, true),
        }
    }
}

impl Model for SimpleModel {
    fn forward(&self, inputs: &[&Tensor]) -> Result<Tensor, Error> {
        let state_0: Tensor = self.embedding.forward(inputs)?;
        let state_1 = self.linear_0.forward(&[&state_0])?;
        let state_2 = self.sigmoid_0.forward(&[&state_1])?;
        let state_3 = self.reshape.forward(&[&state_2])?;
        let state_4 = self.linear_1.forward(&[&state_3])?;
        let state_5 = self.sigmoid_1.forward(&[&state_4])?;
        let state_6 = self.linear_2.forward(&[&state_5])?;
        let state_7 = self.softmax.forward(&[&state_6])?;
        Ok(state_7)
    }

    fn input_shape(&self) -> &[usize] {
        &self.input_shape
    }

    fn output_shape(&self) -> &[usize] {
        &self.output_shape
    }
}
