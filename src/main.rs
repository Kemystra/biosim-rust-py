use std::num::NonZeroU32;

use winit::window::WindowBuilder;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use softbuffer::{Context, Surface};

struct Renderer {
    width: usize,
    height: usize,
    tmp_buffer: Vec<u32>
}

impl Renderer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tmp_buffer: vec![0; width*height]
        }
    }

    fn plot_pixel(&mut self, x: usize, y: usize, color: u32) {
        self.tmp_buffer[x + (y*self.width)] = color;
    }

    fn render(&mut self) -> &Vec<u32> {
        let color = 255 | (255 << 8) | (255 << 16);

        for y in 0..400 {
            for x in 0..400 {
                self.plot_pixel(x,y,color);
            }
        }

        &self.tmp_buffer
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    
    let context = unsafe { Context::new(&window) }.unwrap();
    let mut surface = unsafe { Surface::new(&context, &window) }.unwrap();

    let (width, height) = { let size = window.inner_size(); (size.width, size.height) };
    let mut renderer = Renderer::new(width as usize, height as usize);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::CloseRequested }
                if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            // Exit window with ANY keypress
            Event::WindowEvent { window_id, event: WindowEvent::KeyboardInput {..} }
                if window_id == window.id() => { *control_flow = ControlFlow::Exit }

            Event::RedrawRequested(window_id)
                if window_id == window.id() => {
                    let (width, height) = { let size = window.inner_size(); (size.width, size.height) };
                    surface.resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap()
                    ).unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();

                    buffer.copy_from_slice(renderer.render());

                    buffer.present().unwrap();
                }

            _ => {}
        }
    });
}
