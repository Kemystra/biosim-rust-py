use rand::SeedableRng;
use thiserror::Error;

use crate::renderer::Color;
use crate::neuron::Brain;

pub type Gene = u16;
pub struct Genome(Vec<Gene>);

// Reminder: Genome uses little-endian ordering
impl Genome {
    // Combine 2 bytes, and collect
    pub fn from_byte_slice(bytes: &[u8]) -> Self {
        let mut result = vec![];
        let mut gene: Gene;

        for i in 0..bytes.len() {
            if i % 2 == 1 { continue }

            gene = (bytes[i] as Gene) | ((bytes[i+1] as Gene) << 8);
            result.push(gene);
        }

        Genome(result)
    }

    pub fn genes(&self) -> &Vec<Gene> {
        &self.0
    }

    // XOR the hell out of it until a u32 is left
    // What's the endianness of each Gene? Just gonna make it little-endian
    pub fn generate_color(&self) -> Result<Color, GenomeError> {
        let mut val: u32 = (self.0[0] as u32) | ((self.0[1] as u32) << 16);

        for i in 2..self.0.len() {
            if i % 2 == 1 { continue }

            // If Genome has odd-numbered genes
            // XOR with 0
            // Yes some deref bullshit there
            val ^= (self.0[i] as u32) | ((self.0.get(i+i).map_or(0, |x| *x) as u32) << 16);
        }

        Ok(Color::from_xrgb_u32(val))
    }
/*
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
    fn mutate_genome() {
        unimplemented!();
    }
}
