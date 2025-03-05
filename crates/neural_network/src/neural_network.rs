use crate::layer::Layer;

// 定义神经网络结构
#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {

    pub fn new(layer_sizes: &[usize]) -> NeuralNetwork {

        let layers = layer_sizes.windows(2)
            .map(|w| Layer::new(w[0], w[1]))
            .collect();

        NeuralNetwork { layers }
    }

    pub fn forward(&self, input: &[f64]) -> Vec<f64> {
        self.layers.iter().fold(input.to_vec(), |acc, layer| layer.forward(&acc))
    }

    pub fn backward(&mut self, inputs: &[f64], target: &[f64], learning_rate: f64) {
        let mut layer_inputs = vec![inputs.to_vec()];
        let mut current_input = inputs.to_vec();

        for layer in &self.layers {
            current_input = layer.forward(&current_input);
            layer_inputs.push(current_input.clone());
        }

        let error = layer_inputs.last().unwrap()
            .iter()
            .zip(target.iter())
            .map(|(o, t)| o - t)
            .collect::<Vec<_>>();
        let mut current_error = error;

        for (layer, inputs) in self.layers.iter_mut().rev().zip(layer_inputs.iter().rev().skip(1)) {
            current_error = layer.backward(inputs, &current_error, learning_rate);
        }
    }

}