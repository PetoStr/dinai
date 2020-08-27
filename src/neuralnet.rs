//! Neural network using genetic algorithms.

use crate::math::{self, Matrixf};

/// Simple neural network with fixed topology.
#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    hidden_layer_in: Matrixf,
    hidden_layer_out: Matrixf,
}

impl NeuralNetwork {
    /// Creates new `NeuralNetwork` according to input and output size.
    pub fn new(inputs: usize, outputs: usize) -> Self {
        Self {
            hidden_layer_in: Matrixf::with_random(inputs, inputs + 1, -1.0, 1.0),
            hidden_layer_out: Matrixf::with_random(inputs + 1, outputs, -1.0, 1.0),
        }
    }

    /// Feeds the neural network with the input, producing an ouput matrix with only one column and
    /// as many rows as requested outputs.
    pub fn feed(&self, input: &Matrixf) -> Matrixf {
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
        const PROBABILITY: f32 = 0.15;
        self.hidden_layer_in.mutate(PROBABILITY);
        self.hidden_layer_out.mutate(PROBABILITY);
    }

    fn add_bias(layer: &mut Matrixf) {
        let bias = Matrixf::with_val(1.0, layer.rows(), layer.columns());
        *layer += &bias;
    }

    fn activate(layer: &mut Matrixf) {
        layer.apply(math::sigmoid);
    }
}
