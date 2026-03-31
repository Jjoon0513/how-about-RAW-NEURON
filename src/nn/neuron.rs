use crate::math::functions::relu;

pub struct Neuron{
    pub weights: Vec<f32>,
    pub bias: f32
}

impl Neuron {
    pub fn forward(&self, inputs: &[f32]) -> f32{
        relu(self.weights.iter()
    .zip(inputs.iter())
    .map(|(w, x)| w * x)
    .sum::<f32>() + self.bias)
    }
}

pub struct Layer {
    pub neurons: Vec<Neuron>
}

impl Layer {
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        self.neurons.iter().map(|n|
            n.forward(input)
        ).collect()
    }
}

pub struct Network {
    pub layers: Vec<Layer>
}

impl Network {
    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        self.layers.iter().fold(input.to_vec(), |acc, l| {
            l.forward(&acc)
        })
    }
}




