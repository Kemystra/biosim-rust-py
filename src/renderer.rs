use std::default::Default;
use std::cmp::PartialEq;

use thiserror::Error;

use crate::simulation::Simulation;


pub type Buffer = Vec<Color>;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new<T: TryInto<u8>>(r: T, g: T, b: T) -> Self {
        Self (
            r.try_into().unwrap_or_default(),
            g.try_into().unwrap_or_default(),
            b.try_into().unwrap_or_default()
        )
    }
}

#[derive(Default, Debug)]
struct RendererAttributes {
    pub field_width: usize,
    pub field_height: usize,

    pub field_color: Color,
    pub border_color: Color
}

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Trying to access a pixel out of Buffer range ({0}, {1})")]
    OutOfBufferRange(usize, usize),
    #[error("Trying to stamp on a block outside of field ({0}, {1})")]
    OutOfFieldRange(usize, usize),
    #[error("Total width/height of Buffer should be bigger than 0_usize")]
    BufferTooSmall
}

pub struct Renderer<'a> {
    attr: RendererAttributes,
    buffer_height: usize,
    buffer_width: usize,

    is_initialized: bool,
    empty_field_buffer: &'a[Color]
}

impl Renderer<'_> {
    fn new(attr: RendererAttributes) -> Self {
        let buffer_width = attr.field_width + 2;
        let buffer_height = attr.field_height + 2;

        Self {
            attr,
            buffer_width,
            buffer_height,
            is_initialized: false,
            empty_field_buffer: &vec![]
        }
    }

    pub fn init(&mut self) {
        let mut initial_field = Vec::with_capacity(
            self.buffer_width * self.buffer_height
        );

        // Draw border
        // Top & bottom
        for i in 0..self.buffer_width {
            self.plot_pixel(&mut initial_field, i, 0, self.attr.border_color);
            self.plot_pixel(&mut initial_field, i, self.buffer_height - 1, self.attr.border_color);
        }

        // Right & left
        for i in 0..self.buffer_width {
            self.plot_pixel(&mut initial_field, 0, i, self.attr.border_color);
            self.plot_pixel(&mut initial_field, self.buffer_width - 1, i, self.attr.border_color);
        }

        // Draw empty field
        for x in 0..self.buffer_width {
            for y in 0..self.buffer_height {
                self.plot_pixel(&mut initial_field, x, y, self.attr.field_color);
            }
        }

        self.empty_field_buffer = &initial_field;
    }

    pub fn render(&self, sim: &Simulation) -> Buffer {
        let mut buffer = Vec::with_capacity(self.buffer_height * self.buffer_width);
        for c in sim.creatures() {
            let pos = c.position();
            self.plot_pixel(&mut buffer, pos.x, pos.y, c.color());
        }

        buffer
    }

    pub fn buffer_dimensions(&self) -> (usize, usize) {
        (self.buffer_width, self.buffer_height)
    }

    fn plot_pixel(&self, buffer: &mut Buffer, x: usize, y: usize, color: Color) {
        buffer[x + (y*self.buffer_width)] = color;
    }
}

pub struct RendererBuilder {
    attr: RendererAttributes
}

impl RendererBuilder {
    pub fn new() -> Self {
        Self {
            attr: RendererAttributes::default()
        }
    }

    pub fn build(mut self) -> Result<Renderer<'static>, RendererError> {
        if self.attr.field_width == 0 || self.attr.field_height == 0 {
            return Err(RendererError::BufferTooSmall);
        }

        return Ok(Renderer::new(self.attr));
    }

    pub fn with_field_color(mut self, color: Color) -> Self {
        self.attr.field_color = color;
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.attr.border_color = color;
        self
    }

    pub fn with_field_dimensions(mut self, width: usize, height: usize) -> Self {
        self.attr.field_width = width;
        self.attr.field_height = height;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_invalid_color() {
        let color = Color::new(256, 34, -1);

        assert_eq!(color, Color::new(0, 34, 0));
    }
}
