use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::{Instant, Duration};

use winit::event::{Event, WindowEvent, KeyEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;
use winit::keyboard::{Key, NamedKey};

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

const BLOCK_SIZE: usize = 5;

const MICROSECONDS_PER_FRAME: u64 = 1_000_000 / 60;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let viewport_width = (FIELD_WIDTH * BLOCK_SIZE + BLOCK_SIZE) as u32;
    let viewport_height = (FIELD_HEIGHT * BLOCK_SIZE + BLOCK_SIZE) as u32;
    let window_size = PhysicalSize::new(viewport_width, viewport_height);

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("Neural Network, No Cap")
        .build(&event_loop)?);

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
    surface.resize(
        NonZeroU32::new(viewport_width).unwrap(),
        NonZeroU32::new(viewport_height).unwrap(),
    )
    .unwrap();

    let sim = Simulation::new(FIELD_WIDTH, FIELD_HEIGHT);

    let gray = Color::from(0x00_aa_aa_aa);
    let light_orange = Color::from(0x00_ff_ee_bf);
    let mut renderer = RendererBuilder::new()
        .with_field_size(FIELD_WIDTH, FIELD_HEIGHT)
        .with_block_size(BLOCK_SIZE)
        .with_field_color(light_orange)
        .with_border_color(gray)
        .build().unwrap();

    let mut now = Instant::now();
    let mut delta_time = Duration::new(0,0);
    let per_frame_duration = Duration::from_micros(MICROSECONDS_PER_FRAME);

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
            }

            Event::AboutToWait => {
                let mut buffer = surface.buffer_mut().unwrap();
                renderer.render(&mut buffer, &sim).unwrap();
                buffer.present().unwrap();

                delta_time = now.elapsed();
                println!("{:?}", delta_time.as_micros());
                now = Instant::now();
            }

            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, ..}, window_id }
                if window_id == window.id() => {
                    let pressed_key;
                    if let Some(key) = get_key_press(event) {
                        pressed_key = key;
                    }
                    else { return }

                    match pressed_key {
                        NamedKey::Space => {
                            println!("FPS: {:?}", 1_000_000 / delta_time.as_micros());
                        },
                        _ => ()
                    }
                }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    })?;

    Ok(())
}

fn get_key_press(key_event: KeyEvent) -> Option<NamedKey> {
    if !key_event.state.is_pressed() { return None }

    if let Key::Named(named_key) = key_event.logical_key {
        Some(named_key)
    }
    else { None }
}
