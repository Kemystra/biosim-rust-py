use std::error::Error;
use std::collections::HashMap;
use rand_chacha::ChaCha8Rng;

use crate::genome::Genome;
use crate::renderer::Color;
use crate::neuron::{ConnectionType, Brain, sensory_neuron, action_neuron};
use sensory_neuron::SensoryNeuron;
use action_neuron::ActionNeuron;
use crate::simulation::Simulation;
use crate::vector2d::Vector2D;


pub struct Creature {
    position: Vector2D<usize>,
    genome: Genome,

    brain: Brain,

    color: Color,
    rng: ChaCha8Rng
}

impl Creature {
    pub fn new(position: Vector2D<usize>, genome: Genome, unique_stream_rng: ChaCha8Rng) -> Result<Self, Box<dyn Error>> {
        let color = genome.generate_color()?;
        let brain = Brain::from_genome(&genome);

        Ok(Self {
            position,
            genome,
            brain,
            color,
            rng: unique_stream_rng
        })
    }

    pub fn think(&mut self, sim: &Simulation) -> () {
        self.brain.gather_sensory_data(&mut self.rng, self, sim)
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

    pub fn rng_mut(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }
}
