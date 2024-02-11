pub mod internal_neuron;
pub mod action_neuron;
pub mod input_neuron;

use crate::genome::Gene;
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
    source_id: usize,
    sink_id: usize,
}

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

macro_rules! NeuronFactory {
    {
        ($trait_name:ident)
        $($neuron_name:ident),* 
    }=> {
        pub fn stringify!($trait_name, "Factory")(id: u8) -> Box<dyn stringify!($trait_name)> {
            match id {
                $(stringify!($neuron_name)::ID => stringify!($neuron_name)::new()),*,
                _ => panic!("No ID match for ", stringify!($trait_name))
            }
        }
    };
}

pub enum ConnectionType {
    InputToAction,
    InputToInternal,
    InternalToInternal,
    InternalToAction,
}
