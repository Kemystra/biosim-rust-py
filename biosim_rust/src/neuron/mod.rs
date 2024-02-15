pub mod internal_neuron;
pub mod action_neuron;
pub mod sensory_neuron;

use crate::genome::Gene;
use sensory_neuron::SensoryNeuron;
use action_neuron::ActionNeuron;
use internal_neuron::InternalNeuron;

pub type InternalNeuronID = u8;

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
