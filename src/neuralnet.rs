//! Neural network using genetic algorithms.

use crate::math::{self, Matrix};

/// Simple neural network with fixed topology.
#[derive(Debug, Clone, Default)]
pub struct NeuralNetwork<const INPUTS: usize, const HIDDEN: usize, const OUTPUTS: usize> {
    hidden_layer_in: Matrix<f32, INPUTS, HIDDEN>,
    hidden_layer_out: Matrix<f32, HIDDEN, OUTPUTS>,
}

impl<const INPUTS: usize, const HIDDEN: usize, const OUTPUTS: usize>
    NeuralNetwork<INPUTS, HIDDEN, OUTPUTS>
{
    /// Creates new `NeuralNetwork` according to input and output size.
    pub fn new() -> Self {
        Self {
            hidden_layer_in: Matrix::with_random(-1.0, 1.0),
            hidden_layer_out: Matrix::with_random(-1.0, 1.0),
        }
    }

    /// Feeds the neural network with the input, producing an ouput matrix with only one column and
    /// as many rows as requested outputs.
    pub fn feed(&self, input: &Matrix<f32, 1, INPUTS>) -> Matrix<f32, 1, OUTPUTS> {
        let mut a = input.clone() * &self.hidden_layer_in;
        Self::add_bias(&mut a);
        Self::activate(&mut a);

        let mut res = a * &self.hidden_layer_out;
        Self::add_bias(&mut res);
        Self::activate(&mut res);

        res
    }

    /// Crossovers two neural networks in order to produce a new child.
    pub fn crossover(&self, other: &Self) -> Self {
        let hidden_layer_in = self.hidden_layer_in.crossover(&other.hidden_layer_in);
        let hidden_layer_out = self.hidden_layer_out.crossover(&other.hidden_layer_out);

        Self {
            hidden_layer_in,
            hidden_layer_out,
        }
    }

    /// Randomly mutates weights.
    pub fn mutate(&mut self) {
        const PROBABILITY: f32 = 0.05;
        math::mutate_matrixf(&mut self.hidden_layer_in, PROBABILITY);
        math::mutate_matrixf(&mut self.hidden_layer_out, PROBABILITY);
    }

    fn add_bias<const R: usize, const C: usize>(layer: &mut Matrix<f32, R, C>) {
        let bias = Matrix::with_val(1.0);
        *layer += &bias;
    }

    fn activate<const R: usize, const C: usize>(layer: &mut Matrix<f32, R, C>) {
        layer.apply(math::sigmoid);
    }
}
