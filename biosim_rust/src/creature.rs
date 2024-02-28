use std::error::Error;
use rand::Rng;
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

    pub fn gather_sensory_data(&mut self, sim: &Simulation) -> () {
        let sensory_neurons: Vec<SensoryNeuron> = self.sensory_data.keys().cloned().collect();
        let mut value: f64;
        for neuron in sensory_neurons {
            value = self.read_sensor(neuron, sim);
            self.sensory_data.insert(neuron, value);
        }
    }

    fn read_sensor(&mut self, neuron: SensoryNeuron, sim: &Simulation) -> f64 {
    // Every single sensory data MUST be between -1.0 and 1.0
    // Some sensory data might be between 0 and 1, and that's okay
    match neuron {
        SensoryNeuron::Random => self.rng_mut().gen_range(-1.0..=-1.0),

        // This part gonna be hell lul
        SensoryNeuron::DistToBarrierNorth => (self.position().y / sim.field_height()) as f64,
        SensoryNeuron::DistToBarrierSouth => ((sim.field_height() - self.position().y) / sim.field_height()) as f64,
        SensoryNeuron::DistToBarrierWest => (self.position().x / sim.field_width()) as f64,
        SensoryNeuron::DistToBarrierEast => ((sim.field_width() - self.position().x) / sim.field_width()) as f64,
        }
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
