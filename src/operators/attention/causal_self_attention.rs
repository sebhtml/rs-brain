use std::ops::Deref;

use crate::{
    BinaryOperator, Device, Error, Mask, MatMul, Scale, Softmax, Tensor, TernaryOperator,
    UnaryOperator,
};

/// MaskedScaledDotProductAttention is not a ONNX operator.
/// https://onnx.ai/onnx/operators/index.html ???
/// Attention Is All You Need
/// https://arxiv.org/abs/1706.03762
#[derive(Clone)]
pub struct ScaledDotProductAttention {
    qk_matmul: MatMul,
    scale: Scale,
    mask: Mask,
    softmax: Softmax,
    matmul: MatMul,
}

impl ScaledDotProductAttention {
    pub fn try_new(device: &Device, rows: usize, cols: usize) -> Result<Self, Error> {
        let qk_matmul = MatMul::new(device, true);
        let alpha = 1.0 / f32::sqrt(cols as f32);
        let scale = Scale::new(device, alpha);
        let mask_rows = rows;
        let mask_cols = rows;
        let mask = Mask::try_new(device, mask_rows, mask_cols)?;
        let next_op_is_cross_entropy_loss = false;
        let softmax = Softmax::new(device, next_op_is_cross_entropy_loss);
        let matmul = MatMul::new(device, false);

        let attention = Self {
            qk_matmul,
            scale,
            mask,
            softmax,
            matmul,
        };
        Ok(attention)
    }
}

impl TernaryOperator for ScaledDotProductAttention {
    fn forward(&self, q: &Tensor, k: &Tensor, v: &Tensor) -> Result<Tensor, Error> {
        let weights = self.qk_matmul.forward(q, k)?;
        let scaled_weights = self.scale.forward(&weights)?;
        let masked_weights = self.mask.forward(&scaled_weights)?;
        let softmaxed_weights = self.softmax.forward(&masked_weights)?;
        let attentions = self.matmul.forward(&softmaxed_weights, v)?;
        Ok(attentions)
    }
}
