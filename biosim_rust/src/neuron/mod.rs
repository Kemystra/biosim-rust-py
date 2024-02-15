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
}
/*
impl Connection {
    pub fn from_gene(gene: Gene) -> Self {
        // Gene bit layout (from front):
        // bit 0-1 indicates ConnectionType
        let connection_type = match gene >> 14 {
            0 => ConnectionType::SensoryToAction,
            1 => ConnectionType::SensoryToInternal,
            2 => ConnectionType::InternalToInternal,
            3 => ConnectionType::InternalToAction
        };

        // bit 2-6 indicates source ID
    }
}
*/

pub enum ConnectionType {
    SensoryToAction {source: SensoryNeuron, sink: ActionNeuron},
    SensoryToInternal {source: SensoryNeuron, sink: u8},
    InternalToInternal {source: u8, sink: u8},
    InternalToAction {source: u8, sink: ActionNeuron},
}
