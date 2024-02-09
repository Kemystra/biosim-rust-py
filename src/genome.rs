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

    // XOR the hell out of it until a single u16 is left
    // And then multiply by 2^8 to expand into 24bit color
    pub fn generate_color(&self) -> Result<Color, GenomeError> {
        // Yes, I have to own the value first
        let mut val: u32 = self.0.iter().map(|x| *x).reduce(|acc, e| {
            acc ^ e
        })
            .ok_or(GenomeError::EmptyGenome)?
            .into();

        // Shift that stuff 4 bit forward, then OR it with other 8 bits somewhere
        // Janky but bruh
        let front_bits = (val & 0x00_00_F0_00) << 8;
        let back_bits = val & 0x00_00_00_0F;
        val = (val << 4) | front_bits | back_bits;

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
        assert_eq!(color, Color::from_xrgb_u32(2253824));
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
