use rand::SeedableRng;

use crate::renderer::Color;
use crate::neuron::Brain;


pub struct Genome(Vec<u16>);

impl Genome {
    pub fn generate_color(&self) -> Color {

    }

    pub fn generate_brain(&self) -> Brain {

    }

    pub fn replicate<R: SeedableRng>(&mut self, rng: R) -> Self {

    }

    fn randomly_mutate<R: SeedableRng>(&mut self, rng: R) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_genome() {
        unimplemented!();
    }

    #[test]
    fn brain_from_genome() {
        unimplemented!();
    }

    #[test]
    fn mutate_genome() {
        unimplemented!();
    }
}
