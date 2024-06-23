mod model;
pub use model::*;
use more_asserts::debug_assert_lt;
pub mod mega_man;
pub mod multi_head_attention_model;
pub mod perceptron;
pub mod simple;
pub mod transformer_model;
use crate::{error, new_tensor, new_tensor_with_grad, BinaryOperator, Metrics, OptimizerTrait};
use std::fs;

use crate::{tensor::Error, tensor::ErrorEnum, Device, TensorWithGrad, Tokenizer, TokenizerTrait};

pub struct ModelDetails<Model, LossOperator, Optimizer>
where
    Model: UnaryModel,
    LossOperator: BinaryOperator,
    Optimizer: OptimizerTrait,
{
    pub device: Device,
    pub tokenizer: Option<Tokenizer>,
    pub examples: Vec<(TensorWithGrad, TensorWithGrad)>,
    pub model: Model,
    pub loss_operator: LossOperator,
    pub optimizer: Optimizer,
    pub learning_rate: f32,
    pub shuffle_examples: bool,
    pub clipped_gradient_norm: bool,
    pub epochs: usize,
    pub progress: usize,
    pub initial_metrics: Metrics,
    pub final_metrics: Metrics,
}

pub fn into_one_hot_encoded_rows(
    device: &Device,
    input_tokens: &[usize],
    num_classes: usize,
) -> Result<TensorWithGrad, Error> {
    debug_assert_lt!(*input_tokens.iter().max().unwrap(), num_classes);
    let len = input_tokens.len() * num_classes;
    let result = new_tensor!(
        device,
        input_tokens.len(),
        num_classes,
        vec![Default::default(); len],
    )?;
    let mut result_values = result.get_values()?;
    for (index, token) in input_tokens.iter().enumerate() {
        result_values[result.index(index, *token)] = 1.0;
    }
    new_tensor_with_grad!(
        device,
        input_tokens.len(),
        num_classes,
        result_values,
        &[],
        false,
        false,
    )
}

fn load_examples(
    device: &Device,
    file_path: &str,
    max_chars: Option<usize>,
    max_number_of_examples: usize,
    input_sequence_length: usize,
    output_sequence_length: usize,
    tokenizer: &mut Tokenizer,
) -> Result<Vec<(TensorWithGrad, TensorWithGrad)>, Error> {
    let mut examples = Vec::new();
    let mut text =
        fs::read_to_string(file_path).map_err(|_| error!(ErrorEnum::IncompatibleTensorShapes))?;
    if let Some(max_chars) = max_chars {
        text = text[0..max_chars].to_owned();
    }
    println!("[load_megaman_examples] loaded {} bytes", text.len());
    let tokens: Vec<usize> = tokenizer.encode(&text);
    let vocab_size = tokenizer.vocab_size();
    println!("[load_megaman_examples] loaded {} tokens", tokens.len());
    let mut i = 0;
    while i + input_sequence_length < tokens.len() && i < max_number_of_examples {
        let input_begin = i;
        let input_end = input_begin + input_sequence_length;
        let input_tokens = &tokens[input_begin..input_end];
        let one_hot_encoded_tokens = into_one_hot_encoded_rows(device, input_tokens, vocab_size)?;
        let output_begin = input_begin + 1;
        let output_end = output_begin + output_sequence_length;
        let output_tokens = &tokens[output_begin..output_end];
        let output_multiclass = into_one_hot_encoded_rows(device, output_tokens, vocab_size)?;

        examples.push((
            //
            one_hot_encoded_tokens, //
            output_multiclass,
        ));
        i += 1;
    }
    Ok(examples)
}
