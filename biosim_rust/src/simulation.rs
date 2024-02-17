use std::error::Error;
use rand::{SeedableRng, RngCore};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_chacha::ChaCha8Rng;

use crate::creature::Creature;
use crate::genome::Genome;
use crate::vector2d::Vector2D;

pub type RngSeed = [u8; 32];

pub struct Simulation {
    field_width: usize,
    field_height: usize,
    all_field_pos: Vec<Vector2D<usize>>,

    initial_total_creature: usize,
    total_genome: usize,

    creatures: Vec<Creature>,
    rng: Pcg64
}

impl Simulation {
    pub fn new(field_width: usize, field_height: usize,
        initial_total_creature: usize, seed: RngSeed,
        total_genome: usize) -> Self {

        let mut all_field_pos: Vec<Vector2D<usize>> = vec![];
        for x in 0..field_width {
            for y in 0..field_height {
                all_field_pos.push(Vector2D::new(x, y));
            }
        }

        Self {
            field_width, field_height,
            all_field_pos,
            creatures: vec![],
            initial_total_creature,
            total_genome,
            rng: Pcg64::from_seed(seed)
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut all_possible_coords = self.all_field_pos.choose_multiple(&mut self.rng, self.initial_total_creature);
        let mut new_creature: Creature;
        let mut genome_byte_array = vec![0_u8; self.total_genome];

        let current_gen_seed = self.rng.next_u64();
        let mut creature_rng;

        for i in 0..self.initial_total_creature {
            creature_rng = ChaCha8Rng::seed_from_u64(current_gen_seed);
            creature_rng.set_stream(i as u64);
            self.rng.fill_bytes(&mut genome_byte_array);

            new_creature = Creature::new(
                *all_possible_coords.next().unwrap(),
                Genome::from_byte_slice(&genome_byte_array),
                creature_rng
            )?;

            self.creatures.push(new_creature);
        }

        Ok(())
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_field_width() {
        let sim = Simulation::new(100,100,20,[0; 32], 4);
        assert_eq!(sim.field_width, 100);
    }

    #[test]
    fn get_field_height() {
        let sim = Simulation::new(100,100,20,[0; 32], 4);
        assert_eq!(sim.field_height, 100);
    }
}
