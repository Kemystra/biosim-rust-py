use std::default::Default;
use std::convert::From;
use std::cmp::PartialEq;

use thiserror::Error;

use crate::simulation::Simulation;
use crate::vector2d::Vector2D;


type Buffer<'a> = &'a mut [u32];

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Color(u32);

impl Color {
    pub fn new<T: TryInto<u8>>(r: T, g: T, b: T) -> Self {
        Self (
            r.try_into().unwrap_or_default() as u32 |
            ((g.try_into().unwrap_or_default() as u32) << 8) |
            ((b.try_into().unwrap_or_default() as u32) << 16)
        )
    }

    pub fn rgb_u32(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug)]
struct RendererAttributes {
    pub width: usize,
    pub height: usize,

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

pub struct Renderer {
    attr: RendererAttributes,
    current_color: Color,
    is_initialized: bool
}

impl Renderer {
    fn new(attr: RendererAttributes) -> Self {
        Self {
            attr,
            current_color: Color::default(),
            is_initialized: false
        }
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

    pub fn build(mut self) -> Result<Renderer, RendererError> {
        if self.attr.width == 0 || self.attr.height == 0 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_invalid_color() {
        let color = Color::new(256, 34, -1);

        assert_eq!(color, Color::new(0, 34, 0));
    }

    #[test]
    fn output_rgb_as_u32() {
        let color = Color::new(100, 234, 88);
        let expected = 100_u32 | (234 << 8) | (88 << 16);

        assert_eq!(color.rgb_u32(), expected);
    }
}
