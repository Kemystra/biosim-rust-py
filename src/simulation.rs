use crate::creature::Creature;


pub struct Simulation {
    field_width: usize,
    field_height: usize,

    creatures: Vec<Creature>,
    seed: Vec<u8>
}

impl Simulation {
    pub fn new(field_width: usize, field_height: usize, seed: Vec<u8>) -> Self {
        Self {
            field_width, field_height,
            creatures: vec![],
            seed
        }
    }

    pub fn run(&mut self) {
        
    }

    pub fn creatures(&self) -> &Vec<Creature> {
        &self.creatures
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_field_width() {
        let sim = Simulation::new(100,100,vec![10]);
        assert_eq!(sim.field_width, 100);
    }

    #[test]
    fn get_field_height() {
        let sim = Simulation::new(100,100,vec![10]);
        assert_eq!(sim.field_height, 100);
    }
}
