use rand::SeedableRng;

use crate::renderer::Color;
use crate::neuron::Brain;


pub struct Genome(Vec<u16>);

impl Genome {
    // Combine 2 bytes, and collect
    pub fn new(bytes: &[u8]) -> Self {
        let result = vec![];
        let n = 0;
        let gene: u16;

        for i in 0..(bytes.len() / 2) {
            n = i*2;
            gene = (bytes[n] as u16) | (bytes[n+1] as u16) << 8;
            result.push(gene);
        }

        Genome(result)
    }

    pub fn generate_color(&self) -> Color {

    }
/*
    pub fn generate_brain(&self) -> Brain {

    }

    pub fn replicate<R: SeedableRng>(&mut self, rng: R) -> Self {

    }
*/
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
