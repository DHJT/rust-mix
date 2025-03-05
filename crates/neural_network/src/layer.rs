// [使用Rust从零构建神经网络：性能与精度的完美结合](https://mp.weixin.qq.com/s/wnmdbF9hYFq55veusS7Fdg)
use rand::Rng;
// 定义神经网络单层结构
#[derive(Debug)]
pub(crate) struct Layer {
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize) -> Layer {
        let mut rng = rand::thread_rng();

        // 用随机值初始化权重和偏置
        let weights = (0..output_size)
            .map(|_| (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();

        let biases = (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect();

        Layer { weights, biases }
    }

    pub(crate) fn forward(&self, input: &[f64]) -> Vec<f64> {
        self.weights.iter().enumerate().map(|(i, neuron_weights)| {

            let sum: f64 = neuron_weights.iter().zip(input.iter())
                .map(|(w, i)| w * i)
                .sum();

            sigmoid(sum + self.biases[i])
        }).collect()
    }

    pub(crate) fn backward(&mut self, input: &[f64], error: &[f64], learning_rate: f64) -> Vec<f64> {
        let mut input_error = vec![0.0; input.len()];

        for (i, neuron_weights) in self.weights.iter_mut().enumerate() {
            for (j, weight) in neuron_weights.iter_mut().enumerate() {
                input_error[j] += *weight * error[i];
                *weight -= learning_rate * error[i] * input[j];
            }
            self.biases[i] -= learning_rate * error[i];
        }
        input_error
    }

}

// Sigmoid 激活函数
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// 定义均方误差（MSE）计算函数
pub fn mean_squared_error(predicted: &[f64], actual: &[f64]) -> f64 {
    predicted.iter()
        .zip(actual.iter())
        .map(|(p, a)| (p - a).powi(2))
        .sum::<f64>() / predicted.len() as f64
}