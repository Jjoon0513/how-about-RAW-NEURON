pub struct Neuron{
    pub inweights: Vec<f32>,
    pub inbias: f32
}

impl Neuron {
    pub fn forward(&self, inputs: &[f32]) -> f32{
        elu(self.inweights.iter()
    .zip(inputs.iter())
    .map(|(w, x)| w * x)
    .sum::<f32>() + self.inbias)
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
    pub fn forward(&self, input: &[f32]) -> f32 {
        self.layers.iter().map(|l| {
            l.forward(input)
        })
    }
}




fn elu(x: f32) -> f32{
    if x > 0.0 {
        x
    } else {
        x.exp() - 1.0
    }
}