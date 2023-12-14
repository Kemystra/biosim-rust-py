use crate::genome::Genome;
use crate::neuron::{Connection, InputNeuron, ActionNeuron, InternalNeuron};
use crate::vector2d::Vector2D;


pub struct Creature {
    genome: Genome,
    position: Vector2D<usize>,
    connections: Vec<Connection>,
    input_neurons: Vec<Box<InputNeuron>>,
    internal_neurons: Vec<Box<InternalNeuron>>,
    action_neurons: Vec<Box<ActionNeuron>>
}
