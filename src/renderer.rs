use std::{default::Default, error::Error};
use std::convert::From;
use std::cmp::PartialEq;

use thiserror::Error;

use crate::simulation::Simulation;


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

#[derive(Default)]
struct RendererAttributes {
    pub width: usize,
    pub height: usize,

    pub block_width: usize,
    pub block_height: usize,
    pub field_block_width: usize,
    pub field_block_height: usize,
    pub border_width: usize,

    pub field_color: Color,
    pub border_color: Color
}

impl RendererAttributes {
    pub fn renew_total_size(&mut self) {
        self.width = (self.block_width * self.field_block_width) + self.border_width;
        self.height = (self.block_height * self.field_block_height) + self.border_width;
    }
}

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Trying to access a pixel out of Buffer range ({0}, {1})")]
    OutOfBufferRange(usize, usize),
}

pub struct Renderer {
    attr: RendererAttributes
}

impl Renderer {
    fn new(attr: RendererAttributes) -> Self {
        Self {
            attr
        }
    }

    pub fn render(&self, buffer: Buffer, sim: &Simulation) {
        let 
        for i in 0..4 {

        }
    }

    pub fn plot_pixel<T>(&self, buffer: Buffer, x: T, y: T, color: Color) -> Result<(), RendererError>
        where T: Into<usize> {

        let (x, y) = (x.into(), y. into());
        let parsed_index = x + (y*self.attr.width);

        let slice_index = buffer
            .get_mut(parsed_index)
            .ok_or(RendererError::OutOfBufferRange(x, y))?;

        *slice_index = color.rgb_u32();

        Ok(())
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

    pub fn build(mut self) -> Renderer {
        self.attr.renew_total_size();
        Renderer::new(self.attr)
    }

    pub fn with_block_size(mut self, w: usize, h: usize) -> Self {
        self.attr.block_width = w;
        self.attr.block_height = h;

        self
    }

    pub fn with_field_size(mut self, w: usize, h: usize) -> Self {
        self.attr.field_block_width = w;
        self.attr.field_block_height = h;

        self
    }

    pub fn with_border_width(mut self, w: usize) -> Self {
        self.attr.border_width = w;
        self
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

    #[test]
    fn recalculate_total_screensize() {
        let mut attr = RendererAttributes {
            width: 0,
            height: 0,

            block_width: 0,
            block_height: 0,
            field_block_width: 0,
            field_block_height: 0,
            border_width: 0,

            field_color: Color::default(),
            border_color: Color::default()
        };

        attr.block_width = 10;
        attr.block_height = 10;

        attr.field_block_width = 200;
        attr.field_block_height = 200;

        attr.renew_total_size();

        assert_eq!(attr.width, 2000);
        assert_eq!(attr.height, 2000);
    }
}
