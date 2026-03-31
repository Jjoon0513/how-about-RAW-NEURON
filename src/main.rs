use crate::nn::neuron::{
    Neuron,
    Layer,
    Network
};

mod nn;
mod math;

fn main() {
    let network = Network {
        layers: vec![
            Layer {
                neurons: vec![
                    Neuron { weights: vec![0.5], bias: 0.1 },
                    Neuron { weights: vec![0.3], bias: 0.2 },
                    Neuron { weights: vec![0.7], bias: 0.0 },
                ]
            },
            Layer {
                neurons: vec![
                    Neuron { weights: vec![0.4, 0.6, 0.2], bias: 0.1 },
                    Neuron { weights: vec![0.8, 0.1, 0.5], bias: 0.3 },
                ]
            },
            Layer {
                neurons: vec![
                    Neuron { weights: vec![0.3, 0.7], bias: 0.0 },
                ]
            },
        ]
    };

    let input = vec![0.5];
    let output = network.forward(&input);
    println!("{:?}", output);
}
