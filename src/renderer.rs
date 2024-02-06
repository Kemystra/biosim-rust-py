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

    pub block_size: usize,
    pub field_block_width: usize,
    pub field_block_height: usize,

    pub field_color: Color,
    pub border_color: Color
}

impl RendererAttributes {
    pub fn renew_total_size(&mut self) {
        self.width = (self.block_size * self.field_block_width) + self.block_size;
        self.height = (self.block_size * self.field_block_height) + self.block_size;
    }
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

    fn init_render(&mut self, buffer: Buffer) -> Result<(), RendererError> {
        let attr = &self.attr;

        // Draw border
        self.current_color = attr.border_color;

        self.stamp_block(buffer, 0, 0, (attr.width, attr.block_size))?;
        self.stamp_block(buffer, 0, attr.height - attr.block_size, (attr.width, attr.block_size))?;

        self.stamp_block(buffer, 0, attr.block_size, (attr.block_size, attr.height - attr.block_size*2))?;
        self.stamp_block(buffer, attr.width - attr.block_size, attr.block_size, (attr.block_size, attr.height - attr.block_size*2))?;

        // Draw empty field
        self.current_color = attr.field_color;

        self.stamp_block(
            buffer,
            attr.block_size,
            attr.block_size,
            (attr.width - attr.block_size*2, attr.height - attr.block_size*2)
        )?;

        self.is_initialized = true;
        Ok(())
    }

    pub fn render(&mut self, buffer: Buffer, sim: &Simulation) -> Result<(), RendererError> {
        if !self.is_initialized {
            self.init_render(buffer)?;
        }

        let mut pos: Vector2D<usize>;
        for creature in sim.creatures() {
            pos = creature.position();
            self.current_color = creature.color();
            self.grid_stamp_block(buffer, pos.x, pos.y)?;
        }

        Ok(())
    }

    // Stamp blocks of pixels according to the field grid
    pub fn grid_stamp_block<T>(&self, buffer: Buffer, field_x: T, field_y: T) -> Result<(), RendererError>
        where T: Into<usize> {
        let field_x = field_x.into();
        let field_y = field_y.into();

        if field_x >= self.attr.field_block_width || field_y >= self.attr.field_block_height {
            return Err(RendererError::OutOfFieldRange(field_x, field_y));
        }

        self.stamp_block(
            buffer,
            field_x * self.attr.block_size,
            field_y * self.attr.block_size,
            (self.attr.block_size, self.attr.block_size)
        )?;

        Ok(())
    }

    // Raw block stamping, without caring where it will go
    // Starting from top left corner
    fn stamp_block<T>(&self, buffer: Buffer, x: T, y: T, size: (T,T)) -> Result<(), RendererError>
        where T: Into<usize> {
        // Is this code ugly? Yes
        let (stamp_width, stamp_height) = (size.0.into(), size.1.into());
        let (x, y) = (x.into(), y.into());

        for x_offset in 0..stamp_width.into() {
            for y_offset in 0..stamp_height.into() {
                self.plot_pixel(buffer, x + x_offset, y + y_offset, self.current_color)?;
            }
        }

        Ok(())
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

    pub fn build(mut self) -> Result<Renderer, RendererError> {
        self.attr.renew_total_size();
        if self.attr.width == 0 || self.attr.height == 0 {
            return Err(RendererError::BufferTooSmall);
        }

        return Ok(Renderer::new(self.attr));
    }

    pub fn with_block_size(mut self, s: usize) -> Self {
        self.attr.block_size = s;

        self
    }

    pub fn with_field_size(mut self, w: usize, h: usize) -> Self {
        self.attr.field_block_width = w;
        self.attr.field_block_height = h;

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

            block_size: 0,
            field_block_width: 0,
            field_block_height: 0,

            field_color: Color::default(),
            border_color: Color::default()
        };

        attr.block_size = 10;
        attr.block_size = 10;

        attr.field_block_width = 200;
        attr.field_block_height = 200;

        attr.renew_total_size();

        assert_eq!(attr.width, 2000);
        assert_eq!(attr.height, 2000);
    }
}
