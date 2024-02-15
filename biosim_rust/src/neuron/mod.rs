pub mod internal_neuron;
pub mod action_neuron;
pub mod sensory_neuron;

use crate::genome::Gene;
use sensory_neuron::{SensoryNeuron, TOTAL_SENSORY_NEURON_VARIANT};
use action_neuron::{ActionNeuron, TOTAL_ACTION_NEURON_VARIANT};
use internal_neuron::InternalNeuron;

pub type InternalNeuronID = usize;
const MAX_INTERNAL_NEURONS: usize = 3;

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

impl Connection {
    pub fn from_gene(gene: Gene) -> Self {
        // Gene bit layout (from front):
        // bit 0-1 indicates ConnectionType
        // bit 2-6 indicates source ID
        // bit 7-11 indicates sink ID
        // bit 12-15 indicates weight, a 4-bit signed integer
        let connection_type_id = gene >> 14;
        let source_id = ((gene >> 9) & 0b11111) as usize;
        let sink_id = ((gene >> 4) & 0b11111) as usize;
        let weight: f64 = ((gene & 0b1111) as i8 - 8).into();

        // This is kinda ugly, but it (might) work
        // Btw, should never EVER fail!
        let conn_type = match connection_type_id {
            0 => ConnectionType::SensoryToAction {
                source: SensoryNeuron::from_id(source_id % TOTAL_SENSORY_NEURON_VARIANT).unwrap(),
                sink: ActionNeuron::from_id(sink_id % TOTAL_ACTION_NEURON_VARIANT).unwrap()
            },

            1 => ConnectionType::SensoryToInternal {
                source: SensoryNeuron::from_id(source_id % TOTAL_SENSORY_NEURON_VARIANT).unwrap(),
                sink: sink_id % MAX_INTERNAL_NEURONS
            },

            2 => ConnectionType::InternalToInternal {
                source: source_id % MAX_INTERNAL_NEURONS,
                sink: sink_id % MAX_INTERNAL_NEURONS
            },

            3 => ConnectionType::InternalToAction {
                source: source_id % MAX_INTERNAL_NEURONS,
                sink: ActionNeuron::from_id(sink_id % TOTAL_ACTION_NEURON_VARIANT).unwrap()
            },

            _ => panic!("WTF happened here!")
        };

        Self {
            connection_type: conn_type,
            weight
        }
    }
}

pub enum ConnectionType {
    SensoryToAction {source: SensoryNeuron, sink: ActionNeuron},
    SensoryToInternal {source: SensoryNeuron, sink: InternalNeuronID},
    InternalToInternal {source: InternalNeuronID, sink: InternalNeuronID},
    InternalToAction {source: InternalNeuronID, sink: ActionNeuron},
}


#[cfg(test)]
mod tests {
    use biosim_rust_macros::enum_from_id;

    #[enum_from_id]
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        VariantA,
        VariantB,
        VariantC,
    }

    #[test]
    fn test_enum_from_id() {
        // Use the generated from_id function to get enum variants
        let variant_a = MyEnum::from_id(0);
        let variant_b = MyEnum::from_id(1);
        let variant_c = MyEnum::from_id(2);

        // Assert that the variants are as expected
        assert_eq!(variant_a, Some(MyEnum::VariantA));
        assert_eq!(variant_b, Some(MyEnum::VariantB));
        assert_eq!(variant_c, Some(MyEnum::VariantC));

        // Test with an out-of-range ID, should return None
        let out_of_range = MyEnum::from_id(10);
        assert_eq!(out_of_range, None);
    }
}
