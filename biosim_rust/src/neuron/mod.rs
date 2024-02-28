use std::cmp::Ordering;
use std::collections::HashMap;

pub mod internal_neuron;
pub mod action_neuron;
pub mod sensory_neuron;

use crate::genome::{Gene, Genome};
use sensory_neuron::{SensoryNeuron, TOTAL_SENSORY_NEURON_VARIANT};
use action_neuron::{ActionNeuron, TOTAL_ACTION_NEURON_VARIANT};
use internal_neuron::InternalNeuron;

pub type InternalNeuronID = usize;
const MAX_INTERNAL_NEURONS: usize = 4;

pub struct Brain {
    connections: Vec<Connection>,
    internal_neurons: Vec<InternalNeuron>,
}

impl Brain {
    pub fn connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    pub fn from_genome(genome: &Genome) -> Self {
        let mut connections: Vec<Connection> = genome.genes()
            .iter()
            .map(|gene| Connection::from_gene(*gene))
            .collect();

        // Sorting the connections to ensure:
        // 1. Read SensoryNeuron
        // 2. Go through InternalNeuron
        // 3. Send to ActionNeuron
        // Direct Sensory-Action connections are executed immediately
        connections[..].sort_by(|a,b| a.connection_type.partial_cmp(&b.connection_type).unwrap());

        // Trimming unnecessary connections and neurons
        // This is when:
        // InternalToAction => `source` has no input
        //
        // InternalToInternal => `source` has no input OR `sink` has no output
        // (If `source` and `sink` is the same, it implies a loopback to itself. That is
        // useless without other separate inputs/outputs)
        //
        // SensoryToInternal => `sink` has no output

        // Map InternalNeuronID to its input/output count
        let mut neurons_input_count: HashMap<InternalNeuronID, usize> = (0..MAX_INTERNAL_NEURONS)
            .into_iter().map(|num| (num, 0)).collect();
        let mut neurons_output_count = neurons_input_count.clone();

        // Calculate neurons input/output
        for conn in &connections {
            Self::count_conn_input_output(conn, &mut neurons_input_count, &mut neurons_output_count);
        }

        let original_length = connections.len();
        Self::recursive_brain_trimming(
            &mut connections,
            original_length,
            &mut neurons_input_count,
            &mut neurons_output_count
        );

        Brain {
            connections,
            internal_neurons: vec![InternalNeuron::new(); MAX_INTERNAL_NEURONS],
        }
    }

    pub fn neurons_empty_value_map(&self) -> (HashMap<SensoryNeuron, f64>, HashMap<ActionNeuron, f64>) {
        let mut sensory_neuron_map = HashMap::new();
        let mut action_neuron_map = HashMap::new();

        for conn in &self.connections {
            Self::gather_sensory_and_action_neurons(&conn, &mut sensory_neuron_map, &mut action_neuron_map);
        }

        (sensory_neuron_map, action_neuron_map)
    }

    fn gather_sensory_and_action_neurons(
        conn: &Connection,
        sensory_neuron_map: &mut HashMap<SensoryNeuron, f64>,
        action_neuron_map: &mut HashMap<ActionNeuron, f64>
    ) -> () {
        match conn.connection_type() {
            &ConnectionType::SensoryToInternal { source, .. } => {
                sensory_neuron_map.insert(source, 0.0);
            }

            &ConnectionType::SensoryToAction { source, sink } => {
                sensory_neuron_map.insert(source, 0.0);
                action_neuron_map.insert(sink, 0.0);
            }

            &ConnectionType::InternalToAction { sink, .. } => {
                action_neuron_map.insert(sink, 0.0);
            }

            _ => {}
        }
    }

    fn recursive_brain_trimming(
        connections: &mut Vec<Connection>,
        original_length: usize,
        neurons_input_count: &mut HashMap<InternalNeuronID, usize>,
        neurons_output_count: &mut HashMap<InternalNeuronID, usize>
    ) -> () {

        connections.retain(|conn| {
            Self::is_connection_useful(conn, neurons_input_count, neurons_output_count)
        });

        if connections.len() < original_length {
            let original_length = connections.len();
            Self::recursive_brain_trimming(connections, original_length, neurons_input_count, neurons_output_count);
        }
    }

    fn is_connection_useful(
        conn: &Connection,
        neurons_input_count: &mut HashMap<InternalNeuronID, usize>,
        neurons_output_count: &mut HashMap<InternalNeuronID, usize>
    ) -> bool {

        // Alright this one is kinda twizzy
        // If it's SensoryToInternal, then we need to check if the InternalNeuron has OUTPUT, not INPUT
        // Vice versa for InternalToAction
        // Very important to remember as the match statement here is an inverse of the
        // count_conn_input_output() function
        match conn.connection_type {
            ConnectionType::SensoryToInternal { sink, .. } => neurons_output_count.get(&sink) != Some(&0),
            ConnectionType::InternalToAction { source, .. } => neurons_input_count.get(&source) != Some(&0),

            ConnectionType::InternalToInternal { source, sink } => {
                Self::handle_interconnection_case(source, sink, neurons_input_count, neurons_output_count)
            }

            _ => true
        }
    }

    fn handle_interconnection_case(
        source: InternalNeuronID, sink: InternalNeuronID,
        neurons_input_count: &mut HashMap<InternalNeuronID, usize>,
        neurons_output_count: &mut HashMap<InternalNeuronID, usize>
    ) -> bool {

        // If the source neuron has no input...
        if neurons_input_count.get(&source) == Some(&0) {
            // The sink neuron will also lose 1 input
            let sink_input_count = *neurons_input_count.get(&sink).unwrap();
            if let Some(num) = sink_input_count.checked_sub(1) {
                neurons_input_count.insert(sink, num).unwrap();
            }

            return false;
        }
        // If the sink neuron has no output...
        else if neurons_output_count.get(&sink) == Some(&0) {
            // The source neuron will also lose 1 output
            let source_output_count = *neurons_output_count.get(&source).unwrap();
            if let Some(num) = source_output_count.checked_sub(1) {
                neurons_output_count.insert(source, num).unwrap();
            }

            return false;
        }

        true
    }

    // Check each connections that has InternalNeuron
    // and mark whether it has valid input/output
    fn count_conn_input_output(
        conn: &Connection,
        neurons_input_count: &mut HashMap<InternalNeuronID, usize>,
        neurons_output_count: &mut HashMap<InternalNeuronID, usize>
    ) -> () {

        // Since the InternalNeuronID has been modulus to MAX_INTERNAL_NEURONS
        // we don't have to worry about non-existent key
        match conn.connection_type {
            ConnectionType::SensoryToInternal { sink, .. } => {
                *neurons_input_count.get_mut(&sink).unwrap() += 1;
            },

            ConnectionType::InternalToAction { source, .. } => {
                *neurons_output_count.get_mut(&source).unwrap() += 1;
            },

            // If it is a loopback to itself, ignore
            ConnectionType::InternalToInternal { source, sink } if source != sink => {
                *neurons_output_count.get_mut(&source).unwrap() += 1;
                *neurons_input_count.get_mut(&sink).unwrap() += 1;
            }

            // We still need this; InternalToInternal is ignored for loopbacks
            // Making the match non-exhaustive
            _ => {}
        }
    }
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
        let conn_type_id = gene >> 14;
        let source_id = ((gene >> 9) & 0b11_11_1) as usize;
        let sink_id = ((gene >> 4) & 0b11_11_1) as usize;
        let weight: f64 = ((gene & 0xF) as i8 - 8).into();

        // This is kinda ugly, but it (might) work
        // Btw, should never EVER fail!
        let conn_type = match conn_type_id {
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

    pub fn connection_type(&self) -> &ConnectionType {
        &self.connection_type
    }

    pub fn weight(&self) ->  f64 {
        self.weight
    }
}

#[derive(PartialEq, Debug)]
pub enum ConnectionType {
    SensoryToAction {source: SensoryNeuron, sink: ActionNeuron},
    SensoryToInternal {source: SensoryNeuron, sink: InternalNeuronID},
    InternalToInternal {source: InternalNeuronID, sink: InternalNeuronID},
    InternalToAction {source: InternalNeuronID, sink: ActionNeuron},
}

// Ugly stuff generated by chatGPT bitches
// No, I don't want to make yet another macro for this
impl PartialOrd for ConnectionType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::{Less, Equal, Greater};
        use ConnectionType::*;

        match (self, other) {
            (SensoryToAction {..}, SensoryToAction {..}) => Some(Equal),
            (SensoryToAction {..}, _) => Some(Less),

            (SensoryToInternal {..}, SensoryToAction {..}) => Some(Greater),
            (SensoryToInternal {..}, SensoryToInternal {..}) => Some(Equal),
            (SensoryToInternal {..}, _) => Some(Less),

            (InternalToInternal {..}, InternalToAction {..}) => Some(Less),
            (InternalToInternal {..}, InternalToInternal {..}) => Some(Equal),
            (InternalToInternal {..}, _) => Some(Greater),

            (InternalToAction {..}, InternalToAction {..}) => Some(Equal),
            (InternalToAction {..}, _) => Some(Greater),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_connection_type_partial_ord() {
        let conn1 = ConnectionType::SensoryToAction {
            source: SensoryNeuron::DistToBarrierEast,
            sink: ActionNeuron::MoveEast
        };

        let conn2 = ConnectionType::SensoryToInternal {
            source: SensoryNeuron::DistToBarrierEast,
            sink: 10
        };

        let conn3 = ConnectionType::InternalToInternal {
            source: 2,
            sink: 4
        };

        let conn4 = ConnectionType::InternalToAction {
            source: 9,
            sink: ActionNeuron::MoveEast
        };

        assert!(conn1 < conn2);
        assert!(conn2 < conn3);
        assert!(conn3 < conn4);
    }
}
