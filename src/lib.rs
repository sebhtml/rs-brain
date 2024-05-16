mod training;
pub use training::*;
mod tensor;
pub use tensor::*;
mod models;
pub use models::*;
mod devices;
pub use devices::*;
mod operators;
pub use operators::*;
mod tokenizers;
pub use tokenizers::*;
mod neural_machine;
pub use neural_machine::*;

const EPSILON: f32 = 1e-8;
