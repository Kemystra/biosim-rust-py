use biosim_rust_macros::enum_from_id;

use crate::simulation::Simulation;
use crate::creature::Creature;


#[enum_from_id]
pub enum SensoryNeuron {
    DistToBarrierNorth,
    DistToBarrierSouth,
    DistToBarrierEast,
    DistToBarrierWest,
}
