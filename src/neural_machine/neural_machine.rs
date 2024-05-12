use std::{ops::Deref, rc::Rc};

use crate::{
    BinaryOperator, Clip, Device, Error, Instruction, Model, Operator, Tensor, TensorF32,
    UnaryOperator, Zero,
};

pub struct NeuralMachine {
    example_input: Tensor,
    example_output: Tensor,
    program_output: Tensor,
    loss: Tensor,
    instructions: Vec<Instruction>,
}

impl NeuralMachine {
    pub fn try_new(
        device: &Device,
        model: &(impl UnaryOperator + Model),
        loss_operator: &(impl BinaryOperator + Operator),
    ) -> Result<Self, Error> {
        // input
        let input_shape = model.input_size();
        let input_len = input_shape[0] * input_shape[1];
        let example_input = device.tensor(
            input_shape[0],
            input_shape[1],
            vec![0.7; input_len],
            false,
            false,
        );
        // output
        let output_shape = model.output_size();
        let output_len = output_shape[0] * output_shape[1];
        let example_output = device.tensor(
            output_shape[0],
            output_shape[1],
            vec![0.7; output_len],
            false,
            false,
        );

        let program_output = model.forward(&example_input)?;
        let loss = BinaryOperator::forward(loss_operator, &example_output, &program_output)?;
        let tape = loss.get_tape();
        let mut instructions = vec![];

        for tensor in tape.iter() {
            let instruction = tensor.forward_instructions().deref().borrow()[0].to_owned();
            let outputs: Vec<Tensor> = instruction.outputs().deref().clone().into_iter().collect();
            let outputs: Vec<&Tensor> = outputs.iter().collect();
            let zero_instruction = Instruction::new(Rc::new(Zero::default()), &[], &outputs);
            instructions.push(zero_instruction);
            instructions.push(instruction);
        }

        for tensor in tape.iter().rev() {
            let instruction = tensor.backward_instructions().deref().borrow()[0].to_owned();
            let outputs: Vec<Tensor> = instruction.outputs().deref().clone().into_iter().collect();
            let outputs: Vec<&Tensor> = outputs.iter().collect();
            let min = -1.0;
            let max = 1.0;
            let clip_instruction = Instruction::new(Rc::new(Clip::new(min, max)), &[], &outputs);
            instructions.push(instruction);
            instructions.push(clip_instruction);
        }

        let program = NeuralMachine {
            example_input,
            example_output,
            program_output,
            loss,
            instructions,
        };
        Ok(program)
    }

    pub fn loss(&self) -> Result<Tensor, Error> {
        Ok(self.loss.clone())
    }
}

impl NeuralMachine {
    pub fn forward(&self, input: &Tensor, expected_output: &Tensor) -> Result<Tensor, Error> {
        println!("NeuralMachine forward");
        // Copy input
        {
            let example_input: &mut TensorF32 =
                &mut self.example_input.tensor().deref().borrow_mut();
            let input: &TensorF32 = &input.tensor().deref().borrow_mut();
            TensorF32::copy(input, example_input)?;
        }
        // Copy expected output
        {
            let example_output: &mut TensorF32 =
                &mut self.example_output.tensor().deref().borrow_mut();
            let expected_output: &TensorF32 = &expected_output.tensor().deref().borrow_mut();
            TensorF32::copy(expected_output, example_output)?;
        }
        // Forward tensors
        for (_i, instruction) in self.instructions.iter().enumerate() {
            instruction.forward()?;

            // TODO impl Display
            /*
            println!(
                "{} -> {}, {} inputs, {} outputs",
                i,
                instruction.operator().name(),
                instruction.inputs().len(),
                instruction.outputs().len()
            );
            println!("outputs");

            instruction.forward()?;

            for output in instruction.outputs().deref().iter() {
                let output_t: &TensorF32 = &output.tensor().deref().borrow();
                let output_g: &TensorF32 = &output.gradient().deref().borrow();
                println!("output_t {}", output_t);
                println!("output_g {}", output_g);
            }
             */
        }
        Ok(self.program_output.clone())
    }
}