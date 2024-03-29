use crate::{
    add_embeddings, get_u8_embedding_table, loss::LossFunctionName, Activation, DatasetDetails,
    LayerConfig, Tensor,
};

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
                rows: 256,
                cols: 256,
                activation: Activation::Sigmoid,
            },
            LayerConfig {
                rows: 256,
                cols: 256,
                activation: Activation::Sigmoid,
            },
            LayerConfig {
                rows: 4,
                cols: 256,
                activation: Activation::Softmax,
            },
        ],
        epochs: 1000,
        progress: 10,
        loss_function_name: LossFunctionName::CrossEntropyLoss,
    }
}
