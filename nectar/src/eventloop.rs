use anyhow::Result;
use winit::{event::*, keyboard::*, window::WindowBuilder, event_loop::EventLoop};
use crate::window::State;
use crate::events::*;
use crate::world::World;

pub async fn run(event_loop: EventLoop<()>, world: &World) -> Result<usize>{
    env_logger::init();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await; // Inits Window!

    event_loop.run(move |event, window_target| { // Starts Eventloop
        world.event_handler.call_events(EventType::Update);

        match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => if !state.input(event) { // UPDATED!
                match event {
                    /*WindowEvent::KeyboardInput {
                        event: ref keyboard_input,
                        ..
                    } => {},*/
                    WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                                ..
                        } => window_target.exit(),
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    },
                    WindowEvent::RedrawRequested => {
                        state.update();
                        match state.render() {
                            Ok(_) => {},
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => window_target.exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    });
    Ok(0)
}
