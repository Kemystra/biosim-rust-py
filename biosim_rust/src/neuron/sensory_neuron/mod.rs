use biosim_rust_macros::enum_from_id;

use crate::simulation::Simulation;
use crate::creature::Creature;


#[enum_from_id]
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum SensoryNeuron {
    DistToBarrierNorth,
    DistToBarrierSouth,
    DistToBarrierEast,
    DistToBarrierWest,
}
