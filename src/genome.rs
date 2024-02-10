use rand::SeedableRng;
use thiserror::Error;

use crate::renderer::Color;
use crate::neuron::Brain;


pub struct Genome(Vec<u16>);

impl Genome {
    // Combine 2 bytes, and collect
    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        let mut result = vec![];
        let mut gene: u16;

        for i in 0..bytes.len() {
            if i % 2 == 1 { continue }

            gene = (bytes[i] as u16) | ((bytes[i+1] as u16) << 8);
            result.push(gene);
        }

        Genome(result)
    }

    // XOR the hell out of it until a u32 is left
    pub fn generate_color(&self) -> Result<Color, GenomeError> {
        let mut val: u32 = (self.0[0] as u32) | ((self.0[1] as u32) << 16);

        for i in 2..self.0.len() {
            if i % 2 == 1 { continue }

            val ^= (self.0[i] as u32) | ((self.0[i+1] as u32) << 16);
        }

        Ok(Color::from_xrgb_u32(val))
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

#[derive(Debug, Error)]
pub enum GenomeError {
    #[error("Genome is empty")]
    EmptyGenome
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_genome() {
        let bytes: [u8; 4] = [100, 34, 90, 210];
        let genome = Genome::from_byte_slice(&bytes);

        let color = genome.generate_color().unwrap();
        assert_eq!(color, Color::new(90, 34, 100));
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
