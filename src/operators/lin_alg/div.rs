use crate::{tensor::Error, tensor::Tensor, DeviceTrait};

pub struct Div {}

impl Div {
    pub fn execute(
        inputs: &[&Tensor],
        outputs: &[&Tensor],
        _execution_unit: usize,
    ) -> Result<(), Error> {
        let input_0 = inputs[0];
        let input_1 = inputs[1];
        let output = outputs[0];
        let device = input_0.device();
        device.div(input_0, input_1, output)
    }
}
