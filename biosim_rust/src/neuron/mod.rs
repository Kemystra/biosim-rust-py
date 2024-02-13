pub mod internal_neuron;
pub mod action_neuron;
pub mod sensory_neuron;

use crate::genome::Gene;
use sensory_neuron::SensoryNeuron;
use action_neuron::ActionNeuron;
use internal_neuron::InternalNeuron;

pub struct Brain {
    connections: Vec<Connection>,
    internal_neurons: Vec<InternalNeuron>,
    sensory_neurons: Vec<SensoryNeuron>,
    action_neurons: Vec<ActionNeuron>
}

pub struct Connection {
    connection_type: ConnectionType,
    weight: f64,
    source_id: usize,
    sink_id: usize,
}
/*
impl Connection {
    pub fn from_gene(gene: Gene) -> Self {
        // Gene bit layout (from front):
        // bit 0-1 indicates ConnectionType
        let connection_type = match gene >> 14 {
            0 => ConnectionType::InputToAction,
            1 => ConnectionType::InputToInternal,
            2 => ConnectionType::InternalToInternal,
            3 => ConnectionType::InternalToAction
        };

        // bit 2-6 indicates source ID
    }
}
*/

pub enum ConnectionType {
    InputToAction,
    InputToInternal,
    InternalToInternal,
    InternalToAction,
}
