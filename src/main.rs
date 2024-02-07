mod simulation;
mod creature;
mod genome;
mod neuron;
mod renderer;
mod vector2d;

use simulation::Simulation;
use renderer::{RendererBuilder, Color};

const FIELD_WIDTH: usize = 100;
const FIELD_HEIGHT: usize = 100;

const BLOCK_SIZE: usize = 5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sim = Simulation::new(FIELD_WIDTH, FIELD_HEIGHT);

    let gray = Color::from(0x00_aa_aa_aa);
    let light_orange = Color::from(0x00_ff_ee_bf);
    let mut renderer = RendererBuilder::new()
        .with_field_color(light_orange)
        .with_border_color(gray)
        .build()?;

    Ok(())
}
