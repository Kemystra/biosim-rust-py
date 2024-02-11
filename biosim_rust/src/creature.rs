use std::error::Error;

use crate::genome::Genome;
use crate::renderer::Color;

use crate::neuron::{Connection, sensory_neuron, action_neuron, internal_neuron};
use sensory_neuron::SensoryNeuron;
use action_neuron::ActionNeuron;
use internal_neuron::InternalNeuron;

use crate::vector2d::Vector2D;


pub struct Creature {
    position: Vector2D<usize>,

    genome: Genome,
    color: Color,

    connections: Vec<Connection>,
    sensory_neurons: Vec<Box<dyn SensoryNeuron>>,
    internal_neurons: Vec<Box<InternalNeuron>>,
    action_neurons: Vec<Box<dyn ActionNeuron>>
}

impl Creature {
    pub fn new(position: Vector2D<usize>, genome: Genome) -> Result<Self, Box<dyn Error>> {
        let color = genome.generate_color()?;
        Ok(Self {
            position,
            genome,
            color,
            connections: vec![],
            sensory_neurons: vec![],
            internal_neurons: vec![],
            action_neurons: vec![]
        })
    }
    pub fn position(&self) -> Vector2D<usize> {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
