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
use renderer::SimRenderer;

const VIEWPORT_WIDTH: u32 = 500;
const VIEWPORT_HEIGHT: u32 = 500;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window_size = PhysicalSize::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT);

    let window = Rc::new(WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("Neural Network, No Cap")
        .build(&event_loop).unwrap());

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    let renderer = SimRenderer::new(VIEWPORT_WIDTH as usize, VIEWPORT_HEIGHT as usize);

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                surface
                    .resize(
                        NonZeroU32::new(VIEWPORT_WIDTH).unwrap(),
                        NonZeroU32::new(VIEWPORT_HEIGHT).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                buffer.present().unwrap();
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
