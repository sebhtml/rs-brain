use crate::{
    devices::Device, error, gradient_instruction, inference_instruction, tensor::Error,
    tensor::ErrorEnum, tensor::Tensor, BinaryOperator, OpCode, TensorWithGrad,
};

pub struct MatMul {
    device: Device,
    transb: bool,
}

impl MatMul {
    pub fn new(device: &Device, transb: bool) -> Self {
        MatMul {
            device: device.clone(),
            transb,
        }
    }
}

impl BinaryOperator for MatMul {
    fn forward(
        &self,
        input_0: &TensorWithGrad,
        input_1: &TensorWithGrad,
    ) -> Result<TensorWithGrad, Error> {
        let input_0_tensor: &Tensor = &input_0.tensor().read().unwrap();
        let input_1_tensor: &Tensor = &input_1.tensor().read().unwrap();
        let compatible = match self.transb {
            false => input_0_tensor.cols() == input_1_tensor.rows(),
            true => input_0_tensor.cols() == input_1_tensor.cols(),
        };
        if !compatible {
            println!("Incompatible shapes in matrix multiplication");
            println!("transa: {}, transb: {}", false, self.transb);
            println!(
                "Between A {:?} and B {:?}",
                *input_0_tensor.size(),
                *input_1_tensor.size(),
            );
            debug_assert!(false);
            return Err(error!(ErrorEnum::IncompatibleTensorShapes));
        }

        let rows = input_0_tensor.rows();
        let transb = self.transb;
        let cols = if transb {
            input_1_tensor.rows()
        } else {
            input_1_tensor.cols()
        };
        let len = rows * cols;
        let output = self.device.tensor_with_grad(
            rows,
            cols,
            vec![0.0; len],
            &[input_0, input_1],
            true,
            false,
        )?;

        let inputs = [input_0, input_1];
        let outputs = [&output];
        let zero = self.device.tensor(1, 1, vec![0.0])?;
        output.push_instruction(inference_instruction!(
            OpCode::ScalarMul,
            &[&zero, &outputs[0].tensor().read().unwrap()],
            &[&outputs[0].tensor().read().unwrap()],
        ));
        output.push_instruction(inference_instruction!(
            OpCode::ScalarMul,
            &[&zero, &outputs[0].gradient().read().unwrap()],
            &[&outputs[0].gradient().read().unwrap()],
        ));
        output.push_instruction(inference_instruction!(
            OpCode::Gemm(false, transb, false),
            &[
                &inputs[0].tensor().read().unwrap(),
                &inputs[1].tensor().read().unwrap(),
                &outputs[0].tensor().read().unwrap(),
            ],
            &[&outputs[0].tensor().read().unwrap()],
        ));

        if input_1.gradient().read().unwrap().requires_grad() {
            output.push_instruction(gradient_instruction!(
                OpCode::Gemm(true, false, transb),
                &[
                    &input_0.tensor().read().unwrap(),
                    &output.gradient().read().unwrap(),
                    &input_1.gradient().read().unwrap(),
                ],
                &[&input_1.gradient().read().unwrap()],
            ));
        }

        if input_0.gradient().read().unwrap().requires_grad() {
            output.push_instruction(gradient_instruction!(
                OpCode::Gemm(false, !transb, false),
                &[
                    &output.gradient().read().unwrap(),
                    &input_1.tensor().read().unwrap(),
                    &input_0.gradient().read().unwrap(),
                ],
                &[&input_0.gradient().read().unwrap()],
            ));
        }

        Ok(output)
    }
}
