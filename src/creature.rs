use genome::Genome;
use neuron::{Connection, InputNeuron, ActionNeuron, InternalNeuron};
use vector2d::Vector2D;


struct Creature {
    genome: Genome,
    position: Vector2D<usize>,
    connections: Vec<Connection>,
    input_neurons: Vec<Box<InputNeuron>>,
    internal_neurons: Vec<Box<InternalNeuron>>,
    action_neurons: Vec<Box<ActionNeuron>>
}
