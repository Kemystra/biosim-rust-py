use crate::simulation::Simulation;
use crate::creature::Creature;


pub trait SensoryNeuron {
    fn new() -> Self where Self: Sized;
    fn read_input(&self, sim: &Simulation, creature: &Creature) -> f64;
}

pub trait SensoryNeuronType {
    const ID: u8;
}
