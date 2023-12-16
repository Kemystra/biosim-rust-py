use std::cmp::Ordering;


#[derive(Debug)]
pub struct InternalNeuron {
    tmp_value: f64
}

impl InternalNeuron {
    // Uses Leaky ReLU
    pub fn activation_func(&mut self, input: f64) -> f64 {
        let total_input = input + self.tmp_value;
        let value = match total_input.total_cmp(&0_f64) {
            Ordering::Greater => total_input,
            _ => total_input * 0.01
        };

        self.tmp_value = value;
        return value;
    }
}
