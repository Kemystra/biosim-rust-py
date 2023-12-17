use std::num::NonZeroU32;
use std::rc::Rc;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;

mod simulation;
mod creature;
mod genome;
mod neuron;
mod renderer;
mod vector2d;

use simulation::Simulation;
use renderer::RendererBuilder;

const FIELD_WIDTH: usize = 100;
const FIELD_HEIGHT: usize = 100;

const BLOCK_WIDTH: usize = 5;
const BLOCK_HEIGHT: usize = 5;


fn main() {
    let viewport_width = (FIELD_WIDTH * BLOCK_WIDTH) as u32;
    let viewport_height = (FIELD_HEIGHT * BLOCK_HEIGHT) as u32;
    let window_size = PhysicalSize::new(viewport_width, viewport_height);

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("Neural Network, No Cap")
        .build(&event_loop).unwrap());

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let sim = Simulation::new(FIELD_WIDTH, FIELD_HEIGHT);
    let mut renderer = RendererBuilder::new().build().unwrap();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                surface
                    .resize(
                        NonZeroU32::new(viewport_width).unwrap(),
                        NonZeroU32::new(viewport_height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                renderer.render(&mut buffer, &sim).unwrap();
                buffer.present().unwrap();
                println!("{:?}", elwt.control_flow());
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    }).unwrap();
}
