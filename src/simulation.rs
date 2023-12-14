use crate::creature::Creature;


pub struct Simulation {
    field_width: usize,
    field_height: usize,

    creatures: Vec<Creature>
}

impl Simulation {
    fn new(field_width: usize, field_height: usize) -> Self {
        Self {
            field_width, field_height,
            creatures: vec![]
        }
    }

    fn field_width(&self) -> usize {
        self.field_width
    }

    fn field_height(&self) -> usize {
        self.field_height
    }

    fn run(&mut self) {
        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_field_width() {
        let sim = Simulation::new(100,100);
        assert_eq!(sim.field_width, 100);
    }

    #[test]
    fn get_field_height() {
        let sim = Simulation::new(100,100);
        assert_eq!(sim.field_height, 100);
    }
}
