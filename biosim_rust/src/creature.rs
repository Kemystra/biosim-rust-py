use std::error::Error;
use rand::Rng;
use std::collections::HashMap;
use rand_chacha::ChaCha8Rng;

use crate::genome::Genome;
use crate::renderer::Color;
use crate::neuron::{Brain, sensory_neuron, action_neuron};
use sensory_neuron::SensoryNeuron;
use action_neuron::ActionNeuron;
use crate::simulation::{Signal, Simulation};
use crate::vector2d::Vector2D;

pub type CreatureRng = ChaCha8Rng;


pub struct Creature {
    position: Vector2D<usize>,
    genome: Genome,

    brain: Brain,
    sensory_data: HashMap<SensoryNeuron, f64>,
    action_data: HashMap<ActionNeuron, f64>,

    color: Color,
    rng: CreatureRng
}

impl Creature {
    pub fn new(position: Vector2D<usize>, genome: Genome, unique_stream_rng: CreatureRng) -> Result<Self, Box<dyn Error>> {
        let color = genome.generate_color()?;
        let brain = Brain::from_genome(&genome);

        let (sensory_data, action_data) = brain.neurons_empty_value_map();

        Ok(Self {
            position,
            genome,
            brain,
            sensory_data,
            action_data,
            color,
            rng: unique_stream_rng
        })
    }

    // Ugly nesting, but either this or cloning the keys/using RefCells
    pub fn gather_sensory_data(&mut self, sim: &Simulation) -> () {
        for (neuron, value) in self.sensory_data.iter_mut() {
            // Every single sensory data MUST be between -1.0 and 1.0
            // Some sensory data might be between 0 and 1, and that's okay
            *value = match neuron {
                SensoryNeuron::Random => self.rng.gen_range(-1.0..=1.0),

                // This part gonna be hell lul
                SensoryNeuron::DistToBarrierNorth => self.position.y as f64 / sim.field_height() as f64,
                SensoryNeuron::DistToBarrierSouth => 1.0 - (self.position.y as f64 / sim.field_height() as f64),
                SensoryNeuron::DistToBarrierWest => self.position.x as f64 / sim.field_width() as f64,
                SensoryNeuron::DistToBarrierEast => 1.0 - (self.position.x as f64 / sim.field_width() as f64),
            }
        }
    }

    pub fn execute_actions(&mut self, sim: &Simulation) -> Vec<Signal> {
        let mut signals = vec![];
        let mut raw_movement_value = Vector2D::new(0.0, 0.0);

        let mut normalized_value: f64;
        for (neuron, &value) in self.action_data.iter() {
            normalized_value = (value.tanh() + 1.0) / 2.0;
            match neuron {
                ActionNeuron::MoveNorth => raw_movement_value.y -= normalized_value,
                ActionNeuron::MoveSouth => raw_movement_value.y += normalized_value,
                ActionNeuron::MoveEast => raw_movement_value.x += normalized_value,
                ActionNeuron::MoveWest => raw_movement_value.x -= normalized_value,
            }
        }

        if let Some(pos_change) = self.process_raw_movement_value(raw_movement_value, sim) {
            signals.push(pos_change);
        }

        signals
    }

    fn process_raw_movement_value(&mut self, value: Vector2D<f64>, sim: &Simulation) -> Option<Signal> {
        // We see if the creature is 'determined' to move (using Rng), and move them 1 pixel in the
        // desired direction
        let mut movement = Vector2D::new(0,0);
        if value.x != 0.0 {
            if self.rng.gen_bool(value.x.abs()) {
                movement.x = value.x.signum() as usize;
            }
        }

        if value.y != 0.0 {
            if self.rng.gen_bool(value.y.abs()) {
                movement.y = value.y.signum() as usize;
            }
        }

        if movement != Vector2D::new(0, 0) {
            let new_position = self.position + movement;
            if !sim.is_position_occupied(&new_position) {
                return Some(
                    Signal::PositionChanged { old: self.position, new: new_position }
                );
            }
        }

        None
    }

    pub fn think(&mut self) {
        self.brain.process_connections(
            &self.sensory_data,
            &mut self.action_data
        );
    }

    pub fn position(&self) -> &Vector2D<usize> {
        &self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn brain(&self) -> &Brain {
        &self.brain
    }
}


#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;
    use sensory_neuron::TOTAL_SENSORY_NEURON_VARIANT;

    fn gen_simulation() -> Simulation {
        Simulation::new(100, 100, 5, [0; 32], 10)
    }

    fn gen_creature() -> Creature {
        let genome = Genome::from_byte_slice(&[0; 20]);
        let brain = Brain::from_genome(&genome);
        Creature {
            position: Vector2D::new(4, 10),
            genome,

            brain,
            sensory_data: HashMap::new(),
            action_data: HashMap::new(),

            color: Color::new(0, 0, 0),
            rng: CreatureRng::from_seed([0; 32])
        }
    }

    #[test]
    fn test_gathering_sensory_data() -> () {
        let mut creature = gen_creature();
        let sim = gen_simulation();

        for id in 0..TOTAL_SENSORY_NEURON_VARIANT {
            creature.sensory_data.insert(SensoryNeuron::from_id(id).unwrap(), 0.0);
        }

        creature.gather_sensory_data(&sim);

        let sensory_data = creature.sensory_data;
        assert_eq!(sensory_data[&SensoryNeuron::Random], 0.6738395137652948);

        assert_eq!(sensory_data[&SensoryNeuron::DistToBarrierNorth], 0.1);
        assert_eq!(sensory_data[&SensoryNeuron::DistToBarrierSouth], 1.0 - 0.1);
        assert_eq!(sensory_data[&SensoryNeuron::DistToBarrierWest], 0.04);
        assert_eq!(sensory_data[&SensoryNeuron::DistToBarrierEast], 1.0 - 0.04);
    }
}
