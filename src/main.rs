use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use ant_sim::Simulation; // Ensure ant_sim is the correct crate name and Simulation is exposed.

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Ants Simulation")
        .with_inner_size(LogicalSize::new(Simulation::WIDTH as f64, Simulation::HEIGHT as f64))
        .build(&event_loop)
        .expect("Failed to create window");

    let surface_texture = SurfaceTexture::new(window.inner_size().width, window.inner_size().height, &window);
    let mut pixels = Pixels::new(Simulation::WIDTH as u32, Simulation::HEIGHT as u32, surface_texture)
        .expect("Failed to create pixel buffer");

    let mut simulation = Simulation::new();

    let updates_per_render = 5;  // Number of updates to perform before each render
    let mut update_count = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; // Ensures that your application will continue to run as frequently as possible
        match event {
            Event::WindowEvent { event, .. } => {
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                    input: winit::event::KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                }) {
                    *control_flow = ControlFlow::Exit;
                }
            },
            Event::MainEventsCleared => {
                simulation.update(); // Update simulation state
                update_count += 1;

                if update_count >= updates_per_render {
                    window.request_redraw();
                    update_count = 0; // Reset the update count after rendering
                }
            },
            Event::RedrawRequested(_) => {
                simulation.render(pixels.get_frame_mut());
                if pixels.render().is_err() {
                    eprintln!("Failed to render pixels");
                }
            },
            _ => {}
        }
    });
}
