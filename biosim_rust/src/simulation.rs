use std::cell::{RefCell, Ref};
use std::collections::HashMap;
use std::error::Error;
use rand::{SeedableRng, RngCore};
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;

use crate::creature::{Creature, CreatureRng};
use crate::genome::Genome;
use crate::vector2d::Vector2D;

pub type RngSeed = [u8; 32];

pub struct Simulation {
    field_width: usize,
    field_height: usize,
    occupation_map: HashMap<Vector2D<usize>, bool>,
    all_field_position: Vec<Vector2D<usize>>,

    initial_total_creature: usize,
    total_genes: usize,

    creatures: Vec<RefCell<Creature>>,
    rng: Pcg64
}

impl Simulation {
    pub fn new(field_width: usize, field_height: usize,
        initial_total_creature: usize, seed: RngSeed,
        total_genes: usize) -> Self {

        let mut occupation_map = HashMap::new();
        let mut all_field_position = vec![];
        let mut pos: Vector2D<usize>;
        for x in 0..field_width {
            for y in 0..field_height {
                pos = Vector2D::new(x, y);
                all_field_position.push(pos);
                occupation_map.insert(pos, false);
            }
        }

        Self {
            field_width, field_height,
            occupation_map,
            all_field_position,
            creatures: vec![],
            initial_total_creature,
            total_genes,
            rng: Pcg64::from_seed(seed)
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        let mut all_possible_coords = self.all_field_position
            .choose_multiple(&mut self.rng, self.initial_total_creature);

        let mut new_creature: Creature;
        // Gene is u16, so you need 2 u8 for each Gene
        let mut genome_byte_array = vec![0_u8; self.total_genes * 2];

        let current_gen_seed = self.rng.next_u64();
        let mut creature_rng;
        let mut current_position: Vector2D<usize>;

        for i in 0..self.initial_total_creature {
            creature_rng = CreatureRng::seed_from_u64(current_gen_seed);
            creature_rng.set_stream(i as u64);
            self.rng.fill_bytes(&mut genome_byte_array);

            current_position = *all_possible_coords.next().unwrap();
            self.occupation_map.insert(current_position, true);

            new_creature = Creature::new(
                current_position,
                Genome::from_byte_slice(&genome_byte_array),
                creature_rng
            )?;

            self.creatures.push(RefCell::new(new_creature));
        }

        Ok(())
    }

    pub fn step(&mut self) -> () {
        for creature_ref in &self.creatures {
            let mut creature = creature_ref.borrow_mut();
            creature.gather_sensory_data(self);
            creature.think();
            creature.execute_actions(self);
        }
    }

    pub fn creatures_iter(&self) -> impl Iterator<Item=Ref<Creature>> {
        self.creatures.iter().map(|c| c.borrow())
    }

    pub fn field_width(&self) -> usize {
        self.field_width
    }

    pub fn field_height(&self) -> usize {
        self.field_height
    }

    pub fn is_position_occupied(&self, pos: &Vector2D<usize>) -> bool {
        self.occupation_map.get(pos).is_some_and(|x| *x)
    }
}

pub enum Signal {
    PositionChanged { old: Vector2D<usize>, new: Vector2D<usize> }
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

    #[test]
    fn see_position_occupation() {
        let creature = Creature::new(
            Vector2D::new(100, 100),
            Genome::from_byte_slice(&[0; 10]),
            CreatureRng::from_entropy()
        ).unwrap();

        let mut sim = Simulation::new(200, 200, 1, [0;32], 4);
        sim.creatures.push(RefCell::new(creature));
        sim.occupation_map.insert(Vector2D::new(100, 100), true);

        println!("{:?}", sim.creatures[0].borrow().position());

        assert_eq!(sim.is_position_occupied(&Vector2D::new(100, 100)), true);
        assert_eq!(sim.is_position_occupied(&Vector2D::new(10, 10)), false);
    }
}
