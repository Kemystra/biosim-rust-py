use crate::simulation::Simulation;
use crate::creature::Creature;


pub trait ActionNeuron {
    fn new() -> Self where Self: Sized;
    fn perform_action(&self, sim: &mut Simulation, creature: &mut Creature);
}
