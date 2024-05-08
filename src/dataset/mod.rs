mod mega_man;
mod mega_man_attention;
mod simple;

use std::{fs, rc::Rc};

use crate::{Device, Error, Identity, OperatorTrait, Tensor, Tokenizer, TokenizerTrait};

pub enum Dataset {
    Simple,
    MegaMan,
    MegaManAttention,
}

pub struct DatasetDetails {
    pub device: Device,
    pub tokenizer: Tokenizer,
    pub examples: Vec<(Tensor, Tensor)>,
    pub model: Box<dyn OperatorTrait>,
    pub loss_function_name: Box<dyn OperatorTrait>,
    pub learning_rate: f32,
    pub epochs: usize,
    pub progress: usize,
    pub initial_total_error_min: f32,
    pub final_total_error_max: f32,
}

pub fn load_dataset(dataset: Dataset, device: &Device) -> Result<DatasetDetails, Error> {
    match dataset {
        Dataset::Simple => simple::load_dataset(device),
        Dataset::MegaMan => mega_man::load_dataset(device),
        Dataset::MegaManAttention => mega_man_attention::load_dataset(device),
    }
}

pub fn into_one_hot_encoded_rows(
    device: &Device,
    input_tokens: &[usize],
    num_classes: usize,
) -> Result<Tensor, Error> {
    debug_assert!(num_classes >= *input_tokens.iter().max().unwrap());
    let len = input_tokens.len() * num_classes;
    // TODO avoid allocating a Tensor and a LearningTensor., gradient  should be a Option in LearningTensor
    let result = device.tensor_f32(
        input_tokens.len(),
        num_classes,
        vec![Default::default(); len],
    );
    let mut result_values = result.get_values()?;
    for (index, token) in input_tokens.iter().enumerate() {
        result_values[result.index(index, *token)] = 1.0;
    }
    Ok(device.tensor(
        Rc::new(Identity::new(device)),
        &vec![],
        input_tokens.len(),
        num_classes,
        result_values,
        false,
        false,
    ))
}

fn load_examples(
    device: &Device,
    file_path: &str,
    input_sequence_length: usize,
    output_sequence_length: usize,
    vocab_size: usize,
    tokenizer: &mut Tokenizer,
) -> Result<Vec<(Tensor, Tensor)>, Error> {
    let mut examples = Vec::new();
    let text = fs::read_to_string(file_path).map_err(|_| Error::UnsupportedOperation)?;
    println!("[load_megaman_examples] loaded {} bytes", text.len());
    let tokens: Vec<usize> = tokenizer.encode(&text);
    println!("[load_megaman_examples] loaded {} tokens", tokens.len());
    let mut i = 0;
    let max_number_of_examples = 10;
    while i + input_sequence_length < tokens.len() && i < max_number_of_examples {
        let input_begin = i + 0;
        let input_end = input_begin + input_sequence_length;
        let input_tokens = &tokens[input_begin..input_end];
        let one_hot_encoded_tokens = into_one_hot_encoded_rows(device, input_tokens, vocab_size)?;
        let output_begin = input_begin + 1;
        let output_end = output_begin + output_sequence_length;
        let next_token = &tokens[output_begin..output_end];
        let output_multiclass = into_one_hot_encoded_rows(device, next_token, vocab_size)?;

        examples.push((
            //
            one_hot_encoded_tokens, //
            output_multiclass,
        ));
        i += 1;
    }
    Ok(examples)
}
