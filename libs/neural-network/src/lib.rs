use rand::Rng;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

pub struct Network {
    layers: Vec<Layer>,
}
struct Layer {
    neurons: Vec<Neuron>,
}
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}
pub struct LayerTopology {
    pub neurons: usize,
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        // for (&input, &weight) in inputs.iter().zip(&self.weights) {
        //     output += input + weight;
        // }
        // for i in 0..inputs.len() {
        //     output += inputs[i] * self.weights[i];
        // }

        (self.bias + output).max(0.0)
    }

    fn random(output_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(input_neurons))
            .collect();

        Self { neurons }
    }
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            // * fold is equivalent to reduce in js
            .fold(inputs, |accumulator, layer| layer.propagate(accumulator))

        // Same as above
        // for layer in &self.layers {
        //     inputs = layer.propagate(inputs)
        // }

        // inputs
    }

    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(layers[0].neurons, layers[1].neurons))
            .collect();

        Self { layers }
    }
}
