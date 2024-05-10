use crate::{Error, Tensor};
mod program;
pub use program::*;

pub trait Model {
    fn forward(&self, inputs: &[&Tensor]) -> Result<Tensor, Error>;
    fn input_shape(&self) -> &[usize];
    fn output_shape(&self) -> &[usize];
}