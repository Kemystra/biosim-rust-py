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


