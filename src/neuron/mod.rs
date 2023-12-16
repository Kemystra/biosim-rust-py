pub mod internal_neuron;
pub mod action_neuron;
pub mod input_neuron;

use input_neuron::InputNeuron;
use action_neuron::ActionNeuron;

pub struct Brain {
    connections: Vec<Connection>,
    internal_neurons: Vec<internal_neuron::InternalNeuron>,
    input_neurons: Vec<Box<dyn InputNeuron>>,
    action_neurons: Vec<Box<dyn ActionNeuron>>
}

pub struct Connection {
    connection_type: ConnectionType,
    weight: f64,
    source_index: usize,
    sink_index: usize,
}

pub enum ConnectionType {
    InputToAction,
    InputToInternal,
    InternalToInternal,
    InternalToAction,
}
