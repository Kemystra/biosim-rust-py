use crate::creature::Creature;


pub struct Simulation {
    field_block_width: usize,
    field_block_height: usize,

    creatures: Vec<Creature>
}

impl Simulation {
    pub fn new(field_block_width: usize, field_block_height: usize) -> Self {
        Self {
            field_block_width, field_block_height,
            creatures: vec![]
        }
    }

    pub fn field_block_width(&self) -> usize {
        self.field_block_width
    }

    pub fn field_block_height(&self) -> usize {
        self.field_block_height
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
    fn get_field_block_width() {
        let sim = Simulation::new(100,100);
        assert_eq!(sim.field_block_width, 100);
    }

    #[test]
    fn get_field_block_height() {
        let sim = Simulation::new(100,100);
        assert_eq!(sim.field_block_height, 100);
    }
}
