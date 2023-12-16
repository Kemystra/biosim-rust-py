use crate::genome::Genome;

use crate::neuron::{Connection, input_neuron, action_neuron, internal_neuron};
use input_neuron::InputNeuron;
use action_neuron::ActionNeuron;
use internal_neuron::InternalNeuron;

use crate::vector2d::Vector2D;


pub struct Creature {
    genome: Genome,
    position: Vector2D<usize>,
    connections: Vec<Connection>,
    input_neurons: Vec<Box<dyn InputNeuron>>,
    internal_neurons: Vec<Box<InternalNeuron>>,
    action_neurons: Vec<Box<dyn ActionNeuron>>
}
