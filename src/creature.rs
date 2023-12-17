use crate::genome::Genome;
use crate::renderer::Color;

use crate::neuron::{Connection, input_neuron, action_neuron, internal_neuron};
use input_neuron::InputNeuron;
use action_neuron::ActionNeuron;
use internal_neuron::InternalNeuron;

use crate::vector2d::Vector2D;


pub struct Creature {
    position: Vector2D<usize>,

    genome: Genome,
    color: Color,

    connections: Vec<Connection>,
    input_neurons: Vec<Box<dyn InputNeuron>>,
    internal_neurons: Vec<Box<InternalNeuron>>,
    action_neurons: Vec<Box<dyn ActionNeuron>>
}

impl Creature {
    pub fn position(&self) -> Vector2D<usize> {
        self.position
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
