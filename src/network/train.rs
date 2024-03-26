use crate::{load_dataset, Dataset, Network, Tensor, TrainWorkingMemory};

fn print_total_error(
    network: &Network,
    inputs: &Vec<Tensor>,
    outputs: &Vec<Tensor>,
    last_total_error: f32,
    epoch: usize,
) -> f32 {
    let total_error = network.total_error(inputs, outputs);
    let change = (total_error - last_total_error) / last_total_error;
    println!(
        "Epoch {} Total_error {}, change: {}",
        epoch, total_error, change
    );
    total_error
}

pub struct NetworkTestOutput {
    pub initial_total_error: f32,
    pub final_total_error: f32,
}

pub fn train_network_on_dataset(dataset: &Dataset) -> NetworkTestOutput {
    let mut train_working_memory = TrainWorkingMemory::default();
    let mut initial_total_error = f32::NAN;
    let dataset_details = load_dataset(dataset);
    let examples = dataset_details.examples;
    let layers = dataset_details.layers;

    let mut network = Network::new(layers);

    let inputs = examples.iter().map(|x| x.clone().0).collect();
    let outputs = examples.iter().map(|x| x.clone().1).collect();

    let mut last_total_error = f32::NAN;
    let epochs = dataset_details.epochs;
    let progress = dataset_details.progress;
    for epoch in 0..epochs {
        if epoch % progress == 0 {
            let total_error =
                print_total_error(&network, &inputs, &outputs, last_total_error, epoch);
            if epoch == 0 {
                initial_total_error = total_error;
            }
            last_total_error = total_error;
        }
        network.train(&mut train_working_memory, epoch, &inputs, &outputs);
    }
    let final_total_error =
        print_total_error(&network, &inputs, &outputs, last_total_error, epochs);

    let predictions = network.predict_many(&inputs);

    for i in 0..inputs.len() {
        let output = &outputs[i];
        let prediction = &predictions[i];
        println!("Example {}", i);
        println!("Expected {}", output);
        println!("Actual   {}", prediction);
    }

    NetworkTestOutput {
        initial_total_error,
        final_total_error,
    }
}