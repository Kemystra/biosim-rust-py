use std::cmp::Ordering;


#[derive(Debug, Clone, Copy)]
pub struct InternalNeuron {
    state: f64
}

impl InternalNeuron {
    pub fn new() -> Self {
        Self {
            state: 0.0
        }
    }

    // Uses Leaky ReLU
    pub fn activation_func(&mut self, input: f64) -> f64 {
        let value = match input.total_cmp(&0_f64) {
            Ordering::Greater => input,
            _ => input * 0.01
        };

        self.state = value;
        return value;
    }
}
