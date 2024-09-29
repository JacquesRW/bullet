mod dense_matrix;
mod operations;
mod shape;

use dense_matrix::DenseMatrix;
pub use operations::{Activation, Operation};
pub use shape::Shape;

use crate::backend::ExecutionContext;

impl From<Tensor> for Shape {
    fn from(value: Tensor) -> Self {
        value.values.shape()
    }
}

#[derive(Debug, Default)]
pub struct Tensor {
    pub(crate) values: DenseMatrix,
    pub(crate) gradients: Option<DenseMatrix>,
}

impl diffable::Tensor for Tensor {
    type ModelOfTensor = Shape;
    type ExecutionContext = ExecutionContext;
    type DiffableOperation = Operation;

    fn new(shape: Shape, requires_grad: bool) -> Self {
        Self {
            values: DenseMatrix::zeroed(shape),
            gradients: if requires_grad { Some(DenseMatrix::zeroed(shape)) } else { None },
        }
    }

    fn zero_grad(&mut self) {
        if let Some(grad) = self.gradients.as_mut() {
            grad.set_zero();
        }
    }

    fn copy_values_into(&self, dest: &mut Self) {
        self.values.copy_into(&mut dest.values);
    }

    fn get_scalar(&self) -> Option<f32> {
        if self.values.shape() == Shape::new(1, 1) {
            let mut buf = [0.0];
            self.values.write_to_slice(&mut buf);
            Some(buf[0])
        } else {
            None
        }
    }

    fn set_grad_to_unit(&mut self) {
        let grad = self.gradients.as_mut().unwrap();
        grad.load_from_slice(Shape::new(1, 1), &[1.0]);
    }
}
