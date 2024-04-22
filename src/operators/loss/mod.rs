use crate::{devices::Device, Error, Tensor};
mod residual_sum_of_squares;
pub use residual_sum_of_squares::*;
mod cross_entropy_loss;
pub use cross_entropy_loss::*;

pub trait LossFunction {
    fn evaluate(
        &self,
        accelerator: &Device,
        expected: &Tensor,
        actual: &Tensor,
    ) -> Result<f32, Error>;
    fn derive(
        &self,
        accelerator: &Device,
        expected: &Tensor,
        actual: &Tensor,
        result: &mut Tensor,
    ) -> Result<(), Error>;
}
