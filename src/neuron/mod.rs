pub mod internal_neuron;
pub mod action_neuron;
pub mod input_neuron;


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
