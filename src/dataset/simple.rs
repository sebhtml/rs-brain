use crate::{Activation, DatasetDetails, LayerConfig, Tensor};

use super::mega_man::get_u8_embedding_table;

fn add_embeddings(embedding_table: &Vec<Vec<f32>>, input: &Vec<u8>) -> Tensor {
    let mut values = vec![];
    let mut row = 0;
    let rows = input.len();
    while row < rows {
        let index = input[row];
        values.append(&mut embedding_table[index as usize].clone());
        row += 1;
    }
    Tensor::new(input.len(), embedding_table[0].len(), values)
}

fn load_examples() -> Vec<(Tensor, Tensor)> {
    let embedding_table = get_u8_embedding_table();
    let mut examples = Vec::new();

    examples.push((
        //
        vec![1, 2, 3, 4, 5, 6], //
        vec![1.0, 0.0, 0.0, 0.0],
    ));

    examples.push((
        //
        vec![7, 8, 9, 10, 11, 12], //
        vec![0.0, 0.0, 0.0, 1.0],
    ));

    // TODO instead of manually adding constant embeddings, they should be learned.
    let examples: Vec<(Tensor, Tensor)> = examples
        .into_iter()
        .map(|example| {
            (
                add_embeddings(&embedding_table, &example.0),
                Tensor::new(1, example.1.len(), example.1),
            )
        })
        .collect();

    examples
}

pub fn load_dataset() -> DatasetDetails {
    DatasetDetails {
        examples: load_examples(),
        layers: vec![
            LayerConfig {
                rows: 4,
                cols: 8,
                activation: Activation::Sigmoid,
            },
            LayerConfig {
                rows: 8,
                cols: 4,
                activation: Activation::Sigmoid,
            },
            LayerConfig {
                rows: 4,
                cols: 8,
                activation: Activation::Softmax,
            },
        ],
    }
}
