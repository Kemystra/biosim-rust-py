use std::io::{BufWriter, Write};
use std::fs::File;
use std::error::Error;

mod simulation;
mod creature;
mod genome;
mod neuron;
mod renderer;
mod vector2d;

use simulation::Simulation;
use renderer::{RendererBuilder, Color, Buffer};

const FIELD_WIDTH: usize = 100;
const FIELD_HEIGHT: usize = 100;


fn main() -> Result<(), Box<dyn Error>> {
    let sim = Simulation::new(FIELD_WIDTH, FIELD_HEIGHT);

    let gray = Color::new(0xaa, 0xaa, 0xaa);
    let light_orange = Color::new(0xff, 0xdd, 0x8c);

    let mut renderer = RendererBuilder::new()
        .with_field_color(light_orange)
        .with_border_color(gray)
        .with_field_dimensions(FIELD_WIDTH, FIELD_HEIGHT)
        .build()?;

    renderer.init()?;
    let raw_image_buffer = renderer.render(&sim);
    let (buffer_width, buffer_height) = renderer.buffer_dimensions();

    export_to_tga(raw_image_buffer, buffer_width, buffer_height)?;

    Ok(())
}

fn export_to_tga(buffer: Buffer, buffer_width: usize, buffer_height: usize) -> Result<(), Box<dyn Error>> {
    let mut file_writer = BufWriter::new(File::create("test.tga")?);
    let mut header_data: [u8; 18] = [0; 18];

    // Image type: uncompressed true-color
    header_data[2] = 2;

    // Image width (stored over 2 bytes)
    header_data[12] = (0xFF & buffer_width) as u8;
    header_data[13] = (0xFF & (buffer_width >> 8)) as u8;

    // Image height (stored over 2 bytes)
    header_data[14] = (0xFF & buffer_height) as u8;
    header_data[15] = (0xFF & (buffer_height >> 8)) as u8;

    // Pixel depth (24 bits per pixel)
    header_data[16] = 24;

    // Image descriptor; set ordering to top-bottom, left-right
    header_data[17] = 0b00_10_00_00;

    file_writer.write(&header_data)?;

    for color in buffer {
        file_writer.write(&color.byte_array())?;
    }

    file_writer.flush()?;

    Ok(())
}
