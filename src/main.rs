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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sim = Simulation::new(FIELD_WIDTH, FIELD_HEIGHT);

    let gray = Color::new(0xaa, 0xaa, 0xaa);
    let light_orange = Color::new(0xff, 0xdd, 0x8c);

    let mut renderer = RendererBuilder::new()
        .with_field_color(light_orange)
        .with_border_color(gray)
        .with_field_dimensions(FIELD_WIDTH, FIELD_HEIGHT)
        .build()?;

    Ok(())
}
