use biosim_rust_macros::enum_from_id;
use rand::Rng;

use crate::simulation::Simulation;
use crate::creature::Creature;


#[enum_from_id]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SensoryNeuron {
    Random,
    DistToBarrierNorth,
    DistToBarrierSouth,
    DistToBarrierEast,
    DistToBarrierWest,
}

// We need mutable ref. for Creature because using an RNG
// will mutate it
pub fn read_sensor(sensory_neuron: &SensoryNeuron, creature: &mut Creature, sim: &Simulation) -> f64 {
    // Every single sensory data MUST be between -1.0 and 1.0
    // Some sensory data might be between 0 and 1, and that's okay
    match sensory_neuron {
        SensoryNeuron::Random => creature.rng_mut().gen_range(-1.0..=-1.0),

        // This part gonna be hell lul
        SensoryNeuron::DistToBarrierNorth => (creature.position().y / sim.field_height()) as f64,
        SensoryNeuron::DistToBarrierSouth => ((sim.field_height() - creature.position().y) / sim.field_height()) as f64,
        SensoryNeuron::DistToBarrierWest => (creature.position().x / sim.field_width()) as f64,
        SensoryNeuron::DistToBarrierEast => ((sim.field_width() - creature.position().x) / sim.field_width()) as f64,
    }
}
