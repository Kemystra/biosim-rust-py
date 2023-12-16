use std::default::Default;
use std::convert::From;

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

    pub field_color: Color,
    pub border_color: Color
}

impl RendererAttributes {
    pub fn renew_total_size(&mut self) {
        self.width = self.block_width * self.field_block_width;
        self.height = self.block_height * self.field_block_height;
    }
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

    pub fn render(&self, sim: &Simulation) {
        simulation
    }

    pub fn plot_pixel<T: Into<usize>>(&self, buffer: Buffer, x: T, y: T, color: Color) {
        buffer[x.into() + (y.into()*self.attr.width)] = color.rgb_u32();
    }

    pub fn bresenham_line(
        &mut self, buffer: Buffer, color: Color,
        x0: isize, y0: isize,
        end_x: isize, end_y: isize) {

        let mut curr_x = x0;
        let mut curr_y = y0;

        let dx = (end_x - curr_x).abs();
        let dy = -(end_y - curr_y).abs();
        let mut error = dx + dy;

        let sx = if curr_x < end_x {1} else {-1};
        let sy = if curr_y < end_y {1} else {-1};

        loop {
            self.plot_pixel(buffer, curr_x as usize, curr_y as usize, color);
            if curr_x == end_x && curr_y == end_y {break}
            let e2 = error * 2;

            if e2 >= dy {
                if curr_x == end_x {break}
                error += dy;
                curr_x += sx;
            }

            if e2 <= dx {
                if curr_y == end_y {break}
                error += dx;
                curr_y += sy
            }
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
