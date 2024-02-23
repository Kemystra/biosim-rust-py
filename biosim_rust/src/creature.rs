use std::error::Error;
use std::collections::HashMap;
use rand_chacha::ChaCha8Rng;

use crate::genome::Genome;
use crate::renderer::Color;
use crate::neuron::{ConnectionType, Brain, sensory_neuron::SensoryNeuron};
use crate::simulation::Simulation;
use crate::vector2d::Vector2D;


pub struct Creature {
    position: Vector2D<usize>,
    genome: Genome,

    sensory_data: HashMap<SensoryNeuron, f64>,
    brain: Brain,

    color: Color,
    rng: ChaCha8Rng
}

impl Creature {
    pub fn new(position: Vector2D<usize>, genome: Genome, unique_stream_rng: ChaCha8Rng) -> Result<Self, Box<dyn Error>> {
        let color = genome.generate_color()?;
        let brain = Brain::from_genome(&genome);

        let mut sensory_data = HashMap::new();
        for conn in brain.connections() {
            match conn.connection_type() {
                &ConnectionType::SensoryToAction { source, .. } |
                &ConnectionType::SensoryToInternal { source, .. } => {sensory_data.insert(source, 0.0);},
                _ => {}
            };
        }

        Ok(Self {
            position,
            genome,
            brain,
            sensory_data,
            color,
            rng: unique_stream_rng
        })
    }

    pub fn think(&mut self, sim: &Simulation) -> () {
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
