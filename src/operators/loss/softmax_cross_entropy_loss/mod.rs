use crate::{
    devices::Device, gradient_instruction, loss_instruction, tensor::Error, tensor::Tensor,
    BinaryOperator, DeviceInterface, OpCode, TensorWithGrad,
};

#[derive(Clone)]
pub struct SoftmaxCrossEntropyLoss {
    device: Device,
}

impl SoftmaxCrossEntropyLoss {
    pub fn new(device: &Device) -> Self {
        Self {
            device: device.clone(),
        }
    }

    pub fn execute(inputs: &[&Tensor], outputs: &[&Tensor]) -> Result<(), Error> {
        let expected = inputs[0];
        let actual = inputs[1];
        let loss = outputs[0];
        let device = expected.device();
        device.cross_entropy_loss(expected, actual, loss)
    }
}

impl BinaryOperator for SoftmaxCrossEntropyLoss {
    fn forward(
        &self,
        input_1: &TensorWithGrad,
        input_2: &TensorWithGrad,
    ) -> Result<TensorWithGrad, Error> {
        let output =
            self.device
                .tensor_with_grad(1, 1, vec![0.0], &[input_1, input_2], true, false)?;
        let inputs = [input_1, input_2];
        let outputs = [&output];
        let zero = self.device.tensor(1, 1, vec![0.0])?;
        output.push_instruction(loss_instruction!(
            OpCode::ScalarMul,
            &[&zero, &outputs[0].tensor().read().unwrap()],
            &[&outputs[0].tensor().read().unwrap()],
        ));
        output.push_instruction(loss_instruction!(
            OpCode::ScalarMul,
            &[&zero, &outputs[0].gradient().read().unwrap()],
            &[&outputs[0].gradient().read().unwrap()],
        ));
        output.push_instruction(loss_instruction!(
            OpCode::SoftmaxCrossEntropyLoss,
            &[
                &inputs[0].tensor().read().unwrap(),
                &inputs[1].tensor().read().unwrap(),
            ],
            &[&outputs[0].tensor().read().unwrap()],
        ));
        let inputs = [input_1, input_2];
        let outputs = [input_2];

        let inputs: &[&Tensor] = &[
            &inputs[0].tensor().read().unwrap(),
            &inputs[1].tensor().read().unwrap(),
        ];
        let outputs: &[&Tensor] = &[&outputs[0].gradient().read().unwrap()];

        debug_assert_eq!(inputs.len(), 2);
        debug_assert_eq!(outputs.len(), 1);

        // When Cross-Entropy Loss is used with a Softmax activation function,
        // then we don't need to derive the softmax activations.
        // The derivative of the Loss in respect to logits (before activation) is
        // output of the softmax function - expected output (one-hot encoded)
        if outputs[0].requires_grad() {
            let output_gradient = outputs[0];
            let expected = inputs[0];
            let actual = inputs[1];
            output.push_instruction(gradient_instruction!(
                OpCode::Sub,
                &[actual, expected],
                &[output_gradient],
            ));
        }

        Ok(output)
    }
}
